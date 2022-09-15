use cidr_utils::cidr::Ipv4Cidr;
use pcap_parser::{traits::PcapReaderIterator, LegacyPcapReader, PcapError};
use rusqlite::{Connection, Statement};
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
  conn: &Connection,
  stmt_pcap_attack: &mut Statement,
  stmt_pcap_attack_packet: &mut Statement,
  hm_cidr_udp_attack: &mut HashMap<Ipv4Cidr, HashMap<u16, attack::PcapAttack>>,
  hm_id: &mut HashMap<&str, u32>,
) {
  println!("pcap_process_dir {}", dir.display());

  let pcaps = get_pcaps_ordered(dir);
  pcaps.iter().for_each(|pcap| {
    pcap_process(
      pcap,
      conn,
      stmt_pcap_attack,
      stmt_pcap_attack_packet,
      hm_cidr_udp_attack,
      hm_id,
    )
  });
}

pub fn pcap_process(
  pcap: &PathBuf,
  conn: &Connection,
  stmt_pcap_attack: &mut Statement,
  stmt_pcap_attack_packet: &mut Statement,
  hm_cidr_udp_attack: &mut HashMap<Ipv4Cidr, HashMap<u16, attack::PcapAttack>>,
  hm_id: &mut HashMap<&str, u32>,
) {
  if !pcap.is_file() {
    println!("pcap {} is not a file", pcap.display());
    return;
  }

  println!("processing pcap {}", pcap.display());

  match File::open(pcap) {
    Ok(file) => {
      // 65536 recommended by the lib
      match LegacyPcapReader::new(65536, BufReader::new(file)) {
        Ok(ref mut reader) => {
          loop {
            match reader.next() {
              Ok((offset, ref block)) => {
                match block::process_block(block, hm_id) {
                  Some(new_packet) => {
                    attack::process_new_packet(
                      conn,
                      stmt_pcap_attack,
                      stmt_pcap_attack_packet,
                      hm_cidr_udp_attack,
                      hm_id,
                      new_packet,
                    );
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
}
