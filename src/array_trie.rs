use std::fmt::Debug;
use std::num::NonZeroU32;

pub struct ArrayTrie<const N: usize, T: std::fmt::Debug> {
    nodes: Vec<ArrayTrieNode<N, T>>,
}

impl<const N: usize, T: Debug> Debug for ArrayTrie<N, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArrayTrie")
            .field("nodes", &self.nodes)
            .finish()
    }
}

#[derive(Debug)]
struct ArrayTrieNode<const N: usize, T: std::fmt::Debug> {
    value: Option<T>,
    children: [Option<NonZeroU32>; N],
}

impl<const N: usize, T: std::fmt::Debug> Default for ArrayTrieNode<N, T> {
    fn default() -> Self {
        Self {
            value: None,
            children: [None; N],
        }
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

    pub fn insert_prefixes(&mut self, key: impl IntoIterator<Item = u8>, value: impl Fn() -> T) {
        key.into_iter().fold(0u32, |idx, k| {
            let next = if let Some(i) = self.nodes[idx as usize].children[usize::from(k)] {
                i.get()
            } else {
                self.nodes.push(ArrayTrieNode::default());
                let new = (self.nodes.len() - 1) as u32;
                self.nodes[idx as usize].children[usize::from(k)] = NonZeroU32::new(new);
                new
            };
            self.nodes[next as usize].value = Some(value());
            next
        });
    }

    pub fn walk(&self, key: impl IntoIterator<Item = u8>) -> impl Iterator<Item = &T> {
        key.into_iter()
            .scan(Some(0u32), |idx, k| {
                let i = (*idx)?;
                *idx = self.nodes[i as usize].children[usize::from(k)].map(NonZeroU32::get);
                Some(self.nodes[(*idx)? as usize].value.as_ref())
            })
            .flatten()
    }

    pub fn visit(&self, mut visitor: impl FnMut(&[u8], &T)) {
        let mut key = vec![];
        let mut stack: Vec<usize> = vec![];
        let Some(node) = self.nodes.first() else {
            return;
        };
        if let Some(value) = &node.value {
            visitor(&key, value);
        }
        stack.push(0);
        key.push(0);
        loop {
            debug_assert_eq!(
                stack.len(),
                key.len(),
                "stack: {:?}, key: {:?}",
                &stack,
                &key
            );
            let Some(&node_id) = stack.last() else {
                return;
            };
            let node = &self.nodes[node_id];
            let k = *key.last().unwrap();
            if let Some(child_id) = node.children[usize::from(k)] {
                let child_id = child_id.get() as usize;
                let child = &self.nodes[child_id];
                if let Some(value) = &child.value {
                    visitor(&key, value);
                }
                stack.push(child_id);
                key.push(0);
                continue;
            }
            while let Some(k) = key.last()
                && k == &(N as u8 - 1)
            {
                stack.pop();
                key.pop();
            }
            if let Some(k) = key.last_mut() {
                *k += 1;
            } else {
                return;
            }
        }
    }
}

#[cfg(test)]
fn lowercase_string_to_u8(chars: &str) -> impl Iterator<Item = u8> {
    chars.as_bytes().iter().map(|c| c - b'a')
}

#[test]
fn array_trie() {
    let mut trie: ArrayTrie<26, i32> = ArrayTrie::with_capacity(10);
    trie.insert(lowercase_string_to_u8("abc"), 1);
    trie.insert(lowercase_string_to_u8("abd"), 2);
    trie.insert(lowercase_string_to_u8("bcd"), 3);
    assert_eq!(Some(&1), trie.walk(lowercase_string_to_u8("abc")).next());
    assert_eq!(Some(&2), trie.walk(lowercase_string_to_u8("abd")).next());
    assert_eq!(Some(&3), trie.walk(lowercase_string_to_u8("bcd")).next());
    assert_eq!(None, trie.walk(lowercase_string_to_u8("ab")).next());
    assert_eq!(Some(&1), trie.walk(lowercase_string_to_u8("abcd")).next());
    assert_eq!(None, trie.walk(lowercase_string_to_u8("abe")).next());

    // Multiple values along a path
    trie.insert(lowercase_string_to_u8("a"), 10);
    trie.insert(lowercase_string_to_u8("ab"), 20);
    let vals: Vec<_> = trie.walk(lowercase_string_to_u8("abcd")).collect();
    assert_eq!(vec![&10, &20, &1], vals);

    // Iterator terminates after dead end
    let mut walk = trie.walk(lowercase_string_to_u8("abe"));
    assert_eq!(Some(&10), walk.next());
    assert_eq!(Some(&20), walk.next());
    assert_eq!(None, walk.next());
    assert_eq!(None, walk.next());
}

#[test]
fn array_trie_insert_prefixes() {
    let mut trie: ArrayTrie<10, ()> = ArrayTrie::with_capacity(4);
    trie.insert_prefixes([1, 0, 5], || ());
    assert_eq!(3, trie.walk([1, 0, 5]).count());
    assert_eq!(2, trie.walk([1, 0, 7]).count());
    assert_eq!(1, trie.walk([1, 9]).count());
    let mut collector = vec![];
    trie.visit(|k, _| collector.push(k.to_vec()));
    assert_eq!(vec![vec![1], vec![1, 0], vec![1, 0, 5]], collector);
}
