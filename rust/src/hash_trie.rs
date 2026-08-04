use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct HashTrie<K: Eq + Hash + std::fmt::Debug, T: std::fmt::Debug> {
    root: TrieNode<K, T>,
}

#[derive(Debug)]
struct TrieNode<K: Eq + Hash + std::fmt::Debug, T: std::fmt::Debug> {
    value: Option<T>,
    children: HashMap<K, TrieNode<K, T>>,
}

impl<K: Eq + Hash + std::fmt::Debug, T: std::fmt::Debug> Default for TrieNode<K, T> {
    fn default() -> Self {
        Self { value: None, children: HashMap::new() }
    }
}

impl<K: Eq + Hash + std::fmt::Debug, T: std::fmt::Debug> HashTrie<K, T> {
    pub fn new() -> Self {
        Self { root: TrieNode::default() }
    }

    pub fn insert(&mut self, key: impl IntoIterator<Item = K>, value: T) {
        key.into_iter().fold(&mut self.root, |node, k| {
            node.children.entry(k).or_default()
        }).value = Some(value);
    }

    pub fn walk(&self, key: impl IntoIterator<Item = K>) -> impl Iterator<Item = &T> {
        key.into_iter().scan(Some(&self.root), |node, k| {
            *node = (*node)?.children.get(&k);
            Some((*node)?.value.as_ref())
        }).flatten()
    }
}

#[test]
fn trie() {
    let mut trie = HashTrie::new();
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