use std::collections::VecDeque;

use crate::array_trie::ArrayTrie;

#[derive(Debug)]
pub struct StreamChecker {
    queries: ArrayTrie<26, bool>,
    text_tail: VecDeque<u8>,
}

impl StreamChecker {

    pub fn new(words: Vec<String>) -> Self {
        let max_len = words.iter().map(String::len).max().unwrap();
        let text_tail = VecDeque::with_capacity(max_len);
        let mut queries = ArrayTrie::new();
        for word in words {
            queries.insert(word.bytes().rev().map(|b| b - b'a'), true);
        }
        Self { queries, text_tail }
    }

    pub fn query(&mut self, letter: char) -> bool {
        if self.text_tail.len() >= self.text_tail.capacity() {
            self.text_tail.pop_back();
        }
        self.text_tail.push_front(letter as u8 - b'a');
        self.queries.walk(self.text_tail.iter().copied()).any(|&v| v)
    }
}

#[test]
fn official1_() {
    let mut stream_checker = StreamChecker::new(vec!["cd".to_string(), "f".to_string(), "kl".to_string()]);
    assert_eq!(false, stream_checker.query('a'));
    assert_eq!(false, stream_checker.query('b'));
    assert_eq!(false, stream_checker.query('c'));
    assert_eq!(true, stream_checker.query('d'));
    assert_eq!(false, stream_checker.query('e'));
    // dbg!(&stream_checker);
    assert_eq!(true, stream_checker.query('f'));
    assert_eq!(false, stream_checker.query('g'));
    assert_eq!(false, stream_checker.query('h'));
    assert_eq!(false, stream_checker.query('i'));
    assert_eq!(false, stream_checker.query('j'));
    assert_eq!(false, stream_checker.query('k'));
    assert_eq!(true, stream_checker.query('l'));
}

// [[["abaa","abaab","aabbb","bab","ab"]],["a"],["a"],["b"],["b"],["b"],["a"],["a"],["b"],["b"],["a"],["a"],["a"],["a"],["b"],["a"],["b"],["b"],["b"],["a"],["b"],["b"],["b"],["a"],["a"],["a"],["a"],["a"],["b"],["a"],["b"],["b"],["b"],["a"],["a"],["b"],["b"],["b"],["a"],["b"],["a"]]
// [null,false,false,true,false,true,false,false,true,false,false,false,false,false,true,false,true,false,false,false,true,false,false,false,false,false,false,false,true,false,true,false,false,false,false,true,false,true,false,true,false]
#[test]
fn official17() {
    let mut stream_checker = StreamChecker::new(vec!["abaa","abaab","aabbb","bab","ab"].into_iter().map(String::from).collect());
    let input = [["a"],["a"],["b"],["b"],["b"],["a"],["a"],["b"],["b"],["a"],["a"],["a"],["a"],["b"],["a"],["b"],["b"],["b"],["a"],["b"],["b"],["b"],["a"],["a"],["a"],["a"],["a"],["b"],["a"],["b"],["b"],["b"],["a"],["a"],["b"],["b"],["b"],["a"],["b"],["a"]];
    let expected = [false,false,true,false,true,false,false,true,false,false,false,false,false,true,false,true,false,false,false,true,false,false,false,false,false,false,false,true,false,true,false,false,false,false,true,false,true,false,true,false];
    for (input, expected) in input.into_iter().zip(expected) {
        // dbg!(input, expected);
        assert_eq!(expected, stream_checker.query(input[0].chars().next().unwrap()));
    }

}

#[test]
fn single_letter_word_matches_immediately() {
    let mut stream_checker = StreamChecker::new(vec!["a".to_string()]);
    assert!(!stream_checker.query('b'));
    assert!(stream_checker.query('a'));
    assert!(stream_checker.query('a'));
}

#[test]
fn overlapping_suffixes() {
    let mut stream_checker = StreamChecker::new(vec![
        "abc".to_string(),
        "bc".to_string(),
        "c".to_string(),
    ]);
    assert!(!stream_checker.query('a'));
    assert!(!stream_checker.query('b'));
    // dbg!(&stream_checker);
    assert!(stream_checker.query('c'));
}

#[test]
fn duplicate_words_do_not_change_behavior() {
    let mut stream_checker = StreamChecker::new(vec![
        "ab".to_string(),
        "ab".to_string(),
        "b".to_string(),
    ]);
    assert!(!stream_checker.query('a'));
    assert!(stream_checker.query('b'));
}

#[test]
fn respects_max_word_length_window() {
    let mut stream_checker = StreamChecker::new(vec!["abc".to_string()]);
    assert!(!stream_checker.query('a'));
    assert!(!stream_checker.query('b'));
    assert!(stream_checker.query('c'));
    assert!(!stream_checker.query('d'));
    assert!(!stream_checker.query('e'));
    assert!(!stream_checker.query('f'));
}

#[test]
fn supports_length_200_word() {
    let long_word = "a".repeat(200);
    let mut stream_checker = StreamChecker::new(vec![long_word]);
    for _ in 0..199 {
        assert!(!stream_checker.query('a'));
    }
    assert!(stream_checker.query('a'));
}
