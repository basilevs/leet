// https://leetcode.com/problems/number-of-strings-that-appear-as-substrings-in-word

use std::{iter::successors};

use crate::modint::ModInt;

pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
    // https://cp-algorithms.com/string/rabin-karp.html

    let n = patterns.iter().chain([&word]).map(|s| s.len()).max().unwrap();

    let starting_count = patterns.len();

    let p_pow = successors(Some(ModInt::ONE), |x| Some(*x * ModInt::from(31)))
        .take(n + 1)
        .collect::<Vec<_>>();

    let word_prefix_hashes = [ModInt::ZERO].into_iter().chain(word
        .as_bytes()
        .iter()
        .copied()
        .map(ModInt::from)
        .enumerate()
        .scan(ModInt::ZERO, |acc, (i, c)| {
            *acc +=  p_pow[i] * c;
            Some(*acc)
        }))
        .collect::<Vec<_>>();

    let mut pattern_hashes = patterns
        .into_iter()
        .map(|s| {
            let bs = s.as_bytes();
            (   
                bs.len(), 
                bs
                .iter()
                .copied()
                .map(ModInt::from)
                .enumerate()
                .fold(ModInt::ZERO, |acc, (i, c)| acc + p_pow[i] * c)
            )
        })
        .collect::<Vec<_>>();
    
    // dbg!(&p_pow, &word_prefix_hashes, &pattern_hashes);
    for i in 0..word_prefix_hashes.len() {
        pattern_hashes.retain(|&(pattern_length, pattern_hash)|{
            if i + pattern_length >= word_prefix_hashes.len() {
                true
            } else {
                let substring_hash = word_prefix_hashes[i + pattern_length] - word_prefix_hashes[i];
                let pattern_hash = pattern_hash * p_pow[i];
                substring_hash != pattern_hash
            }
        });
    }

    i32::try_from(starting_count - pattern_hashes.len()).unwrap()

}

#[cfg(test)]
mod tests {
use super::num_of_strings;

fn to_vector(strings: &[&str]) -> Vec<String> {
    strings.iter().map(|s| s.to_string()).collect()
}

// Example 1:
// Input: patterns = ["a","abc","bc","d"], word = "abc"
// Output: 3
// Explanation:
// - "a" appears as a substring in "abc".
// - "abc" appears as a substring in "abc".
// - "bc" appears as a substring in "abc".
// - "d" does not appear as a substring in "abc".
// 3 of the strings in patterns appear as a substring in word.
#[test]
fn official1() {
    assert_eq!(3, num_of_strings(to_vector(&["a", "abc", "bc", "d"]),"abc".to_string()));
}

// Example 2:
// Input: patterns = ["a","b","c"], word = "aaaaabbbbb"
// Output: 2
// Explanation:
// - "a" appears as a substring in "aaaaabbbbb".
// - "b" appears as a substring in "aaaaabbbbb".
// - "c" does not appear as a substring in "aaaaabbbbb".
// 2 of the strings in patterns appear as a substring in word.
#[test]
fn official2() {
    assert_eq!(2, num_of_strings(to_vector(&["a", "b", "c"]),"aaaaabbbbb".to_string()));
}

// Example 3:
// Input: patterns = ["a","a","a"], word = "ab"
// Output: 3
// Explanation: Each of the patterns appears as a substring in word "ab".
#[test]
fn official3() {
    assert_eq!(3, num_of_strings(to_vector(&["a", "a", "a"]),"ab".to_string()));
}
 
}