// https://leetcode.com/problems/smallest-palindromic-rearrangement-ii

    pub fn smallest_palindrome(s: String, k: i32) -> String {
        let mut bytes = s.into_bytes();
        let n = bytes.len()/2;
        let mut k = usize::try_from(k-1).expect("k should be positive");
        let total_permutations = usize::try_from(factorial(n as i32).unwrap_or(i32::MAX)).expect("total_permutations should be positive");
        
        if total_permutations <= k {
            return String::new();
        }

        let mut buckets = bytes.iter().take(n).fold([0; 26], |mut acc, &b| {
            acc[usize::from(b - b'a')] += 1;
            acc
        });

        for (i, byte) in bytes.iter_mut().enumerate().take(n) {
            let tail_permutations = factorial((n - i - 1) as i32).unwrap_or(i32::MAX) as usize;
            let mut selection = k / tail_permutations;
            dbg!(i, k, tail_permutations, selection, buckets);
            debug_assert!(selection < 26);
            k -= selection * tail_permutations;
            let mut found = false;
            for (j, bucket) in buckets.iter_mut().enumerate() {
                if *bucket > 0 {
                    if selection == 0 {
                        *byte = b'a' + j as u8;
                        *bucket -= 1;
                        found = true;
                        break;
                    }
                    selection -= 1;
                }
            }
            debug_assert!(found, "should have found a valid character to place at index {}", i);
            debug_assert_eq!(selection, 0, "selection should be zero after the loop");
        }
        let last_part = bytes.len() - n;
        let (a, b) = bytes.split_at_mut(last_part);
        b.copy_from_slice(&a[0..n]);
        b.reverse();
        String::from_utf8(bytes).unwrap()

    }

fn factorial(n: i32) -> Option<i32> {
    (1..=n).try_fold(1i32, |acc, x| acc.checked_mul(x))
}

#[cfg(test)]
mod tests {
    use super::smallest_palindrome;

    #[test]
    fn official1() {
        assert_eq!("baab", smallest_palindrome("abba".to_string(), 2));
    }

    #[test]
    fn official2() {
        assert_eq!("", smallest_palindrome("aa".to_string(), 2));
    }

    #[test]
    fn official3() {
        assert_eq!("abcba", smallest_palindrome("bacab".to_string(), 1));
    }
}
