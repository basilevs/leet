
// Common minimum shared value in sorted inputs 
pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut i1=0;
    let mut i2=0;
    loop {
        let Some(v1) = nums1.get(i1) else {
            return -1;
        };
        let Some(v2) = nums2.get(i2) else {
            return -1;
        };
        if v1 < v2 {
            i1 += 1
        } else if v1 > v2 {
            i2 += 1;
        } else if v1 == v2 {
            return *v1;
        }
    }
}

// Example 1:
// Input: nums1 = [1,2,3], nums2 = [2,4]
// Output: 2
// Explanation: The smallest element common to both arrays is 2, so we return 2.

#[test]
fn official1() {
    assert_eq!(2, get_common(vec![1,2,3], vec![2,4]));
}

// Example 2:
// Input: nums1 = [1,2,3,6], nums2 = [2,3,4,5]
// Output: 2
// Explanation: There are two common elements in the array 2 and 3 out of which 2 is the smallest, so 2 is returned.
#[test]
fn official2() {
    assert_eq!(2, get_common(vec![1,2,3,6], vec![2,3,4,5]));
}
