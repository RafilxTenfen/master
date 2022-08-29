use rtshark::{Layer, Metadata};
use rusqlite::{params, Connection};

// https://www.wireshark.org/docs/dfref/c/chargen.html

pub struct PcapSSDP {
  pub id: i32,
  pub full_uri: String,
}

pub fn drop_table(conn: &Connection) {
  match conn.execute("DROP TABLE IF EXISTS PCAP_SSDP", []) {
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
    "CREATE TABLE IF NOT EXISTS PCAP_SSDP (
      id INTEGER NOT NULL,
      full_uri TEXT NOT NULL
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

impl PcapSSDP {
  pub fn default(id: i32) -> PcapSSDP {
    PcapSSDP {
      id,
      full_uri: String::from(""),
    }
  }

  pub fn insert(&self, conn: &Connection) {
    match conn.execute(
      "INSERT INTO PCAP_SSDP (id, full_uri) values (?1, ?2)",
      params![&self.id, &self.full_uri],
    ) {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting ssdp: {:?}", err)
      }
    }
  }

  pub fn metadata_process(&mut self, metadata: &Metadata) {
    let (name, value) = (metadata.name(), metadata.value());
    match name {
      "http.request.full_uri" => {
        self.full_uri = value.to_string();
      }
      _ => {} // _ => println!("ignored field: {} = {} - {}", name, value, display),
    }
  }
}

pub fn pcap_process_layer_ssdp(layer: &Layer, id: &i32) -> PcapSSDP {
  let mut pcap_ssdp = PcapSSDP::default(*id);

  // if layer.name() != "ssdp" {
  //   return pcap_ssdp;
  // }

  println!("Processing ssdp");
  layer
    .iter()
    .for_each(|metadata| pcap_ssdp.metadata_process(metadata));

  return pcap_ssdp;
}
