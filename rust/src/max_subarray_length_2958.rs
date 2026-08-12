// https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency

use std::collections::HashMap;

pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    assert!(k >= 1);
    let k: usize = k.try_into().unwrap();
    let mut freq = HashMap::new();
    let mut front = nums.iter().enumerate();
    let mut back = nums.iter().enumerate();
    let mut longest_length = 0;
    let mut back_position = 0;
    let mut front_position ;
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
            longest_length = longest_length.max(front_position - back_position);
        }
        debug_assert_eq!(1, freq.values().filter(|&&v| v > k).count());
        loop {
            let Some((i, &num)) = back.next() else {
                break 'outer;
            };
            back_position = i;
            let bucket = freq.get_mut(&num).unwrap();
            *bucket -= 1;
            if *bucket == k {
                longest_length = longest_length.max(front_position - back_position);
                break;
            }
        }
        debug_assert_eq!(0, freq.values().filter(|&&v| v > k).count());
    }
    i32::try_from(longest_length).expect("longest_length should be less than i32::MAX but was: {longest_length}")
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
}
