use etherparse::UdpHeaderSlice;

pub struct PcapFrame {
  pub id: u32,
  pub destination_port: u16,
}

pub fn process_frame(frame: &EthernetFrame, id: u32) -> PcapFrame {
  return PcapFrame {
    id,
    destination_port: udp_header.destination_port(),
  };
}
