extern crate dotenv;

use cidr_utils::cidr::Ipv4Cidr;
use dotenv::dotenv;
use rusqlite::Result;
// use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

// use cidr_utils::cidr::Ipv4Cidr;
// use tokio;
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

// #[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
fn main() -> Result<()> {
  dotenv().ok();
  let currently_dir = get_current_working_dir();
  println!("Working dir {}", currently_dir.display());

  // remind to set wal
  // PRAGMA journal_mode=WAL;
  // https://sqlite.org/wal.html
  // Another option is to run PRAGMA synchronous=OFF.
  // This command will cause SQLite to not wait on data to reach the disk surface,
  // which will make write operations appear to be much faster.
  // But if you lose power in the middle of a transaction, your database file might go corrupt.
  let mut conn = database::conn_get_rust_pcap().unwrap();
  // let (conn, connection) = database::conn_gcp_async_rust_pcap().await;
  // tokio::spawn(async move {
  //   if let Err(e) = connection.await {
  //       eprintln!("connection error: {}", e);
  //   }
  // });

  // database::journal_mode(&mut conn);
  database::drop_tables(&mut conn);
  database::create_tables(&mut conn);
  // database::disable_vacuum(&mut conn);

  // HashMap CIDR => UDP dest port => Attack
  let mut hm_cidr_udp_attack = pcap::new_hm_cidr_udp_attack();
  let mut hm_id = HashMap::<&str, u32>::new();
  let mut hm_ip_cidr = HashMap::<String, Ipv4Cidr>::new();
  let mut hm_ip_id = HashMap::<String, u32>::new();
  let mut hm_cidr_id = HashMap::<Ipv4Cidr, u32>::new();
  let mut tb_ip_id: u32 = 0;

  // Done: PROCESSAR bzip, abrir e ler pcap
  // https://docs.rs/bzip2/latest/bzip2/
  // ls | wc -l: 39066 pcap files

  // TODO:
  // SALVAR SOMENTE ATAQUES de pcaps que foram totalmente processados
  // salvar no db os pcaps ja processados

  // loop entre vários arquivos pcaps, ordenados pela data '-'
  pcap::pcap_process_dir(
    &currently_dir.join("../../pcap/test/pcap_files"),
    // &currently_dir.join("../../pcap"),
    &mut conn,
    &mut hm_cidr_udp_attack,
    &mut hm_id,
    &mut tb_ip_id,
    &mut hm_ip_cidr,
    &mut hm_ip_id,
    &mut hm_cidr_id,
  );

  // match conn.close() {
  //   Ok(_) => {
  //     print!("gcp db conn closed")
  //   }
  //   Err(err) => {
  //     println!("gcp sql close err {}", err)
  //   }
  // }
  database::close(conn);

  Ok(())
}
