use rusqlite::{params, Connection};

pub struct PcapDNS {
  pub id: u32,
  pub tx_id: u16,
  pub qname: String,
  pub qtype: String,
}

impl PcapDNS {
  pub fn insert(&self, conn: &Connection) {
    match conn.execute(
      "INSERT INTO PCAP_DNS (id, tx_id, qname, qtype) values (?1, ?2, ?3, ?4)",
      params![&self.id, &self.tx_id, &self.qname, &self.qtype],
    ) {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting dns: {:?}", err)
      }
    }
  }
}

pub fn process_dns(dns_packet: &dns_parser::Packet, id: u32) -> PcapDNS {
  let mut qname = String::from("qname");
  let mut qtype = String::from("-");

  for question in &dns_packet.questions {
    qname = question.qname.to_string();

    match question.qtype {
      dns_parser::QueryType::A => {
        qtype = String::from("A");
      }
      dns_parser::QueryType::NS => {
        qtype = String::from("NS");
      }
      dns_parser::QueryType::MF => {
        qtype = String::from("MF");
      }
      dns_parser::QueryType::CNAME => {
        qtype = String::from("CNAME");
      }
      dns_parser::QueryType::SOA => {
        qtype = String::from("SOA");
      }
      dns_parser::QueryType::MB => {
        qtype = String::from("MB");
      }
      dns_parser::QueryType::MG => {
        qtype = String::from("MG");
      }
      dns_parser::QueryType::MR => {
        qtype = String::from("MR");
      }
      dns_parser::QueryType::NULL => {
        qtype = String::from("NULL");
      }
      dns_parser::QueryType::WKS => {
        qtype = String::from("WKS");
      }
      dns_parser::QueryType::PTR => {
        qtype = String::from("PTR");
      }
      dns_parser::QueryType::HINFO => {
        qtype = String::from("HINFO");
      }
      dns_parser::QueryType::MINFO => {
        qtype = String::from("MINFO");
      }
      dns_parser::QueryType::MX => {
        qtype = String::from("MX");
      }
      dns_parser::QueryType::TXT => {
        qtype = String::from("TXT");
      }
      dns_parser::QueryType::AAAA => {
        qtype = String::from("AAAA");
      }
      dns_parser::QueryType::SRV => {
        qtype = String::from("SRV");
      }
      dns_parser::QueryType::AXFR => {
        qtype = String::from("AXFR");
      }
      dns_parser::QueryType::MAILB => {
        qtype = String::from("MAILB");
      }
      dns_parser::QueryType::MAILA => {
        qtype = String::from("MAILA");
      }
      dns_parser::QueryType::All => {
        qtype = String::from("All");
      }
    }

    // we just want the first qname/qtype
    break;
  }

  return PcapDNS {
    id,
    tx_id: dns_packet.header.id,
    qname,
    qtype,
  };
}
