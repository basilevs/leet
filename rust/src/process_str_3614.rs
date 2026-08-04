// https://leetcode.com/problems/process-string-with-special-operations-ii

pub fn process_str(s: String, k: i64) -> char {
    let mut len = 0_u64;
    let mut k = u64::try_from(k).unwrap();
    for b in s.bytes() {
        match b {
            b'*' => {
                len = len.saturating_sub(1);
            }
            b'#' => {
                len *= 2;
            }
            b'%' => {}
            _ => len += 1,
        }
    }

    if k >= len {
        return '.';
    }

    for b in s.bytes().rev() {
        match b {
            b'*' => {
                len += 1;
            }
            b'#' => {
                debug_assert!(len % 2 == 0);
                len /= 2;
                k %= len;
            }
            b'%' => {
                k = len - 1 - k;
            }
            c => {
                len -= 1;
                if len == k {
                    return char::from(c);
                }
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official1() {
        assert_eq!(process_str("a#b%*".to_string(), 1), 'a');
    }

    #[test]
    fn official2() {
        assert_eq!(process_str("cd%#*#".to_string(), 3), 'd');
    }

    #[test]
    fn official3() {
        assert_eq!(process_str("z*#".to_string(), 0), '.');
    }
}
