use std::collections::HashMap;

use cidr_utils::cidr::Ipv4Cidr;
use etherparse::Ipv4HeaderSlice;
use postgres::Client;
// use rusqlite::{params, Connection};
// use std::{convert::TryFrom, net::Ipv4Addr, str::FromStr};

pub struct PcapIP {
  pub id: i32,
  pub vitima_addr: String,
  pub dest_addr: String,
  pub vitima_cidr: Ipv4Cidr,
}

impl PcapIP {
  pub fn insert(
    &self,
    conn: &mut Client,
    tb_ip_id: &mut i32,
    hm_ip_id: &mut HashMap<String, i32>,
    vitima_cidr_id: &i32,
  ) {
    let vitima_addr_id = match hm_ip_id.get(&self.vitima_addr) {
      Some(id_cidr) => id_cidr,
      None => {
        *tb_ip_id += 1;

        let vitima_addr_str = self.vitima_addr.to_string();
        match conn.execute(
          "INSERT INTO TBIP (id, ip) values ($1, $2)",
          &[tb_ip_id, &vitima_addr_str],
        ) {
          Ok(_) => {
            // println!("attack inserted");
          }
          Err(err) => {
            println!("Problem inserting TBIP: {:?}", err)
          }
        }
        hm_ip_id.insert(vitima_addr_str, *tb_ip_id);
        tb_ip_id
      }
    };

    match conn.execute(
      "INSERT INTO PCAP_IP (id, vitima_addr_id, vitima_cidr_id) values ($1, $2, $3)",
      &[&self.id, vitima_addr_id, vitima_cidr_id],
    ) {
      Ok(_) => {
        // println!("PcapIP inserted")
      }
      Err(err) => {
        println!("Problem inserting ip: {:?}", err)
      }
    }
  }
}

pub fn process_ip(
  ipv4_header: Ipv4HeaderSlice,
  id: i32,
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
