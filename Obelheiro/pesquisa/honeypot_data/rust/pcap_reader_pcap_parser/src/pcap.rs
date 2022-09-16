use cidr_utils::cidr::Ipv4Cidr;
use pcap_parser::{traits::PcapReaderIterator, LegacyPcapReader, PcapError};
use rusqlite::Connection;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
mod attack;
mod block;

// HashMap CIDR => UDP dest port => Attack
pub fn new_hm_cidr_udp_attack() -> HashMap<Ipv4Cidr, HashMap<u16, attack::PcapAttack>> {
  return HashMap::<Ipv4Cidr, HashMap<u16, attack::PcapAttack>>::new();
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
  conn: &mut Connection,
  hm_cidr_udp_attack: &mut HashMap<Ipv4Cidr, HashMap<u16, attack::PcapAttack>>,
  hm_id: &mut HashMap<&str, u32>,
  hm_ip_cidr: &mut HashMap<String, Ipv4Cidr>,
) {
  println!("pcap_process_dir {}", dir.display());

  let pcaps = get_pcaps_ordered(dir);
  pcaps
    .iter()
    .for_each(|pcap| pcap_process(pcap, conn, hm_cidr_udp_attack, hm_id, hm_ip_cidr));

  let tx_conn = match conn.unchecked_transaction() {
    Ok(tx) => tx,
    Err(err) => {
      println!("Error creating tx {}", err);
      return;
    }
  };

  // check again all the attacks with > 5 packets that were not inserted
  pcap_process_end(&tx_conn, hm_cidr_udp_attack);

  match tx_conn.commit() {
    Ok(_) => {
      // println!("Sending commit");
    }
    Err(err) => {
      println!("Error openning file {}", err);
    }
  };
}

pub fn pcap_process(
  pcap: &PathBuf,
  conn: &mut Connection,
  hm_cidr_udp_attack: &mut HashMap<Ipv4Cidr, HashMap<u16, attack::PcapAttack>>,
  hm_id: &mut HashMap<&str, u32>,
  hm_ip_cidr: &mut HashMap<String, Ipv4Cidr>,
) {
  if !pcap.is_file() {
    println!("pcap {} is not a file", pcap.display());
    return;
  }

  println!("processing pcap {}", pcap.display());

  let tx_conn = match conn.unchecked_transaction() {
    Ok(tx) => tx,
    Err(err) => {
      println!("Error creating tx {}", err);
      return;
    }
  };

  match File::open(pcap) {
    Ok(file) => {
      // 65536 recommended by the lib
      match LegacyPcapReader::new(65536, BufReader::new(file)) {
        Ok(ref mut reader) => {
          loop {
            match reader.next() {
              Ok((offset, ref block)) => {
                match block::process_block(block, hm_id, hm_ip_cidr) {
                  Some(new_packet) => {
                    attack::process_new_packet(&tx_conn, hm_cidr_udp_attack, hm_id, new_packet);
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
    }
    Err(err) => {
      println!("Error openning file {}", err)
    }
  }

  // println!("Reached commit");
  match tx_conn.commit() {
    Ok(_) => {
      // println!("Sending commit");
    }
    Err(err) => {
      println!("Error openning file {}", err);
    }
  };
}

// inserts all the attacks with packets.len > 5 that weren't inserted in the
// add new packet loop
pub fn pcap_process_end(
  conn: &Connection,
  hm_cidr_udp_attack: &mut HashMap<Ipv4Cidr, HashMap<u16, attack::PcapAttack>>,
) {
  for (_, udp_attack) in hm_cidr_udp_attack {
    for (_, attack) in udp_attack {
      if attack.packets.len() > 5 {
        attack.insert(conn);
      }
    }
  }
}
