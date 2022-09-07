use etherparse::UdpHeaderSlice;

pub struct PcapUDP {
  pub id: u32,
  pub destination_port: u16,
}

pub fn process_udp(udp_header: &UdpHeaderSlice, id: u32) -> PcapUDP {
  return PcapUDP {
    id,
    destination_port: udp_header.destination_port(),
  };
}
