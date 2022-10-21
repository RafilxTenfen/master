use std::collections::HashMap;

use cidr_utils::cidr::Ipv4Cidr;
use etherparse::Ipv4HeaderSlice;
// use tokio_postgres::Client;
// use rusqlite::{params, Connection};
// use std::{convert::TryFrom, net::Ipv4Addr, str::FromStr};

pub struct PcapIP {
  pub id: u32,
  pub vitima_addr: String,
  pub dest_addr: String,
  pub vitima_cidr: Ipv4Cidr,
}

pub fn process_ip(
  ipv4_header: Ipv4HeaderSlice,
  id: u32,
  hm_ip_cidr: &mut HashMap<String, Ipv4Cidr>,
) -> PcapIP {
  let vitima_addr = ipv4_header.source_addr().to_string();
  let (vitima_cidr, vitima) = process_ip_cidr(vitima_addr, hm_ip_cidr);

  return PcapIP {
    id,
    vitima_addr: vitima,
    dest_addr: ipv4_header.destination_addr().to_string(),
    vitima_cidr,
  };
}

pub fn process_ip_cidr(
  vitima_addr: String,
  hm_ip_cidr: &mut HashMap<String, Ipv4Cidr>,
) -> (Ipv4Cidr, String) {
  match hm_ip_cidr.get(&vitima_addr) {
    Some(cidr) => (*cidr, vitima_addr),
    None => {
      let cidr = match Ipv4Cidr::from_str(vitima_addr.to_string()) {
        Ok(cidr) => cidr,
        Err(err) => {
          println!("Err parsing cidr {}", err);
          Ipv4Cidr::from_str("192.168.51.0/24").unwrap()
        }
      };
      hm_ip_cidr.insert(vitima_addr.to_string(), cidr);
      (cidr, vitima_addr)
    }
  }
}
