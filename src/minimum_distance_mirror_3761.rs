// https://leetcode.com/problems/minimum-absolute-distance-between-mirror-pairs?envType=daily-question&envId=2026-04-16https://leetcode.com/problems/minimum-absolute-distance-between-mirror-pairs?envType=daily-question&envId=2026-04-16

use std::collections::HashMap;


    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let mut last_position_by_value: HashMap<i32, usize> = HashMap::new();
        nums.into_iter().enumerate().filter_map(|(position, value)| {
            let value = remove_trailing_zeros(value);
            let mirror = mirror(value);
            let prev_position_option = last_position_by_value.get(&mirror).copied();
            last_position_by_value.insert(value, position);
            // dbg!(&position, &value, &mirror, &prev_position_option);
            prev_position_option.map(|p| (position - p) as i32)
        }).min().unwrap_or(-1)
    }

#[inline]
fn mirror(mut input:i32) -> i32 {
    let mut result = 0;
    while input > 0 {
        result = result * 10 + input % 10;
        input /= 10
    }
    result
}

#[inline]
fn remove_trailing_zeros(mut input:i32) -> i32 {
    while input > 0 && input % 10 == 0 {
        input /= 10;
    }
    input
}

#[test]
fn official1() {
    assert_eq!(1, min_mirror_pair_distance([12,21,45,33,54].to_vec()));
}

#[test]
fn official2() {
    assert_eq!(1, min_mirror_pair_distance([120,21].to_vec()));
    assert_eq!(1, min_mirror_pair_distance([21,120].to_vec()));
}