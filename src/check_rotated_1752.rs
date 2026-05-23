pub fn check(nums: Vec<i32>) -> bool {
    let partition_value = *nums.first().unwrap();
    let prefix_len = nums.iter().take_while(|&x| *x == partition_value).count();
    nums[prefix_len..].is_sorted_by_key(|&x| (x<=partition_value, x))
}


// Example 1:
// Input: nums = [3,4,5,1,2]
// Output: true
// Explanation: [1,2,3,4,5] is the original sorted array.
// You can rotate the array by x = 2 positions to begin on the element of value 3: [3,4,5,1,2].
#[test]
fn official1() {
    assert!(check(vec![3,4,5,1,2]));
}

// Example 2:
// Input: nums = [2,1,3,4]
// Output: false
// Explanation: There is no sorted array once rotated that can make nums.
#[test]
fn official2() {
    assert!(!check(vec![2,1,3,4]));
}

// Example 3:
// Input: nums = [1,2,3]
// Output: true
// Explanation: [1,2,3] is the original sorted array.
// You can rotate the array by x = 0 positions (i.e. no rotation) to make nums.
#[test]
fn official3() {
    assert!(check(vec![1,2,3]));
}

#[test]
fn t1() {
    assert!(check(vec![3,3,4,5,1,2,3,3]));
}