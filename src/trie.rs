use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct Trie<K: Eq + Hash, T> {
    children: HashMap<K, TrieNode<K, T>>,
}

#[derive(Debug)]
struct TrieNode<K: Eq + Hash, T> {
    value: Option<T>,
    children: HashMap<K, TrieNode<K, T>>,
}

impl<K: Eq + Hash, T: Default> Trie<K, T> {
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

    pub fn get<I: Iterator<Item = K>>(&self, mut key: I) -> Option<&T> {
         let Some(mut node) = self.children.get(&(key.next().expect("Key must have at least one element"))) else {
            return None;
        };
        for k in key {
            let Some(n) = node.children.get(&k) else {
                 return None
            };
            node = n;
        }
        node.value.as_ref()
    }
}

#[test]
fn trie() {
    let mut trie = Trie::new();
    trie.insert("abc".chars(), 1);
    trie.insert("abd".chars(), 2);
    trie.insert("bcd".chars(), 3 );
    assert_eq!(Some(&1), trie.get("abc".chars()));
    assert_eq!(Some(&2), trie.get("abd".chars()));
    assert_eq!(Some(&3), trie.get("bcd".chars()));
    assert_eq!(None, trie.get("ab".chars()));
    assert_eq!(None, trie.get("abcd".chars()));
    assert_eq!(None, trie.get("abe".chars()));
}