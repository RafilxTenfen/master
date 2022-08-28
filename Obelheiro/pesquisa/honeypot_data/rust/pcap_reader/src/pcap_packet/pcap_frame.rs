use rtshark::Layer;
use rusqlite::{Connection, params};
use chrono::{DateTime, FixedOffset};

// https://www.wireshark.org/docs/dfref/f/frame.html

pub struct PcapFrame {
  pub id: i32,
  pub timestamp_str: String,
  pub timestamp: DateTime<FixedOffset>,
  pub time_epoch: String,
}

pub fn drop_table(conn: &Connection) {
  let result = conn.execute(
    "DROP TABLE IF EXISTS PCAP_FRAME",
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
    "CREATE TABLE IF NOT EXISTS PCAP_FRAME (
      id INTEGER NOT NULL,
      timestamp_str TEXT NOT NULL,
      time_epoch REAL NOT NULL
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
    let result = conn.execute(
        "INSERT INTO PCAP_FRAME (id, timestamp_str, time_epoch) values (?1, ?2, ?3)",
        params![&self.id, &self.timestamp_str.to_string(), &self.time_epoch],
    );

    match result {
      Ok(_) => {
        println!("frame inserted")
      }
      Err(err) => {
        println!("Problem inserting frame: {:?}", err)
      }
    }
  }
}

pub fn pcap_process_layer_frame(layer: Layer, conn: &Connection, id: i32) -> PcapFrame {
  let mut pcap_frame = PcapFrame::default(id);

  if layer.name() != "frame" {
    return pcap_frame;
  }

  println!("Processing frame");
  for metadata in layer {
    let (name, value, _display) = (metadata.name(), metadata.value(), metadata.display());
    match name {
      "frame.time" => {
        // SystemTime::from(value);
        // println!("timestamp: {}", value);
        // "1983 Apr 13 12:09:14.274 +0000", "%Y %b %d %H:%M:%S%.3f %z");599409000
        // let q = DateTime::from_str(value);
        let timestamp_formatted = format!("{}:00", value);
        let parsed_datetime = DateTime::parse_from_str(timestamp_formatted.as_str(), "%b %d, %Y %H:%M:%S%.9f %z");
        match parsed_datetime {
          Ok(datetime) => {
            pcap_frame.timestamp = datetime;
          },
          Err(err) => println!("Error parse data: {:?} = {}", err, value)
        }
        pcap_frame.timestamp_str = value.to_string();
      }
      // "frame.time_epoch" => {
      //   pcap_frame.time_epoch = value.to_string();
      // },
      _ => {},
      // _ => println!("ignored field: {} = {} - {}", name, value, _display),
    }
  }
  pcap_frame.insert(conn);
  return pcap_frame;
}
