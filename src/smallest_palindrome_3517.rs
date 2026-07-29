// https://leetcode.com/problems/smallest-palindromic-rearrangement-i

    pub fn smallest_palindrome(s: String) -> String {
        debug_assert!(s.chars().zip(s.chars().rev()).all(|(a, b)| a == b));

        let mut vec = s.into_bytes();
        let n = vec.len();
        let first_part = n / 2;
        let last_part = n - first_part;
        let (a, b) = vec.split_at_mut(last_part);
        // bucket sort would be better!
        a[0..first_part].sort_unstable();
        b.copy_from_slice(&a[..first_part]);
        b.reverse();
        String::from_utf8(vec).unwrap()
    }

#[cfg(test)]
mod tests {
    use super::smallest_palindrome;

    #[test]
    fn official1() {
        assert_eq!("z", smallest_palindrome("z".to_string()));
    }

    #[test]
    fn official2() {
        assert_eq!("abbba", smallest_palindrome("babab".to_string()));
    }

    #[test]
    fn official3() {
        assert_eq!("acddca", smallest_palindrome("daccad".to_string()));
    }
}
