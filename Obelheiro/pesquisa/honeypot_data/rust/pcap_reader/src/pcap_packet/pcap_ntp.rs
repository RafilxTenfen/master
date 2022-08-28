use rtshark::Layer;
use rusqlite::{Connection, params};

// https://www.wireshark.org/docs/dfref/n/ntp.html

pub struct PcapNTP {
  pub id: i32,
  pub refid: String,
}

pub fn drop_table(conn: &Connection) {
  let result = conn.execute(
    "DROP TABLE IF EXISTS PCAP_NTP",
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

pub fn create_table(conn: &Connection) {
  let result = conn.execute(
    "CREATE TABLE IF NOT EXISTS PCAP_NTP (
      id INTEGER NOT NULL,
      refid TEXT NOT NULL
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

impl PcapNTP {
  pub fn default(id: i32) -> PcapNTP {
    PcapNTP {
      id,
      refid: String::from(""),
    }
  }

  pub fn insert(&self, conn: &Connection) {
    let result = conn.execute(
        "INSERT INTO PCAP_NTP (id, refid) values (?1, ?2)",
        params![&self.id, &self.refid],
    );

    match result {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting ntp: {:?}", err)
      }
    }
  }
}

pub fn pcap_process_layer_ntp(layer: Layer, conn: &Connection, id: i32) -> PcapNTP {
  let mut pcap_ntp = PcapNTP::default(id);

  if layer.name() != "ntp" {
    return pcap_ntp;
  }

  println!("Processing ntp");
  for metadata in layer {
    let (name, value, _display) = (metadata.name(), metadata.value(), metadata.display());
    match name {
      "ntp.refid" => {
        pcap_ntp.refid = value.to_string();
      }
      _ => {},
      // _ => println!("ignored field: {} = {} - {}", name, value, display),
    }
  }
  pcap_ntp.insert(conn);
  return pcap_ntp;
}
