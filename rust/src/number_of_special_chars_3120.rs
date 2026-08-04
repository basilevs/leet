use std::iter::zip;

pub fn number_of_special_chars(word: String) -> i32 {
    let mut lower = vec![false; 26];
    let mut upper = vec![false; lower.len()];
    for c in word.chars() {
        if ('a'..='z').contains(&c) {
            lower[usize::from(c as u8 - b'a')] = true;
        } else if ('A'..='Z').contains(&c) {
            upper[usize::from(c as u8 - b'A')] = true;
        }
    }
    zip(lower, upper).filter(|(a, b)| *a && *b).count() as i32
}

// Example 1:
// Input: word = "aaAbcBC"
// Output: 3
// Explanation:
// The special characters in word are 'a', 'b', and 'c'.
#[test]
fn official1() {
    assert_eq!(3, number_of_special_chars("aaAbcBC".to_string()));
}

// Example 2:
// Input: word = "abc"
// Output: 0
// Explanation:
// No character in word appears in uppercase.
#[test]
fn official2() {
    assert_eq!(0, number_of_special_chars("abc".to_string()));
}

// Example 3:
// Input: word = "abBCab"
// Output: 1
// Explanation:
// The only special character in word is 'b'.
#[test]
fn official3() {
    assert_eq!(1, number_of_special_chars("abBCab".to_string()));
}
