pub fn left_right_difference(mut nums: Vec<i32>) -> Vec<i32> {
    let mut right_sum: i32 = nums.iter().sum();
    let mut left_sum = 0;
    for i in nums.iter_mut() {
        right_sum -= *i;
        let result: i32 = left_sum - right_sum; // Use std::mem::replace instead of a temporary!
        left_sum += *i;
        *i = result.abs();
    }
    nums
}
