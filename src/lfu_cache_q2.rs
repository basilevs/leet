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
        let current = self.buffer.get(n);
        if let Some(p) = self.buffer.previous(n) {
            let previous = self.buffer.get(p);
            if previous.cnt <= current.cnt {
                self.buffer.swap_order(n, p);
            }
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if let Some(&n) = self.index.get(&key) {
           self.buffer.get_mut(n).value = value;
           self.access(n);
            self.assert_valid_node(n);
        } else {
            let (n, old) = self.buffer.push_head_evicting(Node { key, value, cnt: 0 });
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
        if let Some(p) = self.buffer.previous(n) {
            let previous = self.buffer.get(p);
            assert!(previous.cnt <= node.cnt);
        }
        self.buffer.assert_valid_node(n);
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
    lfu.put(2, 2);   // cache=[2,1], cnt(2)=1, cnt(1)=1
    assert_eq!(1, lfu.get(1));  // cache=[1,2], cnt(2)=1, cnt(1)=2
                                
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