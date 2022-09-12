use cidr_utils::cidr::Ipv4Cidr;
use etherparse::Ipv4HeaderSlice;
use rusqlite::{params, Connection};
// use std::{convert::TryFrom, net::Ipv4Addr, str::FromStr};

pub struct PcapIP {
  pub id: u32,
  pub vitima_addr: String,
  pub vitima_cidr: Ipv4Cidr,
}

impl PcapIP {
  pub fn insert(&self, conn: &Connection) {
    match conn.execute(
      "INSERT INTO PCAP_IP (id, vitima_addr, vitima_cidr) values (?1, ?2, ?3)",
      params![&self.id, &self.vitima_addr, &self.vitima_cidr.to_string()],
    ) {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting ip: {:?}", err)
      }
    }
  }
}

pub fn process_ip(ipv4_header: Ipv4HeaderSlice, id: u32) -> PcapIP {
  let vitima_addr = ipv4_header.source_addr().to_string();
  let vitima_cidr = match Ipv4Cidr::from_str(vitima_addr.as_str()) {
    // adding CIDR takes 10sec +
    Ok(cidr) => cidr,
    Err(err) => {
      println!("Problem Ipv4Cidr ip: {:?}, - err {:?}", vitima_addr, err);
      Ipv4Cidr::from_str("192.168.51.0/24").unwrap()
    }
  };

  return PcapIP {
    id,
    vitima_addr,
    vitima_cidr,
  };
}
