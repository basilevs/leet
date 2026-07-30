// https://leetcode.com/problems/smallest-palindromic-rearrangement-ii

    pub fn smallest_palindrome(s: String, k: i32) -> String {
        let mut bytes = s.into_bytes();
        let n = bytes.len()/2;
        let mut k = u64::from(u32::try_from(k-1).expect("k should be positive"));

        let mut buckets = bytes.iter().take(n).fold([0u32; 26], |mut acc, &b| {
            acc[usize::from(b - b'a')] += 1;
            acc
        });

        let total_permutations = bucket_permutations(&buckets);
        if total_permutations <= k {
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
                if tail_permutations <= k {
                    buckets[j] += 1;
                    k -= tail_permutations;
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

/// Stands in for "more distinct arrangements than any valid `k` could ever
/// select" (`k` comes from an `i32`, so this comfortably dominates every
/// real `k`).
const MANY: u64 = u64::MAX;

/// Number of distinct arrangements of the multiset described by `buckets`,
/// i.e. `n! / (count_1! * count_2! * ...)`, saturating at `MANY` once the
/// value provably exceeds anything `k` could hold.
///
/// A raw `n!` numerator overflows any fixed-width integer for even modest
/// `n` (e.g. `n >= 35` overflows `u128`) long before it's divided back down
/// to size, which silently discards precision. Instead this merges bucket
/// counts one at a time via the standard multiplicative nCr identity
/// (`value *= (total + x); value /= x`), which only ever holds actual,
/// exact intermediate binomial coefficients. The single largest bucket is
/// merged for free (a lone group has exactly one arrangement), and merging
/// stops the moment `value` exceeds `MANY`. Because true multinomial values
/// grow combinatorially fast, any configuration whose value would exceed
/// `MANY` is detected within a handful of merge steps, so this is O(1)
/// amortized regardless of `n`.
fn bucket_permutations(buckets: &[u32; 26]) -> u64 {
    let mut counts = *buckets;
    counts.sort_unstable_by(|a, b| b.cmp(a));

    let mut total = u128::from(counts[0]);
    let mut value = 1u128;
    for &count in &counts[1..] {
        for x in 1..=u128::from(count) {
            value = value * (total + x) / x;
            if value >= u128::from(MANY) {
                return MANY;
            }
        }
        total += u128::from(count);
    }
    value as u64
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

    #[test]
    fn all_same_characters_k_exceeds_count() {
        assert_eq!("", smallest_palindrome("aaaa".to_string(), 2));
    }

    #[test]
    fn k_at_and_beyond_distinct_letter_boundary() {
        assert_eq!("cbaabc", smallest_palindrome("abccba".to_string(), 6));
        assert_eq!("", smallest_palindrome("abccba".to_string(), 7));
    }

    #[test]
    fn duplicate_half_letters_k_exceeds_count() {
        assert_eq!("", smallest_palindrome("aabbaa".to_string(), 4));
    }

    // Regression test: a half long enough that `n!` alone overflows u128
    // (n >= 35) used to make `bucket_permutations` silently report "more
    // permutations than could ever exist" instead of the true count (1
    // here, since every character is identical), so an out-of-range k was
    // wrongly accepted instead of yielding "".
    #[test]
    fn large_uniform_half_k_exceeds_count() {
        let half = "a".repeat(40);
        let s = format!("{half}{half}");
        assert_eq!(s, smallest_palindrome(s.clone(), 1));
        assert_eq!("", smallest_palindrome(s, 2));
    }

    // Regression test: with a naive per-position factorial recomputation,
    // a large half with many distinct letters made this take multiple
    // seconds. It should stay fast even at this size.
    #[test]
    fn large_diverse_half_stays_fast() {
        let mut half: Vec<u8> = Vec::new();
        for _ in 0..2000 {
            half.extend(b'a'..=b'z');
        }
        half.sort_unstable();
        let half_s = String::from_utf8(half).unwrap();
        let rev: String = half_s.chars().rev().collect();
        let s = format!("{half_s}{rev}");

        let start = std::time::Instant::now();
        let result = smallest_palindrome(s, 1_000_000_000);
        assert!(!result.is_empty());
        assert!(
            start.elapsed().as_secs() < 1,
            "took {:?}, expected sub-second",
            start.elapsed()
        );
    }
}
