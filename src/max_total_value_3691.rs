// https://leetcode.com/problems/maximum-total-subarray-value-ii

use std::{collections::BinaryHeap};

use crate::sparse_table::SparseTable;

pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let min = SparseTable::new(nums.clone(), i32::min);
    let max = SparseTable::new(nums, i32::max);
    let k = usize::try_from(k).expect("k should be positive");
    let mut queue = BinaryHeap::with_capacity(n);

    for left in 0..n {
        let value = max.query(left..n) - min.query(left..n);
        queue.push((value, left, n));
    }
    
    let mut result = 0i64;
    for _ in 0..k {
        let (value, left, right) = queue.pop().expect("k <= number of subarrays");
        result += i64::from(value);
        if left + 1 < right {
            let end = right - 1;
            let value = max.query(left..end) - min.query(left..end);
            queue.push((value, left, end));
        }
    }
    result

}

#[test]
fn official1() {
    assert_eq!(4, max_total_value(vec![1, 3, 2], 2));
}

#[test]
fn official2() {
    assert_eq!(12, max_total_value(vec![4, 2, 5, 1], 3));
}

#[test]
fn official37() {
    assert_eq!(3, max_total_value(vec![11, 8], 3));
}
