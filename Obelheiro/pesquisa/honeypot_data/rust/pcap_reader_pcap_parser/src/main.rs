// use chrono::{DateTime, Duration, FixedOffset};
// use cidr_utils::cidr::Ipv4Cidr;
// use circular::Buffer;
use pcap_parser::traits::PcapReaderIterator;
use pcap_parser::*;
// use rtshark::{RTShark, RTSharkBuilder};
// use rusqlite::{params, Connection, Result};
use rusqlite::{Connection, Result};
// use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
// use std::io::{self, BufRead};
use etherparse::SlicedPacket;
use pktparse::{ethernet, icmp, ipv4, udp};
use std::path::PathBuf;
// use snoopy::{}/
// use dnslogger; // Error   = note: /usr/bin/ld: cannot find -lpcap: No such file or directory
//                // collect2: error: ld returned 1 exit status
//                // use packet::{Packet, AsPacket};
//                // use packet::AsPacket;
//                // use packet::ether;
//                // use std::time::Duration;
use libpcap_tools::*;


fn get_current_working_dir() -> PathBuf {
  let working_dir = env::current_dir();
  let current_dir = match working_dir {
    Ok(workdir) => workdir,
    Err(error) => panic!("Problem reading current dir: {:?}", error),
  };
  return current_dir;
}

