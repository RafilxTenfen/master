use cidr_utils::cidr::Ipv4Cidr;
use rusqlite::Result;
// use std::borrow::BorrowMut;
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

  // remind to set wal
  // PRAGMA journal_mode=WAL;
  // https://sqlite.org/wal.html
  // Another option is to run PRAGMA synchronous=OFF.
  // This command will cause SQLite to not wait on data to reach the disk surface,
  // which will make write operations appear to be much faster.
  // But if you lose power in the middle of a transaction, your database file might go corrupt.
  let mut conn = database::conn_get_mix_protocol()?;
  database::journal_mode(&conn);
  database::drop_tables(&conn);
  database::create_tables(&conn);

  // HashMap CIDR => UDP dest port => Attack
  let mut hm_cidr_udp_attack = pcap::new_hm_cidr_udp_attack();
  let mut hm_id = HashMap::<&str, u32>::new();
  let mut hm_ip_cidr = HashMap::<String, Ipv4Cidr>::new();

  // loop entre vários arquivos pcaps, ordenados pela data '-'
  pcap::pcap_process_dir(
    // &currently_dir.join("../../pcap/test/pcap_files"),
    &currently_dir.join("../../pcap"),
    &mut conn,
    &mut hm_cidr_udp_attack,
    &mut hm_id,
    &mut hm_ip_cidr,
  );

  database::close(conn);

  Ok(())
}
