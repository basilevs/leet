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

    pub fn walk(&self, key: impl IntoIterator<Item = K>) -> impl Iterator<Item = &T> {
        let mut node = Some(&self.root);
        let mut keys = key.into_iter();
        std::iter::from_fn(move || loop {
            let n = node?;
            let k = keys.next()?;
            match n.children.get(&k) {
                Some(child) => {
                    node = Some(child);
                    if let Some(v) = child.value.as_ref() {
                        return Some(v);
                    }
                }
                None => {
                    node = None;
                    return None;
                }
            }
        })
    }
}

#[test]
fn trie() {
    let mut trie = Trie::new();
    trie.insert("abc".chars(), 1);
    trie.insert("abd".chars(), 2);
    trie.insert("bcd".chars(), 3);
    assert_eq!(Some(&1), trie.walk("abc".chars()).next());
    assert_eq!(Some(&2), trie.walk("abd".chars()).next());
    assert_eq!(Some(&3), trie.walk("bcd".chars()).next());
    assert_eq!(None, trie.walk("ab".chars()).next());
    assert_eq!(Some(&1), trie.walk("abcd".chars()).next());
    assert_eq!(None, trie.walk("abe".chars()).next());

    // Multiple values along a path
    trie.insert("a".chars(), 10);
    trie.insert("ab".chars(), 20);
    let vals: Vec<_> = trie.walk("abcd".chars()).collect();
    assert_eq!(vec![&10, &20, &1], vals);

    // Iterator terminates after dead end
    let mut walk = trie.walk("abe".chars());
    assert_eq!(Some(&10), walk.next()); // "a" has value
    assert_eq!(Some(&20), walk.next()); // "ab" has value
    assert_eq!(None, walk.next()); // "e" not found - dead end
    assert_eq!(None, walk.next()); // stays terminated

    // Iterator terminates after keys exhausted
    let mut walk = trie.walk("ab".chars());
    assert_eq!(Some(&10), walk.next());
    assert_eq!(Some(&20), walk.next());
    assert_eq!(None, walk.next()); // no more keys
    assert_eq!(None, walk.next()); // stays terminated
}