use etherparse::UdpHeaderSlice;
use rusqlite::Statement;

pub struct PcapUDP {
  pub id: u32,
  pub destination_port: u16,
}

impl PcapUDP {
  pub fn insert(&self, stmt_pcap_udp: &mut Statement) {
    match stmt_pcap_udp.execute((&self.id, &self.destination_port)) {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting udp: {:?}", err)
      }
    }
  }
}

pub fn process_udp(udp_header: &UdpHeaderSlice, id: u32) -> PcapUDP {
  return PcapUDP {
    id,
    destination_port: udp_header.destination_port(),
  };
}
