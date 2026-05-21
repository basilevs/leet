use std::num::NonZeroU32;

#[derive(Debug)]
pub struct ArrayTrie<const N: usize, T: std::fmt::Debug> {
    nodes: Vec<ArrayTrieNode<N, T>>,
}

#[derive(Debug)]
struct ArrayTrieNode<const N: usize, T: std::fmt::Debug> {
    value: Option<T>,
    children: [Option<NonZeroU32>; N],
}

impl<const N: usize, T: std::fmt::Debug> Default for ArrayTrieNode<N, T> {
    fn default() -> Self {
        Self { value: None, children: [None; N] }
    }
}

impl<const N: usize, T: std::fmt::Debug> ArrayTrie<N, T> {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut nodes = Vec::with_capacity(capacity.max(1));
        nodes.push(ArrayTrieNode::default());
        Self { nodes }
    }

    pub fn insert(&mut self, key: impl IntoIterator<Item = u8>, value: T) {
        let idx = key.into_iter().fold(0u32, |idx, k| {
            if let Some(i) = self.nodes[idx as usize].children[usize::from(k)] {
                i.get()
            } else {
                self.nodes.push(ArrayTrieNode::default());
                let new = (self.nodes.len() - 1) as u32;
                self.nodes[idx as usize].children[usize::from(k)] = NonZeroU32::new(new);
                new
            }
        });
        self.nodes[idx as usize].value = Some(value);
    }

    pub fn walk(&self, key: impl IntoIterator<Item = u8>) -> impl Iterator<Item = &T> {
        key.into_iter().scan(Some(0u32), |idx, k| {
            let i = (*idx)?;
            *idx = self.nodes[i as usize].children[usize::from(k)].map(NonZeroU32::get);
            Some(self.nodes[(*idx)?  as usize].value.as_ref())
        }).flatten()
    }
}

#[test]
fn array_trie() {
    let mut trie: ArrayTrie<26, i32> = ArrayTrie::with_capacity(10);
    trie.insert(b"abc".iter().map(|c| c - b'a'), 1);
    trie.insert(b"abd".iter().map(|c| c - b'a'), 2);
    trie.insert(b"bcd".iter().map(|c| c - b'a'), 3);
    assert_eq!(Some(&1), trie.walk(b"abc".iter().map(|c| c - b'a')).next());
    assert_eq!(Some(&2), trie.walk(b"abd".iter().map(|c| c - b'a')).next());
    assert_eq!(Some(&3), trie.walk(b"bcd".iter().map(|c| c - b'a')).next());
    assert_eq!(None, trie.walk(b"ab".iter().map(|c| c - b'a')).next());
    assert_eq!(Some(&1), trie.walk(b"abcd".iter().map(|c| c - b'a')).next());
    assert_eq!(None, trie.walk(b"abe".iter().map(|c| c - b'a')).next());

    // Multiple values along a path
    trie.insert(b"a".iter().map(|c| c - b'a'), 10);
    trie.insert(b"ab".iter().map(|c| c - b'a'), 20);
    let vals: Vec<_> = trie.walk(b"abcd".iter().map(|c| c - b'a')).collect();
    assert_eq!(vec![&10, &20, &1], vals);

    // Iterator terminates after dead end
    let mut walk = trie.walk(b"abe".iter().map(|c| c - b'a'));
    assert_eq!(Some(&10), walk.next());
    assert_eq!(Some(&20), walk.next());
    assert_eq!(None, walk.next());
    assert_eq!(None, walk.next());
}
