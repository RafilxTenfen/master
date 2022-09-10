use ntp_parser::NtpPacket;

pub struct PcapNTP {
  pub id: u32,
  pub refid: u32,
}

pub fn process_ntp(ntp_packet: &NtpPacket, id: u32) -> PcapNTP {
  match ntp_packet {
    ntp_parser::NtpPacket::V3(ntp_v3) => {
      return PcapNTP {
        id,
        refid: ntp_v3.ref_id,
      };
    }
    ntp_parser::NtpPacket::V4(ntp_v4) => {
      return PcapNTP {
        id,
        refid: ntp_v4.ref_id,
      };
    }
  }
}
