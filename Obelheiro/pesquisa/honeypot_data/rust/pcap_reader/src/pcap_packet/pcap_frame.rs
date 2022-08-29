use chrono::{DateTime, FixedOffset};
use rtshark::{Layer, Metadata};
use rusqlite::{params, Connection};

// https://www.wireshark.org/docs/dfref/f/frame.html

pub struct PcapFrame {
  pub id: i32,
  pub timestamp_str: String,
  pub timestamp: DateTime<FixedOffset>,
  pub time_epoch: String,
}

pub fn drop_table(conn: &Connection) {
  match conn.execute("DROP TABLE IF EXISTS PCAP_FRAME", []) {
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
    "CREATE TABLE IF NOT EXISTS PCAP_FRAME (
      id INTEGER NOT NULL,
      timestamp_str TEXT NOT NULL,
      time_epoch REAL NOT NULL
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

impl PcapFrame {
  pub fn default(id: i32) -> PcapFrame {
    PcapFrame {
      id,
      timestamp: DateTime::default(),
      timestamp_str: String::from(""),
      time_epoch: String::from(""),
    }
  }

  pub fn insert(&self, conn: &Connection) {
    match conn.execute(
      "INSERT INTO PCAP_FRAME (id, timestamp_str, time_epoch) values (?1, ?2, ?3)",
      params![&self.id, &self.timestamp_str.to_string(), &self.time_epoch],
    ) {
      Ok(_) => {
        println!("frame inserted")
      }
      Err(err) => {
        println!("Problem inserting frame: {:?}", err)
      }
    }
  }

  pub fn metadata_process(&mut self, metadata: &Metadata) {
    let (name, value) = (metadata.name(), metadata.value());
    match name {
      "frame.time" => {
        // "1983 Apr 13 12:09:14.274 +0000", "%Y %b %d %H:%M:%S%.3f %z");599409000
        let timestamp_formatted = format!("{}:00", value);
        let parsed_datetime =
          DateTime::parse_from_str(timestamp_formatted.as_str(), "%b %d, %Y %H:%M:%S%.9f %z");
        match parsed_datetime {
          Ok(datetime) => {
            self.timestamp = datetime;
          }
          Err(err) => println!("Error parse data: {:?} = {}", err, value),
        }
        self.timestamp_str = value.to_string();
      }
      "frame.time_epoch" => {
        self.time_epoch = value.to_string();
      }
      _ => {} // _ => println!("ignored field: {} = {} - {}", name, value, _display),
    }
  }
}

pub fn pcap_process_layer_frame(layer: &Layer, id: &i32) -> PcapFrame {
  let mut pcap_frame = PcapFrame::default(*id);

  println!("Processing frame");
  layer
    .iter()
    .for_each(|metadata| pcap_frame.metadata_process(metadata));

  return pcap_frame;
}
