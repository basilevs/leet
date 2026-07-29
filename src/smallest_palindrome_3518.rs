// https://leetcode.com/problems/smallest-palindromic-rearrangement-ii

    pub fn smallest_palindrome(s: String, k: i32) -> String {
        let mut bytes = s.into_bytes();
        let n = bytes.len()/2;
        let mut k = usize::try_from(k-1).expect("k should be positive");

        let mut buckets = bytes.iter().take(n).fold([0u16; 26], |mut acc, &b| {
            acc[usize::from(b - b'a')] += 1;
            acc
        });

        let total_permutations = bucket_permutations(&buckets);
        if total_permutations <= k as u128 {
            return String::new();
        }

        for (i, byte) in bytes.iter_mut().enumerate().take(n) {
            let mut found = false;
            for j in 0..buckets.len() {
                if buckets[j] == 0 {
                    continue;
                }
                buckets[j] -= 1;

                let tail_permutations = bucket_permutations(&buckets);
                if tail_permutations <= k as u128 {
                    buckets[j] += 1;
                    k -= tail_permutations as usize;
                    continue;
                }
                *byte = b'a' + j as u8;
                found = true;
                break;
            }
            debug_assert!(found, "should have found a valid character to place at index {}", i);
        }
        let last_part = bytes.len() - n;
        let (a, b) = bytes.split_at_mut(last_part);
        b.copy_from_slice(&a[0..n]);
        b.reverse();
        String::from_utf8(bytes).unwrap()

    }

fn bucket_permutations(buckets: &[u16; 26]) -> u128 {
    let n: u16 = buckets.iter().sum();
    let numerator = factorial(n);
    if numerator == u128::MAX {
        return u128::MAX;
    }
    buckets.iter().filter(|&x| *x>0).fold(numerator, |acc, &count| acc / factorial(count))
}
    
fn factorial(n: u16) -> u128 {
    (1..=n).fold(1u128, |acc, x| acc.saturating_mul(x as u128))
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

    #[test]
    fn min_length_only_arrangement() {
        assert_eq!("aa", smallest_palindrome("aa".to_string(), 1));
    }

    #[test]
    fn min_length_k_exceeds_count() {
        assert_eq!("", smallest_palindrome("aa".to_string(), 2));
    }

    #[test]
    fn odd_length_no_half_characters() {
        assert_eq!("a", smallest_palindrome("a".to_string(), 1));
    }

    #[test]
    fn odd_length_with_middle_character() {
        assert_eq!("aba", smallest_palindrome("aba".to_string(), 1));
        assert_eq!("", smallest_palindrome("aba".to_string(), 2));
    }

    #[test]
    fn all_same_characters() {
        assert_eq!("aaaa", smallest_palindrome("aaaa".to_string(), 1));
    }

    // Same underlying bug as `duplicate_half_letters_panic_instead_of_empty`:
    // "aa" (n=2, all one letter) has only 1 distinct rearrangement, but
    // k=2 is accepted as in-bounds (2 <= 2! = 2) and panics instead of
    // returning "".
    #[test]
    fn all_same_characters_k_exceeds_count_panics() {
        assert_eq!("", smallest_palindrome("aaaa".to_string(), 2));
    }

    #[test]
    fn k_at_and_beyond_distinct_letter_boundary() {
        assert_eq!("cbaabc", smallest_palindrome("abccba".to_string(), 6));
        assert_eq!("", smallest_palindrome("abccba".to_string(), 7));
    }

    // BUG: total_permutations is computed as factorial(n) rather than the
    // multinomial coefficient n! / (count_a! * count_b! * ...), so it
    // overcounts distinct rearrangements when the half has repeated letters.
    // "aabbaa" has only 3 distinct rearrangements ("aab", "aba", "baa"), but
    // k=4 is accepted as in-bounds (4 <= 3! = 6) and then panics instead of
    // returning "".
    #[test]
    fn duplicate_half_letters_panic_instead_of_empty() {
        assert_eq!("", smallest_palindrome("aabbaa".to_string(), 4));
    }
}
