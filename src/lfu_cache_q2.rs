use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
struct Node {
    value: i32,
    cnt: usize,
    access_id: usize, // cheat to avoid use of linked lists to preserve insertion order
}

#[derive(Debug)]
pub struct LFUCache {
    access_id: usize,
    capacity: usize,
    index: HashMap<i32, Node>,
    priority_queue: BTreeMap<(usize, usize), i32>,
}

impl LFUCache {
    /// # Panics
    /// Panics if `capacity` is not positive.
    #[must_use]
    pub fn new(capacity: i32) -> Self {
        assert!(capacity > 0);
        let capacity: usize = capacity.try_into().unwrap();
        Self {
            capacity,
            access_id: 0,
            index: HashMap::with_capacity(capacity),
            priority_queue: BTreeMap::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.index.get_mut(&key) {
            let result = node.value;
            self.access(key);
            result
        } else {
            -1
        }
    }

    fn access(&mut self, key: i32) {
        let node = self.index.get_mut(&key).unwrap();
        let removed = self.priority_queue.remove(&(node.cnt, node.access_id));
        debug_assert_eq!(Some(key), removed);
        node.cnt += 1;
        node.access_id = self.access_id;
        self.access_id += 1;
        let replaced = self.priority_queue.insert((node.cnt, node.access_id), key);
        debug_assert_eq!(None, replaced);
    }

    /// # Panics
    /// Panics on internal invariant violation (should never occur in correct usage).
    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(n) = self.index.get_mut(&key) {
            n.value = value;
        } else {
            if self.capacity <= self.index.len() {
                let evicted_key = self.priority_queue.first_entry().unwrap().remove();
                self.index.remove(&evicted_key);
            }
            let node = Node {value, cnt: 0, access_id: self.access_id};
            self.priority_queue.insert((node.cnt, node.access_id), key);
            self.index.insert(key, node);
            self.access_id += 1;
        }
        self.access(key);
        debug_assert!(self.capacity >= self.index.len(), "capacity: {} => len: {}", self.capacity, self.index.len());
    }

}


#[test]
fn official1() {
    // cnt(x) = the use counter for key x
    // cache=[] will show the last used order for tiebreakers (leftmost element is  most recent)
    let mut lfu = LFUCache::new(2);
    lfu.put(1, 1); // cache=[1,_], cnt(1)=1
    assert_eq!(vec![1], to_keys_vec(&lfu));
    lfu.put(2, 2); // cache=[2,1], cnt(2)=1, cnt(1)=1
    assert_eq!(vec![2, 1], to_keys_vec(&lfu));
    assert_eq!(1, lfu.get(1)); // cache=[1,2], cnt(2)=1, cnt(1)=2
    assert_eq!(vec![1, 2], to_keys_vec(&lfu));
    lfu.put(3, 3); // 2 is the LFU key because cnt(2)=1 is the smallest, invalidate 2.
    // cache=[1,3], cnt(3)=1, cnt(1)=2
    assert_eq!(vec![1, 3], to_keys_vec(&lfu));
    assert_eq!(-1, lfu.get(2));
    assert_eq!(3, lfu.get(3)); // cache=[3,1], cnt(3)=2, cnt(1)=2
    assert_eq!(vec![3, 1], to_keys_vec(&lfu));
    lfu.put(4, 4); // Both 1 and 3 have the same cnt, but 1 is LRU, invalidate 1.
    // cache=[3,4], cnt(4)=1, cnt(3)=2
    assert_eq!(vec![3, 4], to_keys_vec(&lfu));
    assert_eq!(-1, lfu.get(1));
    assert_eq!(3, lfu.get(3)); // cache=[3,4], cnt(4)=1, cnt(3)=3
    assert_eq!(4, lfu.get(4)); // cache=[4,3], cnt(4)=2, cnt(3)=3
}

#[test]
fn capacity_one() {
    let mut c = LFUCache::new(1);
    c.put(1, 10);
    assert_eq!(10, c.get(1));
    c.put(2, 20); // evicts 1
    assert_eq!(-1, c.get(1));
    assert_eq!(20, c.get(2));
    c.put(2, 30); // update existing
    assert_eq!(30, c.get(2));
}

#[test]
fn evict_least_frequent() {
    // key 1 accessed twice, key 2 once → evict key 2
    let mut c = LFUCache::new(2);
    c.put(1, 10);
    c.put(2, 20);
    c.get(1); // cnt(1)=2, cnt(2)=1
    c.put(3, 30); // evicts key 2 (lowest freq)
    assert_eq!(-1, c.get(2));
    assert_eq!(10, c.get(1));
    assert_eq!(30, c.get(3));
}

