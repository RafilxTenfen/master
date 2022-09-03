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
use dnslogger; // Error   = note: /usr/bin/ld: cannot find -lpcap: No such file or directory
               // collect2: error: ld returned 1 exit status
               // use packet::{Packet, AsPacket};
               // use packet::AsPacket;
               // use packet::ether;
               // use std::time::Duration;
use std::ascii::escape_default;
use std::str;

use libpcap_tools::*;

// mod pcap_packet;
// use std::path::Path;

fn show(bs: &[u8]) -> String {
  let mut visible = String::new();
  for &b in bs {
    let part: Vec<u8> = escape_default(b).collect();
    visible.push_str(str::from_utf8(&part).unwrap());
  }
  visible
}

fn main() -> Result<()> {
  let currently_dir = get_current_working_dir();
  println!("Working dir {}", currently_dir.display());

  let db_path = currently_dir
    .as_path()
    .join("../../db/database-2022-05-11/mix_protocol.sqlite");
  let conn = Connection::open(db_path)?;
  // drop_tables(&conn);
  // create_tables(&conn);

  // loop entre vários arquivos pcaps, ordenados pela data '-'
  let pcap_str = currently_dir
    .as_path()
    .join("../../pcap/tcpdump.out.2021-04-14_14-55-38.377363_dcn.pcap00000000")
    .into_os_string()
    .into_string()
    .unwrap();
  println!("PCAP dir {}", pcap_str);

  // HashMap CIDR => UDP dest port => Attack
  // let mut map_attacks = HashMap::<Ipv4Cidr, HashMap<i32, PcapAttack>>::new();
  // let mut map_id = HashMap::<&str, i32>::new();
  // let mut id_attack = 0;

  let file_result = File::open(
    currently_dir
      .as_path()
      .join("../../pcap/tcpdump.out.2021-04-14_14-55-38.377363_dcn.pcap00000000"),
  );

  let mut pcap_index = 0;
  match file_result {
    Ok(file) => {
      let reader = BufReader::new(file);
      let info = CurrentSectionInfo::default();
      // let buffer = Buffer::with_capacity(65536);

      // pub fn parse_packet(packet: &pcap::Packet) -> Option<PacketPrinter> {
      //   let ethernet = EthernetPacket::new(packet.data).unwrap();
      //   match ethernet.get_ethertype() {
      //     EtherTypes::Ipv4 => {
      //       let ipv4_packet = Ipv4Packet::new(ethernet.payload()).unwrap();
      //       match ipv4_packet.get_next_level_protocol() {
      //         IpNextHeaderProtocols::Udp => {
      //           let udp_packet = UdpPacket::new(ipv4_packet.payload()).unwrap();
      //           let (_rest, dns_message) = dns_message(udp_packet.payload(), true).unwrap();
      //           Some(PacketPrinter::new(
      //             packet,
      //             &ipv4_packet,
      //             &udp_packet,
      //             &dns_message,
      //           ))
      //         }
      //         _ => None,
      //       }
      //     }
      //     _ => None,
      //   }
      // }
      // PcapNGReader {
      //   info,
      //   reader,
      //   buffer,
      //   consumed: 0,
      //   reader_exhausted: false,
      // };

      let mut num_blocks = 0;
      // 65536
      // match PcapNGReader::new(65536, reader) {
      //   Ok(X) => println!("read PcapNGReader"),
      //   Err(err) => {},
      // }

      let mut reader = LegacyPcapReader::new(65536, reader).expect("LegacyPcapReader");
      // reader
      let mut _linktype_data = Linktype::NULL;
      let _resolve_all_resource_records = false;
      loop {
        // reader.consumed()
        match reader.next() {
          Ok((offset, block)) => {
            num_blocks += 1;
            // println!("got new block: {}", num_blocks);
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
                            // dns_parser::QueryType::MX => {
                            //   println!("DNS parse _packet.questions.dns_question.qtype: MX");
                            // }
                            // _ => {
                            //   println!("DNS parse _packet.questions.dns_question.qtype: OTHER");
                            // }
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
                        // println!("Err dns_parser::Packet::parse {}", err)
                      }
                    }
                  }
                  Err(_err) => {
                    // println!("Err SlicedPacket::from_ethernet {}", err)
                  }
                }

                // _b.data
                // println!("PcapBlockOwned::Legacy : {}", show(b.data));
                // _b.
                let blen = b.caplen as usize;
                match dns_parser::Packet::parse(b.data) {
                  Ok(_packet) => {
                    println!("Packet parsed");
                  }
                  Err(_err) => {
                    // println!("Err dns_parser::Packet::parse {}", err)
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
                    // println!("parse_ipv4_header: {}", show(_data));
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
                    // println!("parse_ipv4_header: {}", show(_data));
                    println!("ETHERNET parsed",);
                  }
                  Err(err) => {
                    println!("Err parse_ethernet_frame {}", err)
                  }
                }

                // let data =
                //   .ok_or(Error::Generic("Parsing PacketData failed (Legacy Packet)"))?;

                // match b.data.as_packet() {
                //     Ok(packet) => {

                //     },
                //     Err(err) => {
                //       println!("err as packet {}", err);
                //     },
                // }

                match pcap_parser::data::get_packetdata(b.data, _linktype_data, blen) {
                  Some(packet_data) => {
                    pcap_index += 1;
                    // let packet = libpcap_tools::Packet{
                    //   interface: 0,
                    //   ts: Duration::default(),
                    //   data: packet_data,
                    //   link_type: linktype_data,
                    //   origlen: b.origlen,
                    //   caplen: b.caplen,
                    //   pcap_index: pcap_index,
                    // };

                    // packet.
                    // packet_data.
                    // packet_data.a
                    // packet_data.
                    match packet_data {
                      data::PacketData::L2(l2) => {
                        // println!("L2");
                        match dns_parser::Packet::parse(l2) {
                          Ok(_packet) => {
                            println!("Packet parsed");
                          }
                          Err(_err) => {
                            // println!("Err dns_parser::Packet::parse {}", err)
                          }
                        }
                      }
                      data::PacketData::L3(_, l3) => {
                        // println!("L3");
                        match dns_parser::Packet::parse(l3) {
                          Ok(_packet) => {
                            println!("Packet parsed");
                          }
                          Err(_err) => {
                            // println!("Err dns_parser::Packet::parse {}", err)
                          }
                        }
                      }
                      data::PacketData::L4(_x, l4data) => {
                        println!("L4");
                        // dns_parser::Packet::parse(l4data)
                        match dns_parser::Packet::parse(l4data) {
                          Ok(_packet) => {
                            println!("Packet parsed");
                          }
                          Err(err) => {
                            println!("Err dns_parser::Packet::parse {}", err)
                          }
                        }
                      }
                      data::PacketData::Unsupported(_) => {}
                    }
                    // Packet {
                    //   interface: 0,
                    //   ts,
                    //   link_type: linktype_data,
                    //   packet_data,
                    //   origlen: b.origlen,
                    //   caplen: b.caplen,
                    //   pcap_index: self.ctx.pcap_index,
                    // }
                    // Packet
                    // println!("tao")
                  }
                  None => {
                    println!("failed to read get_packetdata")
                  }
                }
                // pcap_parser::Data
                // PcapDataEngine::
                // pcap_parser::Data::
                // RTShark
                // _b./
                // _b.`
                // use linktype to parse b.data()
                // b.data
              }
              PcapBlockOwned::NG(_x) => {
                println!("PcapBlockOwned::NG");
              }
            }
            reader.consume(offset);
            // block./
          }

          Err(PcapError::Eof) => break,
          Err(PcapError::Incomplete) => {
            reader.refill().unwrap();
          }
          Err(e) => panic!("error while reading: {:?}", e),
        }
      }
      println!("num_blocks: {}", num_blocks);
      // PcapNGReader::new(65536, buffered).expect("PcapNGReader");
    }
    Err(err) => {
      println!("err reading file {}", err)
    }
  }

  //   match file {
  //     Ok(_) => todo!(),
  //     Err() => todo!(),
  // }

  //

  // let mut reader = PcapNGReader::new(65536, buffered).expect("PcapNGReader");
  // let mut num_blocks = 0;
  // // let mut reader = PcapNGReader::new(65536, file).expect("PcapNGReader");
  // loop {
  //   match reader.next() {
  //     Ok((offset, _block)) => {
  //       println!("got new block");
  //       num_blocks += 1;
  //       reader.consume(offset);
  //     }
  //     Err(PcapError::Eof) => break,
  //     Err(PcapError::Incomplete) => {
  //       reader.refill().unwrap();
  //     }
  //     Err(e) => panic!("error while reading: {:?}", e),
  //   }
  // }
  // println!("num_blocks: {}", num_blocks);

  // let builder = RTSharkBuilder::builder().input_path(&pcap_str);

  // let main_path =  get_current_working_dir();
  // let metadata_blacklist_path = ;

  // println!("rust fields {}", metadata_blacklist_path.to_str().stri);

  // File hosts must exist in current path before this produces output
  // if let Ok(lines) = read_lines(currently_dir.as_path().join("../tshark.fields").as_path()) {
  //   // Consumes the iterator, returns an (Optional) String
  //   for line in lines {
  //     if let Ok(field) = line {
  //       // println!("{}", field);
  //       // let mut strfield = field.clone();
  //       // builder = builder.metadata_blacklist(&strfield);
  //       builder.metadata_blacklist(&field);
  //     }
  //   }
  // }

  // Start a new TShark process
  // let rtshark = builder
  //   .metadata_blacklist("dns.flags")
  //   .metadata_blacklist("dns.flags.response")
  //   .metadata_blacklist("dns.flags.opcode")
  //   .metadata_blacklist("dns.flags.truncated")
  //   .metadata_blacklist("dns.flags.recdesired")
  //   .metadata_blacklist("dns.flags.checkdisable")
  //   .metadata_blacklist("dns.retransmit_request")
  //   .metadata_blacklist("dns.retransmit_request_in")
  //   .metadata_blacklist("dns.retransmission")
  //   .metadata_blacklist("dns.count.queries")
  //   .metadata_blacklist("dns.count.answers")
  //   .metadata_blacklist("dns.count.auth_rr")
  //   .metadata_blacklist("dns.count.add_rr")
  //   .metadata_blacklist("dns.resp.name")
  //   .metadata_blacklist("dns.resp.type")
  //   .metadata_blacklist("dns.resp.ext_rcode")
  //   .metadata_blacklist("dns.resp.edns0_version")
  //   .metadata_blacklist("dns.resp.len")
  //   .metadata_blacklist("dns.flags.z")
  //   .metadata_blacklist("dns.resp.z")
  //   .metadata_blacklist("dns.resp.z.do")
  //   .metadata_blacklist("dns.resp.z.reserved")
  //   .metadata_blacklist("dns.qry.name.len")
  //   .metadata_blacklist("dns.qry.class")
  //   .metadata_blacklist("dns.count.labels")
  //   .metadata_blacklist("dns.flags.authoritative")
  //   .metadata_blacklist("dns.flags.recavail")
  //   .metadata_blacklist("dns.flags.authenticated")
  //   .metadata_blacklist("dns.flags.rcode")
  //   .metadata_blacklist("dns.resp.class")
  //   .metadata_blacklist("dns.resp.ttl")
  //   .metadata_blacklist("dns.soa.mname")
  //   .metadata_blacklist("dns.soa.rname")
  //   .metadata_blacklist("dns.soa.serial_number")
  //   .metadata_blacklist("dns.soa.refresh_interval")
  //   .metadata_blacklist("dns.soa.retry_interval")
  //   .metadata_blacklist("dns.soa.expire_limit")
  //   .metadata_blacklist("dns.soa.minimum_ttl")
  //   .metadata_blacklist("dns.resp.class")
  //   .metadata_blacklist("dns.resp.ttl")
  //   .metadata_blacklist("dns.rrsig.type_covered")
  //   .metadata_blacklist("dns.rrsig.algorithm")
  //   .metadata_blacklist("dns.rrsig.labels")
  //   .metadata_blacklist("dns.rrsig.original_ttl")
  //   .metadata_blacklist("dns.rrsig.signature_expiration")
  //   .metadata_blacklist("dns.rrsig.signature_inception")
  //   .metadata_blacklist("dns.rrsig.key_tag")
  //   .metadata_blacklist("dns.rrsig.signers_name")
  //   .metadata_blacklist("dns.rrsig.signature")
  //   .metadata_blacklist("dns.resp.class")
  //   .metadata_blacklist("dns.resp.ttl")
  //   .metadata_blacklist("dns.mx.preference")
  //   .metadata_blacklist("dns.mx.mail_exchange")
  //   .metadata_blacklist("dns.rrsig.type_covered")
  //   .metadata_blacklist("dns.rrsig.algorithm")
  //   .metadata_blacklist("dns.rrsig.labels")
  //   .metadata_blacklist("dns.rrsig.original_ttl")
  //   .metadata_blacklist("dns.rrsig.signature_expiration")
  //   .metadata_blacklist("dns.rrsig.signature_inception")
  //   .metadata_blacklist("dns.rrsig.key_tag")
  //   .metadata_blacklist("dns.rrsig.signers_name")
  //   .metadata_blacklist("dns.rrsig.signature")
  //   .metadata_blacklist("dns.response_to")
  //   .metadata_blacklist("dns.time")
  //   .metadata_blacklist("ip.version")
  //   .metadata_blacklist("ip.hdr_len")
  //   .metadata_blacklist("ip.dsfield")
  //   .metadata_blacklist("ip.dsfield.dscp")
  //   .metadata_blacklist("ip.dsfield.ecn")
  //   .metadata_blacklist("ip.len")
  //   .metadata_blacklist("ip.id")
  //   .metadata_blacklist("ip.flags")
  //   .metadata_blacklist("ip.flags.rb")
  //   .metadata_blacklist("ip.flags.df")
  //   .metadata_blacklist("ip.flags.mf")
  //   .metadata_blacklist("ip.frag_offset")
  //   .metadata_blacklist("ip.ttl")
  //   .metadata_blacklist("ip.proto")
  //   .metadata_blacklist("ip.checksum")
  //   .metadata_blacklist("ip.checksum.status")
  //   .metadata_blacklist("ip.src")
  //   .metadata_blacklist("ip.src_host")
  //   .metadata_blacklist("ip.host")
  //   .metadata_blacklist("ip.dst")
  //   .metadata_blacklist("ip.dst_host")
  //   .metadata_blacklist("ip.host")
  //   .metadata_blacklist("frame.encap_type")
  //   .metadata_blacklist("frame.offset_shift")
  //   .metadata_blacklist("frame.time_delta")
  //   .metadata_blacklist("frame.time_delta_displayed")
  //   .metadata_blacklist("frame.time_relative")
  //   .metadata_blacklist("frame.number")
  //   .metadata_blacklist("frame.len")
  //   .metadata_blacklist("frame.cap_len")
  //   .metadata_blacklist("frame.marked")
  //   .metadata_blacklist("frame.ignored")
  //   .metadata_blacklist("frame.protocols")
  //   .metadata_blacklist("dns.a")
  //   .metadata_blacklist("dns.cname")
  //   .metadata_blacklist("dns.time")
  //   .metadata_blacklist("dns.opt")
  //   .metadata_blacklist("dns.opt.code")
  //   .metadata_blacklist("dns.opt.len")
  //   .metadata_blacklist("dns.opt.data")
  //   .metadata_blacklist("dns.opt.cookie.client")
  //   .metadata_blacklist("dns.opt.cookie.server")
  //   .metadata_blacklist("dns.ns")
  //   .metadata_blacklist("dns.nsec3.algo")
  //   .metadata_blacklist("dns.nsec3.flags")
  //   .metadata_blacklist("dns.nsec3.flags.opt_out")
  //   .metadata_blacklist("dns.nsec3.iterations")
  //   .metadata_blacklist("dns.nsec3.salt_length")
  //   .metadata_blacklist("dns.nsec3.salt_value")
  //   .metadata_blacklist("dns.nsec3.hash_length")
  //   .metadata_blacklist("dns.nsec3.hash_value")
  //   .metadata_blacklist("dns.nsec3.algo")
  //   .metadata_blacklist("dns.nsec3.flags")
  //   .metadata_blacklist("dns.nsec3.flags.opt_out")
  //   .metadata_blacklist("dns.nsec3.iterations")
  //   .metadata_blacklist("dns.nsec3.salt_length")
  //   .metadata_blacklist("dns.nsec3.salt_value")
  //   .metadata_blacklist("dns.nsec3.hash_length")
  //   .metadata_blacklist("dns.nsec3.hash_value")
  //   .metadata_blacklist("dns.aaaa")
  //   .metadata_blacklist("dns.ds.key_id")
  //   .metadata_blacklist("dns.ds.algorithm")
  //   .metadata_blacklist("dns.ds.digest_type")
  //   .metadata_blacklist("dns.ds.digest")
  //   .metadata_blacklist("dns.retransmit_response")
  //   .metadata_blacklist("dns.retransmit_response_in")
  //   .metadata_blacklist("http.chat")
  //   .metadata_blacklist("http.request.method")
  //   .metadata_blacklist("http.request.uri")
  //   .metadata_blacklist("http.request.version")
  //   .metadata_blacklist("http.host")
  //   .metadata_blacklist("http.request.line")
  //   .metadata_blacklist("http.request.line")
  //   .metadata_blacklist("http.request.line")
  //   .metadata_blacklist("http.request.line")
  //   .metadata_blacklist("http.request")
  //   .metadata_blacklist("http.request_number")
  //   .metadata_blacklist("http.response.version")
  //   .metadata_blacklist("http.response.code")
  //   .metadata_blacklist("http.response.code.desc")
  //   .metadata_blacklist("http.response.phrase")
  //   .metadata_blacklist("http.cache_control")
  //   .metadata_blacklist("http.date")
  //   .metadata_blacklist("http.location")
  //   .metadata_blacklist("http.server")
  //   .metadata_blacklist("http.response.line")
  //   .metadata_blacklist("http.response")
  //   .metadata_blacklist("http.response_number")
  //   .metadata_blacklist("http.time")
  //   .metadata_blacklist("http.request_in")
  //   .metadata_blacklist("http.response_for.uri")
  //   .display_filter("frame || ip || udp || dns || ntp || chargen || ssdp")
  //   .spawn()
  //   .unwrap_or_else(|e| panic!("Error starting tshark: {e}"));
  // add match

  // pcap_process(
  //   rtshark,
  //   &conn,
  //   &mut map_id,
  //   &mut map_attacks,
  //   &mut id_attack,
  // );

  Ok(())
}

