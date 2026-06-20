// https://leetcode.com/problems/process-string-with-special-operations-i/
pub fn process_str(s: String) -> String {
    let dups = s.bytes().filter(|x| *x == b'#').count();
    let mut result: Vec<u8> = Vec::with_capacity(s.len() * dups);
    for b in s.into_bytes() {
        match b {
            b'*' => {
                result.pop();
            }
            b'#' => {
                let len = result.len();
                result.extend_from_within(..len);
            }
            b'%' => result.reverse(),
            _ => result.push(b),
        }
    }
    String::from_utf8(result).unwrap()
}
