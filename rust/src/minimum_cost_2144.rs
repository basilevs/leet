pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
    cost.sort_unstable_by_key(|x| -x);
    cost.chunks(3).flat_map(|s| s[..2.min(s.len())].iter()).sum()
}