// // The output is wrapped in a Result to allow matching on errors
// // Returns an Iterator to the Reader of the lines of the file.
// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where
//   P: AsRef<Path>,
// {
//   let file = File::open(filename)?;
//   Ok(io::BufReader::new(file).lines())
// }

// pub struct PcapAttack {
//   id: i32,
//   ip_vitima_cidr: Ipv4Cidr,
//   packets: Vec<pcap_packet::PcapPacket>,
//   timestamp_inicio: DateTime<FixedOffset>,
//   timestamp_fim: DateTime<FixedOffset>,
// }

fn get_current_working_dir() -> PathBuf {
  let working_dir = env::current_dir();
  let current_dir = match working_dir {
    Ok(workdir) => workdir,
    Err(error) => panic!("Problem reading current dir: {:?}", error),
  };
  return current_dir;
}

// pub fn pcap_process(
//   mut rtshark: RTShark,
//   conn: &Connection,
//   map_id: &mut HashMap<&str, i32>,
//   map_attacks: &mut HashMap<Ipv4Cidr, HashMap<i32, PcapAttack>>,
//   id_attack: &mut i32,
// ) {
//   // read packets until the end of the PCAP file

//   let mut pcap_packet = pcap_packet::PcapPacket::default(0);

//   // let rt =  rtshark.into();

