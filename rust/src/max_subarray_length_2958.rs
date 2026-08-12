// https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency

use std::collections::HashMap;

pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    assert!(k >= 1);
    let k: usize = k.try_into().unwrap();
    let mut freq: HashMap<i32, usize> = HashMap::new();
    // Elements leaving the window.
    let mut back = nums.iter();
    let mut length = 0usize;
    // Each element maps to the longest good window ending on it.
    let longest_length = nums
        .iter()
        .map(|&num| {
            let bucket: &mut usize = freq.entry(num).or_default();
            *bucket += 1;
            length += 1;
            if *bucket > k {
                // Only `num` can exceed `k`, and by exactly one, so shrinking
                // until its earliest occurrence leaves the window is enough.
                loop {
                    let &evicted = back.next().expect("window is not empty");
                    *freq.get_mut(&evicted).unwrap() -= 1;
                    length -= 1;
                    if evicted == num {
                        break;
                    }
                }
            }
            debug_assert_eq!(0, freq.values().filter(|&&v| v > k).count());
            length
        })
        .max()
        .unwrap_or(0);
    i32::try_from(longest_length).unwrap_or_else(|_| {
        panic!("longest_length should be less than 10^5 but was: {longest_length}")
    })
}

#[cfg(test)]
mod tests {
    use super::max_subarray_length;

    #[test]
    fn official1() {
        assert_eq!(6, max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2));
    }

    #[test]
    fn official2() {
        assert_eq!(2, max_subarray_length(vec![1, 2, 1, 2, 1, 2, 1, 2], 1));
    }

    #[test]
    fn official3() {
        assert_eq!(4, max_subarray_length(vec![5, 5, 5, 5, 5, 5, 5], 4));
    }

    // The officials all evict at least once, so they never measure a window
    // that still starts at index 0. These do.
    #[test]
    fn single_element() {
        assert_eq!(1, max_subarray_length(vec![1], 1));
    }

    #[test]
    fn whole_array_is_good() {
        assert_eq!(3, max_subarray_length(vec![1, 2, 3], 3));
    }

    #[test]
    fn longest_window_is_a_prefix() {
        assert_eq!(4, max_subarray_length(vec![2, 3, 1, 1, 1], 2));
    }

    #[test]
    fn longest_window_is_a_suffix() {
        assert_eq!(4, max_subarray_length(vec![1, 1, 1, 2, 3, 1, 1], 2));
    }
}
