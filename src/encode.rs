pub fn usize_varint(_s: usize) -> Vec<u8> {
    Vec::new()
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
