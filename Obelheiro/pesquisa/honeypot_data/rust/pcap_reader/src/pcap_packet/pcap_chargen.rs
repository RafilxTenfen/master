use rtshark::{Layer, Metadata};
use rusqlite::{ Connection, params };

// https://www.wireshark.org/docs/dfref/c/chargen.html

pub struct PcapChargen {
  pub id: i32,
  pub data: String,
}

pub fn drop_table(conn: &Connection) {
  let result = conn.execute(
    "DROP TABLE IF EXISTS PCAP_CHARGEN",
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
    "CREATE TABLE IF NOT EXISTS PCAP_CHARGEN (
      id INTEGER NOT NULL,
      data TEXT NOT NULL
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

impl PcapChargen {
  pub fn default(id: i32) -> PcapChargen {
    PcapChargen {
      id,
      data: String::from(""),
    }
  }

  pub fn insert(&self, conn: &Connection) {
    let result = conn.execute(
        "INSERT INTO PCAP_CHARGEN (id, data) values (?1, ?2)",
        params![&self.id, &self.data],
    );

    match result {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting chargen: {:?}", err)
      }
    }
  }

  pub fn metadata_process(&mut self, metadata: &Metadata) {
    let (name, value) = (metadata.name(), metadata.value());
    match name {
      "chargen.data" => {
        self.data = value.to_string();
      }
      _ => {},
      // _ => println!("ignored field: {} = {} - {}", name, value, display),
    }
  }
}

pub fn pcap_process_layer_chargen(layer: &Layer, id: i32) -> PcapChargen {
  let mut pcap_chargen = PcapChargen::default(id);

  // if layer.name() != "chargen" {
  //   return pcap_chargen;
  // }

  println!("Processing chargen");
  layer.iter().for_each(|metadata| pcap_chargen.metadata_process(metadata));

  return pcap_chargen;
}
