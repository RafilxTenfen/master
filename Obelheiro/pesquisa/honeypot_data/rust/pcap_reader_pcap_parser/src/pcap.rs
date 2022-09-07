use pcap_parser::{traits::PcapReaderIterator, LegacyPcapReader, PcapError};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
mod block;

fn list_files(pathname: &PathBuf, filter: &str) -> Option<Vec<PathBuf>> {
  Some(
    fs::read_dir(pathname)
      .ok()?
      .filter_map(|entry| {
        Some(
          pathname.join(
            entry
              .ok()?
              .path()
              .strip_prefix(pathname)
              .ok()?
              .to_path_buf(),
          ),
        )
      })
      .filter(|path| path.ends_with(filter))
      .collect(),
  )
}

pub fn get_pcaps_ordered(dir: &PathBuf) -> Vec<PathBuf> {
  let pcaps = vec![];

  if !dir.is_dir() {
    return pcaps;
  }

  match list_files(dir, "") {
    Some(mut files) => {
      files.sort();
      return files;
    }
    None => {}
  }

  return pcaps;
}

pub fn pcap_process_dir(dir: &PathBuf) {
  println!("pcap_process_dir {}", dir.display());

  let pcaps = get_pcaps_ordered(dir);
  pcaps.iter().for_each(|pcap| pcap_process(pcap));
}

pub fn pcap_process(pcap: &PathBuf) {
  if !pcap.is_file() {
    println!("pcap {} is not a file", pcap.display());
    return;
  }

  println!("processing pcap {}", pcap.display());

  match File::open(pcap) {
    Ok(file) => {
      // let mut reader = LegacyPcapReader::new(65536, reader).expect("LegacyPcapReader");
      match LegacyPcapReader::new(65536, BufReader::new(file)) {
        Ok(ref mut reader) => {
          loop {
            match reader.next() {
              Ok((offset, ref block)) => {
                block::process_block(block);
                reader.consume(offset)
              }
              Err(PcapError::Eof) => break,
              Err(PcapError::Incomplete) => {
                reader.refill().unwrap();
              }
              Err(e) => panic!("error while reading: {:?}", e),
            }
          }
          // legacy_pcap_reader::process_legacy_pcap_reader(legacyPcapReader);
        }
        Err(err) => {
          println!("Error reading LegacyPcapReader {}", err)
        }
      };
    }
    Err(err) => {
      println!("Error openning file {}", err)
    }
  }
}
