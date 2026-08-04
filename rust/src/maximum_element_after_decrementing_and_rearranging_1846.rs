// https://leetcode.com/problems/maximum-element-after-decreasing-and-rearranging

pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
    arr.sort_unstable(); // Editorial does counting sort
    let mut result = 1;
    for &value in arr[1..].into_iter() {
        if value <= result {
            result = value;
        } else {
            result += 1;
        }
    }
    result

}

#[cfg(test)]
mod tests {
    use super::maximum_element_after_decrementing_and_rearranging;

    // Example 1:
    // Input: arr = [2,2,1,2,1]
    // Output: 2
    // Explanation: 
    // We can satisfy the conditions by rearranging arr so it becomes [1,2,2,2,1].
    // The largest element in arr is 2.
    #[test]
    fn official1() {
        assert_eq!(2, maximum_element_after_decrementing_and_rearranging(vec![2,2,1,2,1]));
    }

    // Example 2:
    // Input: arr = [100,1,1000]
    // Output: 3
    // Explanation: 
    // One possible way to satisfy the conditions is by doing the following:
    // 1. Rearrange arr so it becomes [1,100,1000].
    // 2. Decrease the value of the second element to 2.
    // 3. Decrease the value of the third element to 3.
    // Now arr = [1,2,3], which satisfies the conditions.
    // The largest element in arr is 3.
    #[test]
    fn official2() {
        assert_eq!(3, maximum_element_after_decrementing_and_rearranging(vec![100,1,1000]));
    }

    // Example 3:
    // Input: arr = [1,2,3,4,5]
    // Output: 5
    // Explanation: The array already satisfies the conditions, and the largest element is 5.
    #[test]
    fn official3() {
        assert_eq!(5, maximum_element_after_decrementing_and_rearranging(vec![1,2,3,4,5]));
    }

    #[test]
    fn official19() {
        assert_eq!(3, maximum_element_after_decrementing_and_rearranging(vec![73,98,9]));
    }
}   