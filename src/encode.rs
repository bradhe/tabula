pub fn uint64_varint(s: i64) -> Vec<u8> {
  let mut res = Vec::new();
  let mut val = s;

  while val >= 0x80 {
    let b: u8 = ((val & 0x7F) as u8) | 0x80;
    res.push(b);

    val = val >> 7;
  }

  res
}

pub fn usize_varint(s: usize) -> Vec<u8> {
  uint64_varint(s as i64)
}

pub fn string(s: &str) -> Vec<u8> {
    let mut res = Vec::new();
    res.extend(usize_varint(s.len()));
    res.extend(s.as_bytes());
    res
}

pub fn number(_f: &f64) -> Vec<u8> {
    Vec::new()
}
