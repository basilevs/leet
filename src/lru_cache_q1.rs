use std::collections::HashMap;


#[derive(Debug)]
struct Node {
    key: i32,
    value: i32,
    next: Option<usize>,
    prev: Option<usize>,
}

#[derive(Debug)]
struct LRUCache {
    buffer: Vec<Node>,
    index: HashMap<i32, usize>,
    head: Option<usize>,
    tail: Option<usize>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {


    fn new(capacity: i32) -> Self {
        let capacity = usize::try_from(capacity).expect("Can't be negative");
        LRUCache{ buffer: Vec::with_capacity(capacity), index: HashMap::with_capacity(capacity), head: None, tail: None }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if let Some(&n) = self.index.get(&key) {
            self.assert_valid_node(n);
            self.move_to_head(n);
            self.assert_valid_node(n);
            self.buffer[n].value
        } else {
            -1
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if let Some(&n) = self.index.get(&key) {
           self.buffer[n].value = value;
            self.assert_valid_node(n);
        } else {
            let n = self.evict_or_create();
            let node = &mut self.buffer[n];
            node.value = value;
            node.key = value;
            self.index.insert(key, n);
            self.assert_valid_node(n);
        }
    }

    fn evict_or_create(&mut self) -> usize {
        if self.buffer.capacity() > self.buffer.len() {
            self.buffer.push(Node {key: i32::MIN, value: i32::MIN, next: self.head, prev: None});
            let result = self.buffer.len() - 1;
            debug_assert_eq!(self.head.is_none(), self.tail.is_none());
            if let Some(head) = self.head {
                self.buffer[head].prev = Some(result);
                self.head = Some(result);
            } else {
                self.head = Some(result);
                self.tail = Some(result);
            }
            result
        } else {
            let result = self.tail.unwrap();
            let removed = self.index.remove(&self.buffer[result].key);
            debug_assert_eq!(Some(result), removed);
            self.move_to_head(result);
            result
        }
    }

    fn move_to_head(&mut self,  n: usize) {
        let next = self.buffer[n].next;
        let prev = self.buffer[n].prev;
        
        if let Some(prev) = prev {
            self.buffer[prev].next = next;
            if let Some(next_node) = next {
                self.buffer[next_node].prev = Some(prev);
            } else {
                self.tail = Some(prev);
            }
            self.buffer[n].prev = None;
            self.buffer[n].next = self.head;
            self.buffer[self.head.unwrap()].prev = Some(n);
            self.head = Some(n);
        } else {
            debug_assert_eq!(Some(n), self.head);
        }
    }

    #[cfg(not(debug_assertions))]
    fn assert_valid_node(&self, n:usize) {}

    #[cfg(debug_assertions)]
    fn assert_valid_node(&self, n:usize) {
        let node = &self.buffer[n];
        // dbg!(n, self);
        assert_eq!(self.index.get(&node.key).copied(), Some(n));
        if let Some(next) = node.next {
            assert_eq!(Some(n), self.buffer[next].prev);
        }
        assert_eq!(self.head == Some(n), node.prev.is_none());
        assert_eq!(self.tail == Some(n), node.next.is_none());
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