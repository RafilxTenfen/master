
use std::ascii::escape_default;
use std::str;

pub fn show(bs: &[u8]) -> String {
  let mut visible = String::new();
  for &b in bs {
    let part: Vec<u8> = escape_default(b).collect();
    visible.push_str(str::from_utf8(&part).unwrap());
  }
  visible
}
