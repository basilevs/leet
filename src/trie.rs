use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct Trie<K: Eq + Hash + std::fmt::Debug, T: std::fmt::Debug> {
    children: HashMap<K, TrieNode<K, T>>,
}

#[derive(Debug)]
struct TrieNode<K: Eq + Hash + std::fmt::Debug, T: std::fmt::Debug> {
    value: Option<T>,
    children: HashMap<K, TrieNode<K, T>>,
}

impl<K: Eq + Hash + std::fmt::Debug, T: std::fmt::Debug> Trie<K, T> {
    pub fn new() -> Self {
        Self { children: HashMap::new() }
    }

    pub fn insert<I: Iterator<Item = K>>(&mut self, mut key: I, value: T) {
        let mut node = self.children.entry(key.next().expect("Key must have at least one element")).or_insert_with(|| TrieNode {
            value: None,
            children: HashMap::new(),
        });
        for k in key {
            node = node.children.entry(k).or_insert_with(|| TrieNode {
                value: None,
                children: HashMap::new(),
            });
        }
        node.value = Some(value);
    }

    pub fn get<I: Iterator<Item = K>, V, F: Fn(&T) -> Option<V>>(&self, mut key: I, f: F) -> Option<V> {
         let Some(mut node) = self.children.get(&(key.next().expect("Key must have at least one element"))) else {
            return None;
        };
        if let Some(v) = node.value.as_ref().and_then(|v| f(v)) {
            return Some(v);
        }
        for k in key {
            let Some(n) = node.children.get(&k) else {
                return None
            };
            node = n;
            if let Some(v) = node.value.as_ref().and_then(|v| f(v)) {
                return Some(v);
            }
        }
        None
    }
}

#[test]
fn trie() {
    let mut trie = Trie::new();
    trie.insert("abc".chars(), 1);
    trie.insert("abd".chars(), 2);
    trie.insert("bcd".chars(), 3 );
    assert_eq!(Some(1), trie.get("abc".chars(), |&v| Some(v)));
    assert_eq!(Some(2), trie.get("abd".chars(), |&v| Some(v)));
    assert_eq!(Some(3), trie.get("bcd".chars(), |&v| Some(v)));
    assert_eq!(None, trie.get("ab".chars(), |&v| Some(v)));
    assert_eq!(Some(1), trie.get("abcd".chars(), |&v| Some(v)));
    assert_eq!(None, trie.get("abe".chars(), |&v| Some(v)));
}