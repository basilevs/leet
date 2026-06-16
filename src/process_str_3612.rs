pub fn process_str(s: String) -> String {
    let mut result:Vec<u8> = Vec::new();
    let mut buffer:Vec<u8> = Vec::new();
    for b in s.into_bytes() {
        match b {
            b'*' => {result.pop();},
            b'#' => {
                buffer.clear();
                buffer.extend_from_slice(&result);
                result.extend_from_slice(&buffer);
            },
            b'%' => result.reverse(),
            _ => result.push(b),
        }
    }
    String::from_utf8(result).unwrap()
}