//   // rtshark.read()

//   loop {
//     match rtshark.read() {
//       Ok(opt_packet) => match opt_packet {
//         Some(packet) => {
//           pcap_packet = pcap_packet::pcap_process_packet(&packet, map_id);
//           add_packet_to_attacks(conn, map_attacks, pcap_packet, id_attack);
//         }
//         None => {
//           println!("None packet")
//         }
//       },
//       Err(err) => {
//         print!("Error parsing TShark output: {}", err);
//         break;
//       }
//     }
//   }

//   // while let Some(packet) = rtshark.read().unwrap_or_else(|e| {
//   //   eprintln!("Error parsing TShark output: {e}");
//   //   None
//   // }) {
//   // packet.into()
//   // 5 pacotes
//   // intervalo de 1 minutos
//   // source IP (ip.src - vítima) do mesmo CIDR block e mesma porta destino UDP
//   // cidr utils
//   // println!("-----------------------");

//   // for layer in packet {
//   //   // println!("Layer name: {}", layer.name());
//   //   for metadata in layer {
//   //     println!("{}", metadata.name());
//   //   }
//   // }

//   // pcap_packet = pcap_packet::pcap_process_packet(&packet, map_id);
//   // add_packet_to_attacks(conn, map_attacks, pcap_packet, id_attack);

