use crate::solution::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
       let mut max = i32::MIN;
       let mut max_count = 0;
       let mut unique = HashSet::with_capacity(nums.len());
       for v in nums {
            if v > max {
                max = v;
                max_count = 1;
                unique.insert(v);
            } else if v == max {
                max_count += 1;
                if max_count > 2 {
                    return false;
                }
            } else {
                if !unique.insert(v) {
                    return false;
                }
            }
       }
       max_count == 2 && unique.len() == max as usize
    }
}

#[test]
fn official1() {
    assert!(Solution::is_good(vec![1, 3, 3, 2]));
}

#[test]
fn official2() {
    assert!(!Solution::is_good(vec![9,9]));
}