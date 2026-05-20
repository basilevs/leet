use std::collections::HashSet;

pub fn find_the_prefix_common_array(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut seen = HashSet::with_capacity(a.len());
    for i in 0..a.len() {
        seen.insert(a[i]);
        seen.insert(b[i]);
        a[i] = (2 + 2*i - seen.len()) as i32;
    }
    a
}

// Example 1:

// Input: A = [1,3,2,4], B = [3,1,2,4]
// Output: [0,2,3,4]
// Explanation: At i = 0: no number is common, so C[0] = 0.
// At i = 1: 1 and 3 are common in A and B, so C[1] = 2.
// At i = 2: 1, 2, and 3 are common in A and B, so C[2] = 3.
// At i = 3: 1, 2, 3, and 4 are common in A and B, so C[3] = 4.
#[test]
fn official1() {
    assert_eq!(vec![0,2,3,4], find_the_prefix_common_array(vec![1,3,2,4], vec![3,1,2,4]));
}


// Example 2:

// Input: A = [2,3,1], B = [3,1,2]
// Output: [0,1,3]
// Explanation: At i = 0: no number is common, so C[0] = 0.
// At i = 1: only 3 is common in A and B, so C[1] = 1.
// At i = 2: 1, 2, and 3 are common in A and B, so C[2] = 3.
#[test]
fn official2() {
    assert_eq!(vec![0,1,3], find_the_prefix_common_array(vec![2,3,1], vec![3,1,2]));
}