//   // packet.frame.timestamp
//   // TODO verificar a cada 10000 packets o que da pra limpar do hashmap...
//   // if pcap_packet.id % 1000 == 0 {

//   // }
//   // }

//   // call again at the end
// }

// // pub fn remove_old_attacks(
// //   cidr_udp_attack: &mut HashMap<Ipv4Cidr, HashMap<i32, PcapAttack>>,
// //   timestamp: DateTime<FixedOffset>,
// // ) {
// //   for ele in cidr_udp_attack {
// //     let (ipv4, udp_attack) = (ele.0, ele.1);
// //     for udp0_attack1 in udp_attack {
// //       let (udp_dest_port, attack) = (udp0_attack1.0, udp0_attack1.1);
// //       if !attack.old_attack(timestamp) {
// //         continue;
// //       }

// //       ele.1.remove(udp0_attack1.0)
// //     }
// //   }
// // }

// pub fn add_packet_to_attacks(
//   conn: &Connection,
//   cidr_udp_attack: &mut HashMap<Ipv4Cidr, HashMap<i32, PcapAttack>>,
//   packet: pcap_packet::PcapPacket,
//   id_attack: &mut i32,
// ) {
//   let cidr = packet.ip.vitima_cidr;

//   // cidr => udp => attack
//   match cidr_udp_attack.get_mut(&cidr) {
//     Some(udp_attack) => {
//       // cidr existe
//       // udp => attack
//       let udp_dest_port = packet.udp.destination_port;

