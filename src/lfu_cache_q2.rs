use std::collections::HashMap;

use crate::vec_list::VecList;

#[derive(Debug)]
struct Node {
    key: i32,
    value: i32,
    cnt: usize,
}

#[derive(Debug)]
struct LFUCache {
    buffer: VecList<Node>,
    index: HashMap<i32, usize>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {

    fn new(capacity: i32) -> Self {
        let capacity = usize::try_from(capacity).expect("capacity > 0");
        LFUCache { buffer: VecList::with_capacity(capacity), index: HashMap::with_capacity(capacity )}
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if let Some(&n) = self.index.get(&key) {
            self.assert_valid_node(n);
            self.access(n);
            self.assert_valid_node(n);
            self.buffer.get(n).value
        } else {
            -1
        }
    }

    fn access(&mut self, n: usize) {
        self.buffer.get_mut(n).cnt += 1;
        dbg!(n, &self);
        self.bubble(n);
        dbg!(n, &self);
        self.assert_valid_node(n);
    }

    fn bubble(&mut self, n: usize) {
        dbg!(n, &self);
        let mut i = n;
        loop {
            if let Some(j) = self.buffer.previous(i) {
                if self.buffer.get(j).cnt > self.buffer.get(i).cnt {
                    break;
                }
                i = j;
            } else {
                break;
            }
        }
        if i != n {
            self.buffer.swap_order(i, n);
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if let Some(&n) = self.index.get(&key) {
           self.buffer.get_mut(n).value = value;
           self.access(n);
            self.assert_valid_node(n);
        } else {
            let (n, old) = self.buffer.push_tail_evicting(Node { key, value, cnt: 0 });
            if let Some(old) = old {
                let removed = self.index.remove(&old.key);
                debug_assert_eq!(Some(n), removed);
            }
            self.index.insert(key, n);
            self.bubble(n);
            self.assert_valid_node(n);
        }
    }
    
    #[cfg(not(debug_assertions))]
    fn assert_valid_node(&self, _n: usize) {}

    #[cfg(debug_assertions)]
    fn assert_valid_node(&self, n: usize) {
        dbg!(&self);
        self.buffer.assert_valid_node(n);
        let Node {key, cnt, ..} = self.buffer.get(n);
        assert_eq!(self.index.get(key).copied(), Some(n));
        if let Some(p) = self.buffer.previous(n) {
            let previous = self.buffer.get(p);
            assert!(previous.cnt >= *cnt, "n: {}, key: {}, cnt: {}", n, key, cnt);
        }
        let head_cnt = self.buffer.get(self.buffer.head().unwrap()).cnt;
        assert!( head_cnt >= *cnt, "n: {}, key: {}, cnt: {}, head_cnt: {}", n, key, cnt, head_cnt);
        let tail_cnt = self.buffer.get(self.buffer.tail().unwrap()).cnt;
        assert!(tail_cnt <= *cnt, "n: {}, key: {}, cnt: {}, tail_cnt: {}", n, key, cnt, tail_cnt);
    }
    
}

/**
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */



#[test]
fn official1() {
// cnt(x) = the use counter for key x
// cache=[] will show the last used order for tiebreakers (leftmost element is  most recent)
    let mut lfu = LFUCache::new(2);
    lfu.put(1, 1);   // cache=[1,_], cnt(1)=1
    dbg!(&lfu);
    lfu.put(2, 2);   // cache=[2,1], cnt(2)=1, cnt(1)=1
    dbg!(&lfu);
    assert_eq!(1, lfu.get(1));  // cache=[1,2], cnt(2)=1, cnt(1)=2
    dbg!(&lfu);                 
    lfu.put(3, 3);   // 2 is the LFU key because cnt(2)=1 is the smallest, invalidate 2.
                                // cache=[3,1], cnt(3)=1, cnt(1)=2
    assert_eq!(-1, lfu.get(2));
    assert_eq!(3, lfu.get(3));  // cache=[3,1], cnt(3)=2, cnt(1)=2
    lfu.put(4, 4);   // Both 1 and 3 have the same cnt, but 1 is LRU, invalidate 1.
                                // cache=[4,3], cnt(4)=1, cnt(3)=2
    assert_eq!(-1, lfu.get(1));
    assert_eq!(3, lfu.get(3));  // cache=[3,4], cnt(4)=1, cnt(3)=3
    assert_eq!(4, lfu.get(4));  // cache=[4,3], cnt(4)=2, cnt(3)=3
}

#[test]
fn capacity_one() {
    let mut c = LFUCache::new(1);
    c.put(1, 10);
    assert_eq!(10, c.get(1));
    c.put(2, 20);             // evicts 1
    assert_eq!(-1, c.get(1));
    assert_eq!(20, c.get(2));
    c.put(2, 30);             // update existing
    assert_eq!(30, c.get(2));
}

#[test]
fn evict_least_frequent() {
    // key 1 accessed twice, key 2 once → evict key 2
    let mut c = LFUCache::new(2);
    c.put(1, 10);
    c.put(2, 20);
    c.get(1);        // cnt(1)=2, cnt(2)=1
    c.put(3, 30);    // evicts key 2 (lowest freq)
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
    c.put(3, 30);    // evicts key 1
    assert_eq!(-1, c.get(1));
    dbg!(&c);
    assert_eq!(20, c.get(2));
    assert_eq!(30, c.get(3));
}

#[test]
fn update_refreshes_frequency() {
    // put on existing key increments use counter
    let mut c = LFUCache::new(2);
    c.put(1, 10);
    c.put(2, 20);
    c.put(1, 100);   // update key 1 → cnt(1)=2, cnt(2)=1
    c.put(3, 30);    // evicts key 2 (lower freq)
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
    c.put(3, 30);    // evicts 1 (LRU among tied freq)
    assert_eq!(-1, c.get(1));
    c.put(1, 99);    // re-insert key 1, evicts 2 (LRU among tied freq)
    assert_eq!(99, c.get(1));
    assert_eq!(-1, c.get(2));
    assert_eq!(30, c.get(3));
}

#[test]
fn capacity_three_mixed_frequencies() {
    let mut c = LFUCache::new(3);
    c.put(1, 1);
    c.put(2, 2);
    c.put(3, 3);
    c.get(1);        // cnt(1)=2
    c.get(2);        // cnt(2)=2
    c.get(3);        // cnt(3)=2
    dbg!(&c);
    c.get(1);        // cnt(1)=3
    dbg!(&c);
    c.put(4, 4);     // all tied at cnt=2 except key 1(cnt=3); evict LRU among cnt=2 → key 2
    assert_eq!(1, c.get(1));
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
    c.get(1);        // cnt(1)=2
    c.get(1);        // cnt(1)=3
    c.put(2, 2);     // cnt(2)=1
    dbg!(&c);
    c.put(3, 3);     // evicts 2 (cnt=1 < cnt(1)=3)
    dbg!(&c);
    assert_eq!(1, c.get(1));
    assert_eq!(-1, c.get(2));
    assert_eq!(3, c.get(3));
    c.put(4, 4);     // cnt(3)=2 vs cnt(4)=1; evicts key 4? no — evicts 3 has cnt=2, key 1 has cnt=4; evict LFU=4's cnt=1... wait
    // After above: cnt(1)=4, cnt(3)=2. Insert 4 with cnt=1 → evicts tail which is LFU
    // Actually 4 is being inserted, so we need room. Evict LFU among {1(cnt=4), 3(cnt=2)} → evict 3
    assert_eq!(4, c.get(4));
    assert_eq!(-1, c.get(3));
    assert_eq!(1, c.get(1));
}