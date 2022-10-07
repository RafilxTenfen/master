use etherparse::UdpHeaderSlice;
use postgres::Transaction;
// use rusqlite::{params, Connection};

pub struct PcapUDP {
  pub id: i64,
  pub destination_port: i16,
}

impl PcapUDP {
  pub fn insert(&self, conn: &mut Transaction) {
    match conn.execute(
      "INSERT INTO PCAP_UDP (id, destination_port) values ($1, $2)",
      &[&self.id, &self.destination_port],
    ) {
      Ok(_) => {
        // println!("PcapUDP inserted")
      }
      Err(err) => {
        println!("Problem inserting udp: {:?}", err)
      }
    }
  }
}

pub fn process_udp(udp_header: &UdpHeaderSlice, id: i64) -> PcapUDP {
  return PcapUDP {
    id,
    destination_port: udp_header.destination_port() as i16,
  };
}
