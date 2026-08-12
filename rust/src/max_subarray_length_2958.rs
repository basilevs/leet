// https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency

use std::collections::HashMap;

pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    assert!(k >= 1);
    let k: usize = k.try_into().unwrap();
    let mut freq = HashMap::new();
    let mut front = nums.iter().enumerate();
    let mut back = nums.iter().enumerate();
    let mut longest_length = 0;
    // First window index
    let mut back_position = 0;
    // Last window index
    let mut front_position;
    'outer: loop {
        loop {
            let Some((i, &num)) = front.next() else {
                break 'outer;
            };
            let bucket: &mut usize = freq.entry(num).or_default();
            *bucket += 1;
            front_position = i;
            if *bucket > k {
                break;
            }
            longest_length = longest_length.max(front_position + 1 - back_position);
        }
        debug_assert!(back_position <= front_position);
        debug_assert_eq!(1, freq.values().filter(|&&v| v > k).count());
        loop {
            let Some((i, &num)) = back.next() else {
                break 'outer;
            };
            back_position = i + 1;
            let bucket = freq.get_mut(&num).unwrap();
            *bucket -= 1;
            if *bucket == k {
                longest_length = longest_length.max(front_position + 1 - back_position);
                break;
            }
        }
        debug_assert!(back_position <= front_position);
        debug_assert_eq!(0, freq.values().filter(|&&v| v > k).count());
    }
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
