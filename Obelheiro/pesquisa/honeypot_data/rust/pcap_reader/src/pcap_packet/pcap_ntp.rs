use rtshark::{Layer, Metadata};
use rusqlite::{params, Connection};

// https://www.wireshark.org/docs/dfref/n/ntp.html

pub struct PcapNTP {
  pub id: i32,
  pub refid: String,
}

pub fn drop_table(conn: &Connection) {
  match conn.execute("DROP TABLE IF EXISTS PCAP_NTP", []) {
    Ok(_) => {
      println!("Table created!")
    }
    Err(err) => {
      println!("Problem droping table: {:?}", err)
    }
  }
}

pub fn create_table(conn: &Connection) {
  match conn.execute(
    "CREATE TABLE IF NOT EXISTS PCAP_NTP (
      id INTEGER NOT NULL,
      refid TEXT NOT NULL
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

impl PcapNTP {
  pub fn default(id: i32) -> PcapNTP {
    PcapNTP {
      id,
      refid: String::from(""),
    }
  }

  pub fn insert(&self, conn: &Connection) {
    match conn.execute(
      "INSERT INTO PCAP_NTP (id, refid) values (?1, ?2)",
      params![&self.id, &self.refid],
    ) {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting ntp: {:?}", err)
      }
    }
  }

  pub fn metadata_process(&mut self, metadata: &Metadata) {
    let (name, value) = (metadata.name(), metadata.value());
    match name {
      "ntp.refid" => {
        self.refid = value.to_string();
      }
      _ => {} // _ => println!("ignored field: {} = {} - {}", name, value, display),
    }
  }
}

pub fn pcap_process_layer_ntp(layer: &Layer, id: &i32) -> PcapNTP {
  let mut pcap_ntp = PcapNTP::default(*id);

  // if layer.name() != "ntp" {
  //   return pcap_ntp;
  // }

  println!("Processing ntp");
  layer
    .iter()
    .for_each(|metadata| pcap_ntp.metadata_process(metadata));

  return pcap_ntp;
}
