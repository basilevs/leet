use std::collections::VecDeque;

pub struct StreamChecker {
    hash_length: Vec<(u64, usize)>,
    hash_tail: VecDeque<u64>,
}

impl StreamChecker {

    pub fn new(words: Vec<String>) -> Self {
        for i in words.iter() {
            assert!(i.len() < POWERS.len());
        }
        let hash_tail = VecDeque::with_capacity(words.iter().map(String::len).max().unwrap() + 1);
        let hash_length = words.iter().map(|w| (hash(&w), w.len())).collect();
        dbg!(&hash_length);
        Self { hash_length, hash_tail, }
    }
    
    pub fn query(&mut self, letter: char) -> bool {
        let mut hash = self.hash_tail.front().copied().unwrap_or(0);
        hash = (hash * 31 + letter as u64) % PRIME;
        if self.hash_tail.len() >= self.hash_tail.capacity() {
            self.hash_tail.pop_back();
        }
        self.hash_tail.push_front(hash);
        for &(word_hash, word_length) in &self.hash_length {
            let old_hash = if word_length == self.hash_tail.len() {
                0
            } else {
                if let Some(&h) = self.hash_tail.get(word_length) {
                    h
                } else {
                    continue;
                }
            };

            // Rabin-Karp rolling hash
            let tail_hash = (hash + PRIME - (old_hash * POWERS[word_length]) % PRIME) % PRIME;
            dbg!(hash, word_hash, word_length, self.hash_tail.len(), tail_hash, old_hash);
            if word_hash == tail_hash {
                return true;
            }
        }
        false
    }
}

const POWERS: [u64; 201] = {
    let mut powers = [0; 201];
    powers[0] = 1;
    let mut i = 1;
    while i < powers.len() {
        powers[i] = (powers[i-1] * 31) % PRIME;
        i += 1;
    }
    powers
};

const PRIME: u64 = 1000000009;
fn hash(input: &str) -> u64 {
    let mut result = 0;
    for i in input.chars() {
        result = (result * 31 + i as u64) % PRIME;
    }
    result
}

#[test]
fn official1() {
    let mut stream_checker = StreamChecker::new(vec!["cd".to_string(), "f".to_string(), "kl".to_string()]);
    assert_eq!(false, stream_checker.query('a'));
    assert_eq!(false, stream_checker.query('b'));
    assert_eq!(false, stream_checker.query('c'));
    assert_eq!(true, stream_checker.query('d'));
    assert_eq!(false, stream_checker.query('e'));
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
        dbg!(input, expected);
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