//       match udp_attack.get_mut(&udp_dest_port) {
//         Some(attack) => {
//           // attack exists
//           // check time
//           if attack.same_attack(&packet) {
//             attack.add_packet(packet);
//             return;
//           }

//           // not same attack, should check if len(packets) > 5 to dbinsert or just replace by a new attack
//           if attack.packets.len() > 5 {
//             attack.db_insert(conn); // inserts db
//           }
//           // udp_attack.insert also updates
//           udp_attack.insert(udp_dest_port, new_empty_attack(packet, id_attack));
//           return;
//         }
//         None => {
//           // no attack
//           udp_attack.insert(udp_dest_port, new_empty_attack(packet, id_attack));
//           return;
//         }
//       }
//     }

//     None => {
//       // não tem o cidr,
//       let udp_attack = new_map_attack(packet, id_attack);
//       cidr_udp_attack.insert(cidr, udp_attack);
//       return;
//     }
//   }
// }

// pub fn new_map_attack(
//   packet: pcap_packet::PcapPacket,
//   id_attack: &mut i32,
// ) -> HashMap<i32, PcapAttack> {
//   let mut map_attack = HashMap::<i32, PcapAttack>::new();
//   map_attack.insert(
//     packet.udp.destination_port,
//     new_empty_attack(packet, id_attack),
//   );

