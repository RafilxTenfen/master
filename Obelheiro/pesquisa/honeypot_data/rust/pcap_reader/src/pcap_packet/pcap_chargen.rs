use rtshark::{Layer, Metadata};
use rusqlite::{params, Connection};

// https://www.wireshark.org/docs/dfref/c/chargen.html

pub struct PcapChargen {
  pub id: i32,
  pub data: String,
}

pub fn drop_table(conn: &Connection) {
  match conn.execute("DROP TABLE IF EXISTS PCAP_CHARGEN", []) {
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
    "CREATE TABLE IF NOT EXISTS PCAP_CHARGEN (
      id INTEGER NOT NULL,
      data TEXT NOT NULL
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

impl PcapChargen {
  pub fn default(id: i32) -> PcapChargen {
    PcapChargen {
      id,
      data: String::from(""),
    }
  }

  pub fn insert(&self, conn: &Connection) {
    match conn.execute(
      "INSERT INTO PCAP_CHARGEN (id, data) values (?1, ?2)",
      params![&self.id, &self.data],
    ) {
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
      _ => {
        println!("{}", name)
      } // _ => {} // _ => println!("ignored field: {} = {} - {}", name, value, display),
    }
  }
}

pub fn pcap_process_layer_chargen(layer: &Layer, id: i32) -> PcapChargen {
  let mut pcap_chargen = PcapChargen::default(id);

  println!("Processing chargen");
  layer
    .iter()
    .for_each(|metadata| pcap_chargen.metadata_process(metadata));

  return pcap_chargen;
}
