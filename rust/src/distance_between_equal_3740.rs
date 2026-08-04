use std::{iter::repeat_with};

struct Queue {
    elements: [u32; 2],
    len: u8,
}

impl Queue {
    fn new() -> Queue {
        Queue { elements: [0,0], len: 0 }
    }
    fn push_and_compute_distance(&mut self, candidate: u32) -> Option<i32>{
        if self.len < 2 {
            self.elements[self.len as usize] = candidate;
            self.len += 1;
            None
        } else {
            let i = self.elements[0];
            let j = self.elements[1];
            let k = candidate;
            self.elements[0] = j;
            self.elements[1] = k;
            Some((i.abs_diff(j) + j.abs_diff(k) + k.abs_diff(i)).try_into().expect("Distance is too large"))
        }
    }
}

pub fn minimum_distance(nums: Vec<i32>) -> i32 {
    let mut last: Vec<Queue> = repeat_with(&Queue::new).take(nums.len()).collect();
    nums.into_iter().enumerate().filter_map(|(i, value)| {
        let i = i as u32; // less than 10^5
        let indices = last.get_mut((value-1) as usize).expect("value is greater than input length");
        indices.push_and_compute_distance(i)
    }).min().unwrap_or(-1)
}

#[test]
fn official1() {
    let input = [1,2,1,1,3];
    assert_eq!(6, minimum_distance(Vec::from(input)));
}

#[test]
fn official2() {
    let input = [1,1,2,3,2,1,2];
    assert_eq!(8, minimum_distance(Vec::from(input)));
}

#[test]
fn official3() {
    let input = [1];
    assert_eq!(-1, minimum_distance(Vec::from(input)));
}

#[test]
fn push_out() {
    let input = [1,1,2,3,2,1,2,1,1,1];
    assert_eq!(4, minimum_distance(Vec::from(input)));
}