//   return map_attack;
// }

// pub fn new_empty_attack(packet: pcap_packet::PcapPacket, id: &mut i32) -> PcapAttack {
//   let timestamp = packet.frame.timestamp;
//   *id += 1;

//   return PcapAttack {
//     id: *id,
//     ip_vitima_cidr: packet.ip.vitima_cidr,
//     packets: vec![packet],
//     timestamp_inicio: timestamp,
//     timestamp_fim: timestamp,
//   };
// }

// impl PcapAttack {
//   pub fn add_packet(&mut self, packet: pcap_packet::PcapPacket) {
//     self.timestamp_fim = packet.frame.timestamp;
//     self.packets.push(packet);
//   }

//   // same_attack chcks if the packet happened within 1 minute from the last attack
//   pub fn same_attack(&self, packet: &pcap_packet::PcapPacket) -> bool {
//     match self.timestamp_fim.checked_add_signed(Duration::minutes(1)) {
//       Some(fim_plus_1min) => {
//         return packet.frame.timestamp >= self.timestamp_fim
//           && packet.frame.timestamp <= fim_plus_1min
//       }
//       None => false,
//     }
//   }

//   // if the attack past 1min without new packages, can be removed
//   pub fn old_attack(&self, timestamp: DateTime<FixedOffset>) -> bool {
//     match self.timestamp_fim.checked_add_signed(Duration::minutes(1)) {
//       Some(fim_plus_1min) => return timestamp > fim_plus_1min,
//       None => false,
//     }
//   }

