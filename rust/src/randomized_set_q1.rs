use std::collections::HashMap;

use rand::rngs::ThreadRng;
use rand::prelude::*;


pub struct RandomizedSet {
    index: HashMap<i32, usize>,
    list: Vec<i32>,
    random: ThreadRng,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    pub fn new() -> Self {
        Self { index: HashMap::new(), list: Vec::new(), random: rand::thread_rng() }
    }
    
    pub fn insert(&mut self, val: i32) -> bool {
        let mut result = false;
        self.index.entry(val).or_insert_with(|| {
            result = true;
            self.list.push(val);
            self.list.len() - 1
        });
        result
    }
    
    pub fn remove(&mut self, val: i32) -> bool {
        debug_assert_eq!(self.list.is_empty(), self.index.is_empty());
        let Some(i) = self.index.remove(&val) else {
            return false;
        };
        let removed = self.list.swap_remove(i);
        debug_assert_eq!(val, removed);
        if i < self.list.len()  {
            self.index.insert(self.list[i], i);
        }
        debug_assert_eq!(self.list.is_empty(), self.index.is_empty());
        true
    }
    
    pub fn get_random(&mut self) -> i32 {
        let random: u64 = self.random.next_u64();
        self.list[random as usize % self.list.len()]
    }
}
