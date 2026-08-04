// https://leetcode.com/problems/find-missing-elements

use itertools::Itertools;

pub fn find_missing_elements(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    nums.into_iter()
        .tuple_windows()
        .flat_map(|(a, b)| (a + 1)..b)
        .collect()
}

/// Alternative: mark presence in a `u64` bitset, then scan the full `[min, max]`
/// range collecting values whose bit is unset.
pub fn find_missing_elements_bitset(nums: Vec<i32>) -> Vec<i32> {
    let (&min, &max) = (
        nums.iter().min().unwrap(),
        nums.iter().max().unwrap(),
    );
    let span = (max - min + 1) as usize;
    let mut bits = vec![0u64; span.div_ceil(64)];
    for &v in &nums {
        let i = (v - min) as usize;
        bits[i / 64] |= 1u64 << (i % 64);
    }
    (min..=max)
        .filter(|&v| {
            let i = (v - min) as usize;
            bits[i / 64] & (1u64 << (i % 64)) == 0
        })
        .collect()
}

/// Alternative: mark presence in a boolean array, then scan the full `[min, max]`
/// range collecting values that are still `false`.
pub fn find_missing_elements_bool(nums: Vec<i32>) -> Vec<i32> {
    let (&min, &max) = (
        nums.iter().min().unwrap(),
        nums.iter().max().unwrap(),
    );
    let span = (max - min + 1) as usize;
    let mut present = vec![false; span];
    for &v in &nums {
        present[(v - min) as usize] = true;
    }
    (min..=max)
        .filter(|&v| !present[(v - min) as usize])
        .collect()
}

/// Alternative: sort, then imperatively push every value strictly between
/// consecutive elements.
pub fn find_missing_elements_loop(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    let mut ans = Vec::new();
    for i in 0..nums.len() - 1 {
        for x in nums[i] + 1..nums[i + 1] {
            ans.push(x);
        }
    }
    ans
}

/// Alternative: pack presence into a single `u128` bitmask, then walk from the
/// lowest set bit to the highest, emitting positions whose bit is unset. Only
/// valid when all values fit in `0..128` (true for this problem: `1 <= v <= 100`).
pub fn find_missing_elements_u128(nums: Vec<i32>) -> Vec<i32> {
    let b = nums.into_iter().fold(0u128, |f, n| f | 1 << n);

    std::iter::successors(Some(b), |n| Some(n >> 1))
        .skip(b.trailing_zeros() as _)
        .zip(b.trailing_zeros() as i32..127 - b.leading_zeros() as i32)
        .filter_map(|(b, m)| (b & 1 == 0).then_some(m))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        find_missing_elements, find_missing_elements_bitset,
        find_missing_elements_bool, find_missing_elements_loop,
        find_missing_elements_u128,
    };

    #[test]
    fn official1() {
        assert_eq!(vec![3], find_missing_elements(vec![1, 4, 2, 5]));
    }

    #[test]
    fn official2() {
        assert_eq!(vec![] as Vec<i32>, find_missing_elements(vec![7, 8, 6, 9]));
    }

    #[test]
    fn official3() {
        assert_eq!(vec![2, 3, 4], find_missing_elements(vec![5, 1]));
    }

    #[test]
    fn bitset_matches_official() {
        assert_eq!(vec![3], find_missing_elements_bitset(vec![1, 4, 2, 5]));
        assert_eq!(vec![] as Vec<i32>, find_missing_elements_bitset(vec![7, 8, 6, 9]));
        assert_eq!(vec![2, 3, 4], find_missing_elements_bitset(vec![5, 1]));
    }

    #[test]
    fn bool_matches_official() {
        assert_eq!(vec![3], find_missing_elements_bool(vec![1, 4, 2, 5]));
        assert_eq!(vec![] as Vec<i32>, find_missing_elements_bool(vec![7, 8, 6, 9]));
        assert_eq!(vec![2, 3, 4], find_missing_elements_bool(vec![5, 1]));
    }

    #[test]
    fn u128_matches_official() {
        assert_eq!(vec![3], find_missing_elements_u128(vec![1, 4, 2, 5]));
        assert_eq!(vec![] as Vec<i32>, find_missing_elements_u128(vec![7, 8, 6, 9]));
        assert_eq!(vec![2, 3, 4], find_missing_elements_u128(vec![5, 1]));
    }

    #[test]
    fn loop_matches_official() {
        assert_eq!(vec![3], find_missing_elements_loop(vec![1, 4, 2, 5]));
        assert_eq!(vec![] as Vec<i32>, find_missing_elements_loop(vec![7, 8, 6, 9]));
        assert_eq!(vec![2, 3, 4], find_missing_elements_loop(vec![5, 1]));
    }
}
