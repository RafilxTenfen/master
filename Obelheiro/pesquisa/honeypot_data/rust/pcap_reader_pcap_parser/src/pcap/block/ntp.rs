use ntp_parser::NtpPacket;
use postgres::Client;
// use rusqlite::{params, Connection};

pub struct PcapNTP {
  pub id: i32,
  pub refid: i32,
}

impl PcapNTP {
  pub fn insert(&self, conn: &mut Client) {
    match conn.execute(
      "INSERT INTO PCAP_NTP (id, refid) values ($1, $2)",
      &[&self.id, &self.refid],
    ) {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting ntp: {:?}", err)
      }
    }
  }
}

pub fn process_ntp(ntp_packet: &NtpPacket, id: i32) -> PcapNTP {
  match ntp_packet {
    ntp_parser::NtpPacket::V3(ntp_v3) => {
      return PcapNTP {
        id,
        // TODO: verify monlist check bytes
        refid: ntp_v3.ref_id as i32,
      };
    }
    ntp_parser::NtpPacket::V4(ntp_v4) => {
      return PcapNTP {
        id,
        refid: ntp_v4.ref_id as i32,
      };
    }
  }
}
