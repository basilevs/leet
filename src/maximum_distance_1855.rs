    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut max = 0;
        let min_len = nums1.len().min(nums2.len());
        while i < min_len {
            let value = nums1[i];
            let partition_point = nums2[i..].partition_point(|x| *x >= value);
            max = max.max(partition_point.saturating_sub(1));
            if i + partition_point >= nums2.len() {
                break;
            }
            debug_assert!(partition_point == 0 || nums2[i + partition_point - 1] >= value);
            debug_assert!(nums2[i + partition_point] < value);
            let value2 = nums2[i];
            let partition_point = nums1[i+1..].partition_point(|x| *x > value2);
            i += partition_point + 1;
            debug_assert!(partition_point == 0 || nums1[i - 1] > value2);
            debug_assert!( i >= nums1.len() || nums1[i] <= value2);
        }
        max.try_into().expect("overflow")
    }

#[test]
fn official1() {
    assert_eq!(2, max_distance([55,30,5,4,2].to_vec(), [100,20,10,10,5].to_vec()));
}

#[test]
fn official2() {
    assert_eq!(1, max_distance([2,2,2].to_vec(), [10,10,1].to_vec()));
}

#[test]
fn official3() {
    assert_eq!(2, max_distance([30,29,19,5].to_vec(), [25,25,25,25,25].to_vec()));
}

#[test]
fn testcase10() {
    assert_eq!(10, max_distance([9820,8937,7936,4855,4830,4122,2327,1342,1167,815,414].to_vec(), [9889,9817,9800,9777,9670,9646,9304,8977,8974,8802,8626,8622,8456].to_vec()));
}

#[test]
fn regression_skip_i_can_hide_optimum() {
    assert_eq!(1, max_distance([4, 3].to_vec(), [3, 3, 3].to_vec()));
}