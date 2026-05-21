use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct Trie<K: Eq + Hash + std::fmt::Debug, T: std::fmt::Debug> {
    root: TrieNode<K, T>,
}

#[derive(Debug)]
struct TrieNode<K: Eq + Hash + std::fmt::Debug, T: std::fmt::Debug> {
    value: Option<T>,
    children: HashMap<K, TrieNode<K, T>>,
}

impl<K: Eq + Hash + std::fmt::Debug, T: std::fmt::Debug> Trie<K, T> {
    pub fn new() -> Self {
        Self { root: TrieNode { value: None, children: HashMap::new() } }
    }

    pub fn insert(&mut self, key: impl IntoIterator<Item = K>, value: T) {
        let mut node = &mut self.root;
        for k in key {
            node = node.children.entry(k).or_insert_with(|| TrieNode {
                value: None,
                children: HashMap::new(),
            });
        }
        node.value = Some(value);
    }

    pub fn walk<V>(&self, key: impl IntoIterator<Item = K>, f: impl Fn(&T) -> Option<V>) -> Option<V> {
        let mut node = &self.root;
        for k in key {
            node = node.children.get(&k)?;
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
    assert_eq!(Some(1), trie.walk("abc".chars(), |&v| Some(v)));
    assert_eq!(Some(2), trie.walk("abd".chars(), |&v| Some(v)));
    assert_eq!(Some(3), trie.walk("bcd".chars(), |&v| Some(v)));
    assert_eq!(None, trie.walk("ab".chars(), |&v| Some(v)));
    assert_eq!(Some(1), trie.walk("abcd".chars(), |&v| Some(v)));
    assert_eq!(None, trie.walk("abe".chars(), |&v| Some(v)));
}