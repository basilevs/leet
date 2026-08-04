use std::collections::HashMap;
use crate::vec_list::VecList;


#[derive(Debug)]
struct Node {
    key: i32,
    value: i32,
}

#[derive(Debug)]
pub struct LRUCache {
    buffer: VecList<Node>,
    index: HashMap<i32, usize>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {


    pub fn new(capacity: i32) -> Self {
        let capacity = usize::try_from(capacity).expect("Can't be negative");
        LRUCache{ buffer: VecList::with_capacity(capacity), index: HashMap::with_capacity(capacity) }
    }
    
    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&n) = self.index.get(&key) {
            self.assert_valid_node(n);
            self.buffer.move_to_head(n);
            self.assert_valid_node(n);
            self.buffer.get(n).value
        } else {
            -1
        }
    }
    
    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&n) = self.index.get(&key) {
           self.buffer.get_mut(n).value = value;
           self.buffer.move_to_head(n);
            self.assert_valid_node(n);
        } else {
            let (n, old) = self.buffer.push_head_evicting(Node { key, value });
            if let Some(old) = old {
                let removed = self.index.remove(&old.key);
                debug_assert_eq!(Some(n), removed);
            }
            self.index.insert(key, n);
            self.assert_valid_node(n);
        }
    }

    #[cfg(not(debug_assertions))]
    fn assert_valid_node(&self, _n: usize) {}

    #[cfg(debug_assertions)]
    fn assert_valid_node(&self, n: usize) {
        let node = self.buffer.get(n);
        assert_eq!(self.index.get(&node.key).copied(), Some(n));
        self.buffer.assert_valid_node(n);
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

 #[test]
 fn official1() {
// LRUCache lRUCache = new LRUCache(2);
// lRUCache.put(1, 1); // cache is {1=1}
// lRUCache.put(2, 2); // cache is {1=1, 2=2}
// lRUCache.get(1);    // return 1
// lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
// lRUCache.get(2);    // returns -1 (not found)
// lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
// lRUCache.get(1);    // return -1 (not found)
// lRUCache.get(3);    // return 3
// lRUCache.get(4);    // return 4
    let mut lru_cache = LRUCache::new(2);
    lru_cache.put(1, 1);
    lru_cache.put(2, 2);
    assert_eq!(1, lru_cache.get(1));
    lru_cache.put(3, 3);
    assert_eq!(-1, lru_cache.get(2));
    lru_cache.put(4, 4);
    assert_eq!(-1, lru_cache.get(1));
    assert_eq!(3, lru_cache.get(3));
    assert_eq!(4, lru_cache.get(4));
    lru_cache.put(4, 5);
    assert_eq!(5, lru_cache.get(4));
 }

 #[test]
 fn capacity_one() {
    let mut c = LRUCache::new(1);
    c.put(1, 10);
    assert_eq!(10, c.get(1));
    c.put(2, 20);
    assert_eq!(-1, c.get(1)); // evicted
    assert_eq!(20, c.get(2));
    c.put(2, 30); // update existing
    assert_eq!(30, c.get(2));
 }

 #[test]
 fn update_does_not_evict() {
    // Updating an existing key should NOT evict anything
    let mut c = LRUCache::new(2);
    c.put(1, 1);
    c.put(2, 2);
    c.put(1, 10); // update key 1, should not evict key 2
    assert_eq!(10, c.get(1));
    assert_eq!(2, c.get(2)); // key 2 must still be present
 }

 #[test]
 fn eviction_order_after_get() {
    // get() should make the accessed key most-recently-used
    let mut c = LRUCache::new(2);
    c.put(1, 1);
    c.put(2, 2);
    c.get(1);       // makes key 1 most recent; key 2 is now LRU
    c.put(3, 3);    // should evict key 2
    assert_eq!(-1, c.get(2));
    assert_eq!(1, c.get(1));
    assert_eq!(3, c.get(3));
 }

 #[test]
 fn eviction_order_after_put_update() {
    // put() with existing key should also refresh recency
    let mut c = LRUCache::new(2);
    c.put(1, 1);
    c.put(2, 2);
    c.put(1, 10);   // update key 1 → now most recent; key 2 is LRU
    c.put(3, 3);    // should evict key 2
    assert_eq!(-1, c.get(2));
    assert_eq!(10, c.get(1));
    assert_eq!(3, c.get(3));
 }

 #[test]
 fn key_value_differ() {
    // Key and value are independent; make sure they aren't mixed up
    let mut c = LRUCache::new(2);
    c.put(0, 100000);
    c.put(100, 0);
    assert_eq!(100000, c.get(0));
    assert_eq!(0, c.get(100));
 }

 #[test]
 fn reinsert_after_eviction() {
    let mut c = LRUCache::new(2);
    c.put(1, 1);
    c.put(2, 2);
    c.put(3, 3);  // evicts 1
    assert_eq!(-1, c.get(1));
    c.put(1, 99); // re-insert key 1 (evicts 2)
    assert_eq!(99, c.get(1));
    assert_eq!(-1, c.get(2));
    assert_eq!(3, c.get(3));
 }

 #[test]
 fn capacity_three_full_cycle() {
    let mut c = LRUCache::new(3);
    c.put(1, 1);
    c.put(2, 2);
    c.put(3, 3);
    c.put(4, 4);  // evicts 1
    assert_eq!(-1, c.get(1));
    assert_eq!(2, c.get(2));
    assert_eq!(3, c.get(3));
    assert_eq!(4, c.get(4));
    c.put(5, 5);  // evicts 2 (LRU after the gets above refreshed 2,3,4)
    assert_eq!(-1, c.get(2));
 }

 #[test]
 fn get_missing_key() {
    let mut c = LRUCache::new(2);
    assert_eq!(-1, c.get(0));
    assert_eq!(-1, c.get(10000));
    c.put(5, 5);
    assert_eq!(-1, c.get(4));
    assert_eq!(5, c.get(5));
 }
 