//   pub fn db_insert(&self, conn: &Connection) {
//     match conn.execute(
//       "INSERT INTO PCAP_ATTACK (id, ip_vitima_cidr, packets_per_attack, timestamp_inicio, timestamp_fim) values (?1, ?2, ?3, ?4, ?5)",
//       params![&self.id, &self.ip_vitima_cidr.to_string(), &self.packets.len(), &self.timestamp_inicio.to_string(), &self.timestamp_fim.to_string()],
//     ) {
//       Ok(_) => {
//         // println!("attack inserted");
//         self.insert_pcap_packets(conn);
//       }
//       Err(err) => {
//         println!("Problem inserting attack: {:?}", err)
//       }
//     }
//   }

//   fn insert_pcap_packets(&self, conn: &Connection) {
//     for packet in &self.packets {
//       packet.insert(conn);
//       match conn.execute(
//         "INSERT INTO PCAP_ATTACK_PACKET (attack_id, packet_id) values (?1, ?2)",
//         params![&self.id, &packet.id],
//       ) {
//         Ok(_) => {
//           // println!("attack inserted")
//         }
//         Err(err) => {
//           println!("Problem inserting attack : {:?}", err)
//         }
//       }
//     }
//   }
// }

// pub fn drop_tables(conn: &Connection) {
//   pcap_packet::db_drop_pcap_tables(&conn);
//   drop_table_attack(conn);
//   pcap_packet::db_create_pcap_tables(&conn);
// }

// pub fn create_tables(conn: &Connection) {
//   pcap_packet::db_create_pcap_tables(&conn);
//   create_table_attack(conn);
// }

// pub fn drop_table_attack(conn: &Connection) {
//   match conn.execute("DROP TABLE IF EXISTS PCAP_ATTACK", []) {
//     Ok(_) => {
//       println!("Table created!")
//     }
//     Err(err) => {
//       println!("Problem droping table PCAP_ATTACK: {:?}", err)
//     }
//   }

//   match conn.execute("DROP TABLE IF EXISTS PCAP_ATTACK_PACKET", []) {
//     Ok(_) => {
//       println!("Table created!")
//     }
//     Err(err) => {
//       println!("Problem droping table PCAP_ATTACK_PACKET: {:?}", err)
//     }
//   }
// }

// pub fn create_table_attack(conn: &Connection) {
//   match conn.execute(
//     "CREATE TABLE IF NOT EXISTS PCAP_ATTACK (
//       id INTEGER NOT NULL,
//       ip_vitima_cidr TEXT NOT NULL,
//       packets_per_attack INTEGER NOT NULL,
//       timestamp_inicio TEXT NOT NULL,
//       timestamp_fim TEXT NOT NULL
//      )",
//     [],
//   ) {
//     Ok(_) => {
//       println!("Table created!")
//     }
//     Err(err) => {
//       println!("Problem creating table ATTACK: {:?}", err)
//     }
//   }

