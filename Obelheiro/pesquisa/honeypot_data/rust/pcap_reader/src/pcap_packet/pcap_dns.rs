use rtshark::{Layer, Metadata};
use rusqlite::{params, Connection};

// https://www.wireshark.org/docs/dfref/d/dns.html

// A struct with two fields
pub struct PcapDNS {
  pub id: i32,
  pub tx_id: i32,
  pub qname: String,
  pub qtype: i32,
  pub qtype_text: String,
  pub udp_payload_size: i32,
}

pub fn create_table(conn: &Connection) {
  match conn.execute(
    "CREATE TABLE IF NOT EXISTS PCAP_DNS (
      id INTEGER NOT NULL,
      tx_id INTEGER NOT NULL,
      qname TEXT NOT NULL,
      qtype INTEGER NOT NULL,
      qtype_text TEXT NOT NULL,
      udp_payload_size INTEGER NOT NULL
     )",
    [],
  ) {
    Ok(_) => {
      println!("Table created!")
    }
    Err(err) => {
      println!("Problem creating table: {:?}", err)
    }
  }
}

pub fn drop_table(conn: &Connection) {
  match conn.execute("DROP TABLE IF EXISTS PCAP_DNS", []) {
    Ok(_) => {
      println!("Table created!")
    }
    Err(err) => {
      println!("Problem droping table: {:?}", err)
    }
  }
}

impl PcapDNS {
  pub fn default(id: i32) -> PcapDNS {
    PcapDNS {
      id,
      tx_id: 0,
      qname: String::from(""),
      qtype: 0,
      qtype_text: String::from(""),
      udp_payload_size: 0,
    }
  }

  pub fn insert(&self, conn: &Connection) {
    match conn.execute(
      "INSERT INTO PCAP_DNS (id, tx_id, qname, qtype, qtype_text, udp_payload_size) values (?1, ?2, ?3, ?4, ?5, ?6)",
    params![&self.id, &self.tx_id, &self.qname, &self.qtype, &self.qtype_text, &self.udp_payload_size],
  ) {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting dns: {:?}", err)
      }
    }
  }

  pub fn metadata_process(&mut self, metadata: &Metadata) {
    let (name, value, display) = (metadata.name(), metadata.value(), metadata.display());
    match name {
      "dns.id" => {
        let without_prefix = value.trim_start_matches("0x");
        match i32::from_str_radix(without_prefix, 16) {
          Ok(tx_id) => {
            self.tx_id = tx_id;
          }
          Err(err) => {
            println!("Problem parse dns.id: {:?}", err)
          }
        }
      }
      "dns.qry.name" => self.qname = value.to_string(),
      "dns.qry.type" => {
        match value.parse::<i32>() {
          Ok(qtype) => {
            self.qtype = qtype;
          }
          Err(err) => {
            println!("Problem parse qry.type: {:?}", err)
          }
        }
        self.qtype_text = display.to_string();
      }
      "dns.rr.udp_payload_size" => match value.parse::<i32>() {
        Ok(udp_payload_size) => {
          self.udp_payload_size = udp_payload_size;
        }
        Err(err) => {
          println!("Problem parse udp_payload_size: {:?}", err)
        }
      },
      _ => {} // _ => println!("ignored field: {} = {} - {}", name, value, display),
    }
  }
}

pub fn pcap_process_layer_dns(layer: &Layer, id: i32) -> PcapDNS {
  let mut pcap_dns = PcapDNS::default(id);

  println!("Processing dns");
  layer
    .iter()
    .for_each(|metadata| pcap_dns.metadata_process(metadata));

  return pcap_dns;
}
