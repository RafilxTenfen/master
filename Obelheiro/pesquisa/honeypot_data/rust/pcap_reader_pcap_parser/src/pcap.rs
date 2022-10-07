use bzip2::read::BzDecoder;
use chrono::{Duration, NaiveDateTime};
use cidr_utils::cidr::Ipv4Cidr;
use pcap_parser::{traits::PcapReaderIterator, LegacyPcapReader, PcapError};
use postgres::{Client, Transaction};
// use rusqlite::Connection;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

mod attack;
mod block;

// HashMap CIDR => UDP dest port => Attack
pub fn new_hm_cidr_udp_attack() -> HashMap<Ipv4Cidr, HashMap<i16, attack::PcapAttack>> {
  return HashMap::<Ipv4Cidr, HashMap<i16, attack::PcapAttack>>::new();
}

fn list_files(pathname: &PathBuf, filter: &str) -> Option<Vec<PathBuf>> {
  Some(
    fs::read_dir(pathname)
      .ok()?
      .filter_map(|entry| {
        Some(
          pathname.join(
            entry
              .ok()?
              .path()
              .strip_prefix(pathname)
              .ok()?
              .to_path_buf(),
          ),
        )
      })
      .filter(|path| path.ends_with(filter))
      .collect(),
  )
}

pub fn get_pcaps_ordered(dir: &PathBuf) -> Vec<PathBuf> {
  let pcaps = vec![];

  if !dir.is_dir() {
    return pcaps;
  }

  match list_files(dir, "") {
    Some(mut files) => {
      files.sort();
      return files;
    }
    None => {}
  }

  return pcaps;
}

pub fn pcap_process_dir(
  dir: &PathBuf,
  conn: &mut Client,
  hm_cidr_udp_attack: &mut HashMap<Ipv4Cidr, HashMap<i16, attack::PcapAttack>>,
  hm_id: &mut HashMap<&str, i64>,
  hm_ip_cidr: &mut HashMap<String, Ipv4Cidr>,
) {
  println!("pcap_process_dir {}", dir.display());

  // todo normalize path, remove .. ../ https://docs.rs/normpath/latest/normpath/
  let pcaps = get_pcaps_ordered(dir);
  pcaps
    .iter()
    .for_each(|pcap| pcap_process(pcap, conn, hm_cidr_udp_attack, hm_id, hm_ip_cidr));
  // .for_each(|pcap| {
  //   println!("processing file {}", pcap.display())
  // });

  let mut tx_conn = match conn.build_transaction().start() {
    Ok(tx) => tx,
    Err(err) => {
      println!("Error creating tx {}", err);
      return;
    }
  };

  // check again all the attacks with > 5 packets that were not inserted
  pcap_process_end(&mut tx_conn, hm_cidr_udp_attack);

  match tx_conn.commit() {
    Ok(_) => {
      // println!("Sending commit");
    }
    Err(err) => {
      println!("Error commiting pcap_process_end {}", err);
    }
  };
}

pub fn pcap_process(
  pcap_bz2_path: &PathBuf,
  conn: &mut Client,
  hm_cidr_udp_attack: &mut HashMap<Ipv4Cidr, HashMap<i16, attack::PcapAttack>>,
  hm_id: &mut HashMap<&str, i64>,
  hm_ip_cidr: &mut HashMap<String, Ipv4Cidr>,
) {
  // if !pcap.is_file() {
  //   println!("pcap {} is not a file", pcap.display());
  //   return;
  // }

  println!("processing pcap {}", pcap_bz2_path.display());

  let mut tx_conn = match conn.build_transaction().start() {
    Ok(tx) => tx,
    Err(err) => {
      println!("Error creating tx {}", err);
      return;
    }
  };

  let pcap_bz2_reader = match File::open(pcap_bz2_path) {
    Ok(pcap_bz2) => BzDecoder::new(pcap_bz2),
    Err(err) => {
      println!(
        "Error oppening pcap_bz2_path {} {}",
        pcap_bz2_path.display(),
        err
      );
      return;
    }
  };

  let mut last_packet_timestamp: NaiveDateTime = NaiveDateTime::default();
  // 65536 recommended by the lib
  match LegacyPcapReader::new(65536, pcap_bz2_reader) {
    Ok(ref mut reader) => {
      loop {
        match reader.next() {
          Ok((offset, ref block)) => {
            match block::process_block(block, hm_id, hm_ip_cidr) {
              Some(new_packet) => {
                last_packet_timestamp = new_packet.timestamp;
                attack::process_new_packet(&mut tx_conn, hm_cidr_udp_attack, hm_id, new_packet);
              }
              None => {}
            }
            reader.consume(offset) // !important, otherwise it will not read the next
          }
          Err(PcapError::Eof) => break,
          Err(PcapError::Incomplete) => {
            reader.refill().unwrap();
          }
          Err(e) => panic!("error while reading: {:?}", e),
        }
      }
    }
    Err(err) => {
      println!("Error reading LegacyPcapReader {}", err)
    }
  };

  pcap_process_clear_old_attacks(&mut tx_conn, last_packet_timestamp, hm_cidr_udp_attack);

  // println!("Going to commit operations");
  match tx_conn.commit() {
    Ok(_) => {
      // println!("commit ok");
    }
    Err(err) => {
      println!("Error openning file {}", err);
    }
  };
}

// checks if it can delete old attacks
pub fn pcap_process_clear_old_attacks(
  conn: &mut Transaction,
  last_packet_timestamp: NaiveDateTime,
  hm_cidr_udp_attack: &mut HashMap<Ipv4Cidr, HashMap<i16, attack::PcapAttack>>,
) {
  let mut cidr_udp_attacks_to_delete = HashMap::<Ipv4Cidr, i16>::new();

  for (cidr, udp_attack) in &*hm_cidr_udp_attack {
    for (attack_udp_dest_port, attack) in udp_attack {
      match attack
        .timestamp_fim
        .checked_add_signed(Duration::minutes(1))
      {
        Some(fim_plus_1min) => {
          if last_packet_timestamp <= fim_plus_1min {
            continue;
          }
          // means that it is past attack.timestamp_fim + 1min from last packet
          // then we can insert if any old attack with +5 packets and remove
          // it from the map to clear out memory
          if attack.packets.len() > 5 {
            attack.insert(conn);
          }

          // insert to be deleted afterwards
          cidr_udp_attacks_to_delete.insert(*cidr, *attack_udp_dest_port);
        }
        None => continue,
      }
    }
  }

  // at the end we can safely remove old attack entries
  for (cidr, udp_port_to_delete) in cidr_udp_attacks_to_delete {
    match hm_cidr_udp_attack.get_mut(&cidr) {
      Some(udp_attack) => {
        if let Entry::Occupied(o) = udp_attack.entry(udp_port_to_delete) {
          o.remove();
        }

        // now we check if the second hashmap is empty for that cidr entry
        if !udp_attack.is_empty() {
          continue;
        }

        // if it is we can delete it
        if let Entry::Occupied(o) = hm_cidr_udp_attack.entry(cidr) {
          o.remove();
        }
      }
      None => {}
    }
  }
}

// inserts all the attacks with packets.len > 5 that weren't inserted in the
// add new packet loop
pub fn pcap_process_end(
  conn: &mut Transaction,
  hm_cidr_udp_attack: &mut HashMap<Ipv4Cidr, HashMap<i16, attack::PcapAttack>>,
) {
  for (_, udp_attack) in hm_cidr_udp_attack {
    for (_, attack) in udp_attack {
      if attack.packets.len() > 5 {
        attack.insert(conn);
      }
    }
  }
}