//   // FOREIGN KEY(attack_id) REFERENCES PCAP_ATTACK(id),
//   // FOREIGN KEY(packet_id) REFERENCES PCAP_PACKET(id)

//   match conn.execute(
//     "CREATE TABLE IF NOT EXISTS PCAP_ATTACK_PACKET (
//       attack_id INTEGER NOT NULL,
//       packet_id INTEGER NOT NULL
//     )",
//     [],
//   ) {
//     Ok(_) => {
//       println!("Table created!")
//     }
//     Err(err) => {
//       println!("Problem creating table PCAP_ATTACK_PACKET: {:?}", err)
//     }
//   }
// }

// for dns_query in dns_message.queries {
//   match dns_query.qtype {
//     dnslogger::parse::DnsType::A => {
//       println!("DNS qtype {}", "A");
//     }
//     dnslogger::parse::DnsType::NS => {
//       println!("DNS qtype {}", "NS");
//     }
//     dnslogger::parse::DnsType::CNAME => {
//       println!("DNS qtype {}", "CNAME");
//     }
//     dnslogger::parse::DnsType::SOA => {
//       println!("DNS qtype {}", "SOA");
//     }
//     dnslogger::parse::DnsType::WKS => {
//       println!("DNS qtype {}", "WKS");
//     }
//     dnslogger::parse::DnsType::PTR => {
//       println!("DNS qtype {}", "PTR");
//     }
//     dnslogger::parse::DnsType::HINFO => {
//       println!("DNS qtype {}", "HINFO");
//     }
//     dnslogger::parse::DnsType::MX => {
//       println!("DNS qtype {}", "MX");
//     }
//     dnslogger::parse::DnsType::TXT => {
//       println!("DNS qtype {}", "TXT");
//     }
//     dnslogger::parse::DnsType::AXFR => {
//       println!("DNS qtype {}", "AXFR");
//     }
//     dnslogger::parse::DnsType::ALL => {
//       println!("DNS qtype {}", "ALL");
//     }
//     dnslogger::parse::DnsType::AAAA => {
//       println!("DNS qtype {}", "AAAA");
//     }
//     dnslogger::parse::DnsType::LOC => {
//       println!("DNS qtype {}", "LOC");
//     }
//     dnslogger::parse::DnsType::SPF => {
//       println!("DNS qtype {}", "SPF");
//     }
//     dnslogger::parse::DnsType::SRV => {
//       println!("DNS qtype {}", "SRV");
//     }
//     dnslogger::parse::DnsType::TKEY => {
//       println!("DNS qtype {}", "TKEY");
//     }
//     dnslogger::parse::DnsType::TSIG => {
//       println!("DNS qtype {}", "TSIG");
//     }
//     dnslogger::parse::DnsType::IXFR => {
//       println!("DNS qtype {}", "IXFR");
//     }
//     dnslogger::parse::DnsType::URI => {
//       println!("DNS qtype {}", "URI");
//     }
//     dnslogger::parse::DnsType::TA => {
//       println!("DNS qtype {}", "TA");
//     }
//     dnslogger::parse::DnsType::DLV => {
//       println!("DNS qtype {}", "DLV");
//     }
//     dnslogger::parse::DnsType::OPT => {
//       println!("DNS qtype {}", "OPT");
//     }
//     dnslogger::parse::DnsType::NSEC => {
//       println!("DNS qtype {}", "NSEC");
//     }
//     dnslogger::parse::DnsType::UnknownType(_) => {
//       println!("DNS qtype {}", "UnknownType");
//     }
//   }

// if udp.length > 0 && data.len() > 0 {
//   match dnslogger::parse::dns_message_parse(b.data, resolve_all_resource_records)
//   {
//     Ok(dns_message) => {
//       println!("DNS worked")

//         // match dns_query.name_chain.name {
//         //   Some(qname) => {
//         //     println!("DNS qname {}", qname);
//         //   },
//         //   None => {
//         //     print!("dns sem name_chain");
//         //   },
//         // }
//       // }
//       // println!("DNS ", dns_message.)
//     }
//     Err(err) => {
//       // println!("Err parse dns {}", err)
//     }
//   }
// }
