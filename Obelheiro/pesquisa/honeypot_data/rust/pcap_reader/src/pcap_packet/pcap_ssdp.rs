use rtshark::Layer;
use rusqlite::Connection;

// https://www.wireshark.org/docs/dfref/c/chargen.html

pub struct PcapSSDP {
  pub id: i32,
  pub full_uri: String,
}

pub fn drop_table(conn: &Connection) {
  let result = conn.execute(
    "DROP TABLE IF EXISTS PCAP_SSDP",
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
    "CREATE TABLE IF NOT EXISTS PCAP_SSDP (
      id INTEGER PRIMARY KEY,
      full_uri TEXT NOT NULL
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

impl PcapSSDP {
  pub fn default(id: i32) -> PcapSSDP {
    PcapSSDP {
      id,
      full_uri: String::from(""),
    }
  }

  pub fn insert(&self, conn: &Connection) {
    let result = conn.execute(
        "INSERT INTO PCAP_SSDP (id, full_uri) values (?1, ?2)",
        &[&self.id.to_string(), &self.full_uri.to_string()],
    );

    match result {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting ssdp: {:?}", err)
      }
    }
  }
}

pub fn pcap_process_layer_ssdp(layer: Layer, conn: &Connection, id: i32) -> PcapSSDP {
  let mut pcap_ssdp = PcapSSDP::default(id);

  if layer.name() != "ssdp" {
    return pcap_ssdp;
  }

  println!("Processing ssdp");
  for metadata in layer {
    let (name, value, _display) = (metadata.name(), metadata.value(), metadata.display());
    match name {
      "http.request.full_uri" => {
        pcap_ssdp.full_uri = value.to_string();
      }
      _ => {},
      // _ => println!("ignored field: {} = {} - {}", name, value, display),
    }
  }
  pcap_ssdp.insert(conn);
  return pcap_ssdp;
}
