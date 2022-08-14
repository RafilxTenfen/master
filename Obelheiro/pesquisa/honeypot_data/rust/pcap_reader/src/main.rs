use rtshark::{
  RTSharkBuilder
};
use std::env;
use std::path::PathBuf;
// use std::path::Path;

fn get_current_working_dir() -> std::io::Result<PathBuf> {
  env::current_dir()
}

fn main() {
  let working_dir = get_current_working_dir();
  let currently_dir = match working_dir {
    Ok(workdir) => workdir,
    Err(error) => panic!("Problem reading current dir: {:?}", error),
  };
  println!("Working dir {}", currently_dir.display());

  let pcap_dir = currently_dir.as_path().join("../../pcap/tcpdump.out.2021-04-14_14-55-38.377363_dcn.pcap00000000");
  let pcap_str = pcap_dir.into_os_string().into_string().unwrap();
  println!("PCAP dir {}", pcap_str);

  let builder = RTSharkBuilder::builder().input_path(&pcap_str);
  // Start a new TShark process
  let _rtshark = match builder.spawn() {
    Err(err) =>  { eprintln!("Error running tshark: {err}"); return }
    Ok(rtshark) => rtshark,
  };
  println!("Hello, world!");
}
