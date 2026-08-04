// https://leetcode.com/problems/maximum-total-subarray-value-i

pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
    i64::from(k) * i64::from(nums.iter().max().unwrap() - nums.iter().min().unwrap())
}