#[test]
fn tie_breaks_by_lru() {
    // Both keys have same frequency → evict least recently used
    let mut c = LFUCache::new(2);
    c.put(1, 10);
    c.put(2, 20);
    // cnt(1)=1, cnt(2)=1; key 1 was used least recently
    c.put(3, 30); // evicts key 1
    assert_eq!(-1, c.get(1));
    assert_eq!(20, c.get(2));
    assert_eq!(30, c.get(3));
}

#[test]
fn update_refreshes_frequency() {
    // put on existing key increments use counter
    let mut c = LFUCache::new(2);
    c.put(1, 10);
    c.put(2, 20);
    c.put(1, 100); // update key 1 → cnt(1)=2, cnt(2)=1
    c.put(3, 30); // evicts key 2 (lower freq)
    assert_eq!(100, c.get(1));
    assert_eq!(-1, c.get(2));
    assert_eq!(30, c.get(3));
}

#[test]
fn get_missing_key() {
    let mut c = LFUCache::new(2);
    assert_eq!(-1, c.get(0));
    assert_eq!(-1, c.get(100000));
    c.put(5, 50);
    assert_eq!(-1, c.get(4));
    assert_eq!(50, c.get(5));
}

#[test]
fn reinsert_after_eviction() {
    let mut c = LFUCache::new(2);
    c.put(1, 10);
    c.put(2, 20);
    c.put(3, 30); // evicts 1 (LRU among tied freq)
    assert_eq!(-1, c.get(1));
    c.put(1, 99); // re-insert key 1, evicts 2 (LRU among tied freq)
    assert_eq!(99, c.get(1));
    assert_eq!(-1, c.get(2));
    assert_eq!(30, c.get(3));
}

// #[cfg(test)]
// fn to_entries_vec(input: &LFUCache) -> Vec<(i32, i32)> {
//     input.buffer.iter().map(|n| (n.1.key, n.1.value)).collect()
// }

#[cfg(test)]
fn to_keys_vec(input: &LFUCache) -> Vec<i32> {
    input.priority_queue.values().rev().copied().collect()
}

#[test]
fn capacity_three_mixed_frequencies() {
    let mut c: LFUCache = LFUCache::new(3);
    c.put(1, 1); // cache = [1] cnt(1)=1
    c.put(2, 2); // cache = [2,1] cnt(1)=1 cnt(2)=1
    assert_eq!(vec![2, 1], to_keys_vec(&c));
    c.put(3, 3); // cache = [3,2,1] cnt(1)=1 cnt(2)=1 cnt(3) = 1
    assert_eq!(vec![3, 2, 1], to_keys_vec(&c));
    c.get(1); // cache=[1,3,2] cnt(1)=2 cnt(2)=1 cnt(3) = 1
    assert_eq!(vec![1, 3, 2], to_keys_vec(&c));
    c.get(2); // cache=[2,1,3] cnt(1)=2 cnt(2)=2 cnt(3) = 1
    assert_eq!(vec![2, 1, 3], to_keys_vec(&c));
    c.get(3); // cache=[3,2,1] cnt(1)=2 cnt(2)=2 cnt(3)=2
    assert_eq!(vec![3, 2, 1], to_keys_vec(&c));
    c.get(1); // cache=[1,3,2] cnt(1)=3
    c.put(4, 4); // all tied at cnt=2 except key 1(cnt=3); evict LRU among cnt=2 → key 2
    // cache=[1,3,4]
    assert_eq!(-1, c.get(2));
    assert_eq!(1, c.get(1)); // cache=[1,3,4] cnt(1)=4
    assert_eq!(-1, c.get(2));
    assert_eq!(3, c.get(3));
    assert_eq!(4, c.get(4));
}

#[test]
fn key_value_boundary() {
    // Test boundary values from constraints
    let mut c = LFUCache::new(2);
    c.put(0, 1_000_000_000);
    c.put(100_000, 0);
    assert_eq!(1_000_000_000, c.get(0));
    assert_eq!(0, c.get(100_000));
}

#[test]
fn many_evictions_preserve_highest_freq() {
    let mut c = LFUCache::new(2);
    c.put(1, 1);
    c.get(1); // cnt(1)=2
    c.get(1); // cnt(1)=3
    c.put(2, 2); // cnt(2)=1
    c.put(3, 3); // evicts 2 (cnt=1 < cnt(1)=3)
    assert_eq!(1, c.get(1));
    assert_eq!(-1, c.get(2));
    assert_eq!(3, c.get(3));
    c.put(4, 4); // cnt(3)=2 vs cnt(4)=1; evicts key 4? no — evicts 3 has cnt=2, key 1 has cnt=4; evict LFU=4's cnt=1... wait
    // After above: cnt(1)=4, cnt(3)=2. Insert 4 with cnt=1 → evicts tail which is LFU
    // Actually 4 is being inserted, so we need room. Evict LFU among {1(cnt=4), 3(cnt=2)} → evict 3
    assert_eq!(4, c.get(4));
    assert_eq!(-1, c.get(3));
    assert_eq!(1, c.get(1));
}