fn main() -> Result<()> {
  let currently_dir = get_current_working_dir();
  println!("Working dir {}", currently_dir.display());

  let db_path = currently_dir
    .as_path()
    .join("../../db/database-2022-05-11/mix_protocol.sqlite");
  let _conn = Connection::open(db_path)?;
  // // drop_tables(&conn);
  // // create_tables(&conn);

  // loop entre vários arquivos pcaps, ordenados pela data '-'
  let pcap_str = currently_dir
    .as_path()
    .join("../../pcap/tcpdump.out.2021-04-14_14-55-38.377363_dcn.pcap00000000")
    .into_os_string()
    .into_string()
    .unwrap();
  println!("PCAP dir {}", pcap_str);

  let file_result = File::open(
    currently_dir
      .as_path()
      .join("../../pcap/tcpdump.out.2021-04-14_14-55-38.377363_dcn.pcap00000000"),
  );

  match file_result {
    Ok(file) => {
      let reader = BufReader::new(file);

      let mut num_blocks = 0;
      let mut reader = LegacyPcapReader::new(65536, reader).expect("LegacyPcapReader");

      loop {
        match reader.next() {
          Ok((offset, block)) => {
            num_blocks += 1;
            println!("got new block: {}", num_blocks);
            match block {
              PcapBlockOwned::LegacyHeader(_hdr) => {
                println!("PcapBlockOwned::LegacyHeader");
                // linktype_data = _hdr.network
                // save hdr.network (linktype)
                // match hdr.network {
                //   Linktype::IPV4(ipv4) => {},
                //   (_) => {},
                // }
              }
              PcapBlockOwned::Legacy(b) => {

                // match dns_parser::Packet::parse(b.data) {
                //   Ok(_packet) => {
                //     println!("SlicedPacket DNS parsed");
                //   },
                //   Err(_err) => {
                //     println!("Err dns_parser::Packet::parse {}", _err)
                //   }
                // }

                match SlicedPacket::from_ethernet(b.data) {
                  Ok(_sliced_packet) => {
                    println!("SlicedPacket::from_ethernet parsed");

                    match dns_parser::Packet::parse(_sliced_packet.payload) {
                      Ok(_packet) => {
                        println!("SlicedPacket DNS parsed");
                        if _packet.questions.len() > 1 {
                          println!("_packet.questions.len() > 1: {}", _packet.questions.len());
                        }

                        for dns_question in _packet.questions {
                          match dns_question.qtype {
                            dns_parser::QueryType::A => {
                              println!("DNS parse _packet.questions.dns_question.qtype: A");
                            },
                            dns_parser::QueryType::NS => {
                              println!("DNS parse _packet.questions.dns_question.qtype: NS");
                            },
                            dns_parser::QueryType::MF => {
                              println!("DNS parse _packet.questions.dns_question.qtype: MF");
                            },
                            dns_parser::QueryType::CNAME => {
                              println!("DNS parse _packet.questions.dns_question.qtype: CNAME");
                            },
                            dns_parser::QueryType::SOA => {
                              println!("DNS parse _packet.questions.dns_question.qtype: SOA");
                            },
                            dns_parser::QueryType::MB => {
                              println!("DNS parse _packet.questions.dns_question.qtype: MB");
                            },
                            dns_parser::QueryType::MG => {
                              println!("DNS parse _packet.questions.dns_question.qtype: MG");
                            },
                            dns_parser::QueryType::MR => {
                              println!("DNS parse _packet.questions.dns_question.qtype: MR");
                            },
                            dns_parser::QueryType::NULL => {
                              println!("DNS parse _packet.questions.dns_question.qtype: NULL");
                            },
                            dns_parser::QueryType::WKS => {
                              println!("DNS parse _packet.questions.dns_question.qtype: WKS");
                            },
                            dns_parser::QueryType::PTR => {
                              println!("DNS parse _packet.questions.dns_question.qtype: PTR");
                            },
                            dns_parser::QueryType::HINFO => {
                              println!("DNS parse _packet.questions.dns_question.qtype: HINFO");
                            },
                            dns_parser::QueryType::MINFO => {
                              println!("DNS parse _packet.questions.dns_question.qtype: MINFO");
                            },
                            dns_parser::QueryType::MX => {
                              println!("DNS parse _packet.questions.dns_question.qtype: MX");
                            },
                            dns_parser::QueryType::TXT => {
                              println!("DNS parse _packet.questions.dns_question.qtype: TXT");
                            },
                            dns_parser::QueryType::AAAA => {
                              println!("DNS parse _packet.questions.dns_question.qtype: AAAA");
                            },
                            dns_parser::QueryType::SRV => {
                              println!("DNS parse _packet.questions.dns_question.qtype: SRV");
                            },
                            dns_parser::QueryType::AXFR => {
                              println!("DNS parse _packet.questions.dns_question.qtype: AXFR");
                            },
                            dns_parser::QueryType::MAILB => {
                              println!("DNS parse _packet.questions.dns_question.qtype: MAILB");
                            },
                            dns_parser::QueryType::MAILA => {
                              println!("DNS parse _packet.questions.dns_question.qtype: MAILA");
                            },
                            dns_parser::QueryType::All => {
                              println!("DNS parse _packet.questions.dns_question.qtype: All");
                            },
                          }
                          println!(
                            "DNS parse _packet.questions.dns_question.qname: {}",
                            dns_question.qname
                          )
                        }
                        for dns_rr in _packet.nameservers {
                          println!("DNS parse _packet.nameservers.dns_rr.name: {}", dns_rr.name)
                        }
                        for dns_rr in _packet.answers {
                          println!("DNS parse _packet.answers.dns_rr.name: {}", dns_rr.name)
                        }
                        for dns_rr in _packet.additional {
                          println!("DNS parse _packet.additional.dns_rr.name: {}", dns_rr.name)
                        }
                      }
                      Err(_err) => {
                        println!("Err dns_parser::Packet::parse {}", _err)
                      }
                    }
                  }
                  Err(_err) => {
                    println!("Err SlicedPacket::from_ethernet {}", _err)
                  }
                }

                match udp::parse_udp_header(b.data) {
                  Ok((_data, _udp)) => {
                    println!("UDP dest port {}", _udp.dest_port);
                  }
                  Err(err) => {
                    println!("Err parse_udp_header {}", err)
                  }
                }

                match icmp::parse_icmp_header(b.data) {
                  Ok((_data, _icmp)) => {
                    println!("ICMP checksum {}", _icmp.checksum);
                  }
                  Err(err) => {
                    println!("Err parse_icmp_header {}", err)
                  }
                }

                match ipv4::parse_ipv4_header(b.data) {
                  Ok((_data, _ipv4_header)) => {
                    println!(
                      "IPV4 source {}, dest {}",
                      _ipv4_header.source_addr.to_string(),
                      _ipv4_header.dest_addr.to_string()
                    );
                  }
                  Err(err) => {
                    println!("Err parse_ipv4_header {}", err)
                  }
                }

                match ethernet::parse_ethernet_frame(b.data) {
                  Ok((_data, _ethernet_frame)) => {
                    println!("ETHERNET parsed",);
                  }
                  Err(err) => {
                    println!("Err parse_ethernet_frame {}", err)
                  }
                }
              }
              PcapBlockOwned::NG(_x) => {
                println!("PcapBlockOwned::NG");
              }
            }
            reader.consume(offset);
          }

          Err(PcapError::Eof) => break,
          Err(PcapError::Incomplete) => {
            reader.refill().unwrap();
          }
          Err(e) => panic!("error while reading: {:?}", e),
        }
      }
      println!("num_blocks: {}", num_blocks);
    }
    Err(err) => {
      println!("err reading file {}", err)
    }
  }

  Ok(())
}
