use rusqlite::Result;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
// use cidr_utils::cidr::Ipv4Cidr;

mod database;
mod pcap;

pub fn get_current_working_dir() -> PathBuf {
  let working_dir = env::current_dir();
  let current_dir = match working_dir {
    Ok(workdir) => workdir,
    Err(_) => PathBuf::from("./"),
  };
  return current_dir;
}

fn main() -> Result<()> {
  let currently_dir = get_current_working_dir();
  println!("Working dir {}", currently_dir.display());

  let conn = database::conn_get_mix_protocol()?;
  database::drop_tables(&conn);
  database::create_tables(&conn);

  // HashMap CIDR => UDP dest port => Attack
  let mut hm_cidr_udp_attack = pcap::new_hm_cidr_udp_attack();
  let mut hm_id = HashMap::<&str, u32>::new();

  // loop entre vários arquivos pcaps, ordenados pela data '-'
  pcap::pcap_process_dir(
    &currently_dir.join("../../pcap"),
    &conn,
    &mut hm_cidr_udp_attack,
    &mut hm_id,
  );

  database::close(conn);

  Ok(())
}
