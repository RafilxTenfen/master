use rtshark::Layer;
use rusqlite::{Connection, params};

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
  let result = conn.execute(
    "CREATE TABLE IF NOT EXISTS PCAP_DNS (
      id INTEGER NOT NULL,
      tx_id INTEGER NOT NULL,
      qname TEXT NOT NULL,
      qtype INTEGER NOT NULL,
      qtype_text TEXT NOT NULL,
      udp_payload_size INTEGER NOT NULL
     )",
    [],
  );

  match result {
    Ok(_) => {
      println!("Table created!")
    }
    Err(err) => {
      println!("Problem creating table: {:?}", err)
    }
  }
}

pub fn drop_table(conn: &Connection) {
  let result = conn.execute(
    "DROP TABLE IF EXISTS PCAP_DNS",
    [],
  );

  match result {
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
    let result = conn.execute(
        "INSERT INTO PCAP_DNS (id, tx_id, qname, qtype, qtype_text, udp_payload_size) values (?1, ?2, ?3, ?4, ?5, ?6)",
      params![&self.id, &self.tx_id, &self.qname, &self.qtype, &self.qtype_text, &self.udp_payload_size],
    );

    match result {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting dns: {:?}", err)
      }
    }
  }
}

pub fn pcap_process_layer_dns(layer: Layer, conn: &Connection, id: i32) -> PcapDNS {
  let mut pcap_dns = PcapDNS::default(id);

  if layer.name() != "dns" {
    return pcap_dns;
  }

  println!("Processing dns");
  for metadata in layer {
    let (name, value, display) = (metadata.name(), metadata.value(), metadata.display());
    match name {
      "dns.id" => {
        let without_prefix = value.trim_start_matches("0x");
        let tx_id = i32::from_str_radix(without_prefix, 16).unwrap();
        pcap_dns.tx_id = tx_id;
      }
      "dns.qry.name" => pcap_dns.qname = value.to_string(),
      "dns.qry.type" => {
        pcap_dns.qtype = value.parse::<i32>().unwrap();
        pcap_dns.qtype_text = display.to_string();
      }
      "dns.rr.udp_payload_size" => {
        pcap_dns.udp_payload_size = value.parse::<i32>().unwrap();
      }
      _ => {},
      // _ => println!("ignored field: {} = {} - {}", name, value, display),
    }
  }
  pcap_dns.insert(conn);
  return pcap_dns;
}
