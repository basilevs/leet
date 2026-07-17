// https://leetcode.com/problems/sorted-gcd-pair-queries

use std::collections::BTreeMap;

pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
    let mut gcd_stat=vec![0u64; 50001];
    for (i, &a) in nums.iter().enumerate() {
        for &b in nums.iter().skip(i+1) {
            let g = gcd(a, b);
            gcd_stat[g as usize] += 1;
        }
    }
    let mut gcd_pairs = BTreeMap::new();
    let mut acc = 0u64;
    for (g, &count) in gcd_stat.iter().enumerate() {
        if count > 0 {
            gcd_pairs.insert(acc, g);
            acc += count;
        }
    }
    queries.iter().map(|&q| {
        let q = q as u64;
        match gcd_pairs.range(..=q).next_back() {
            Some((_, &g)) => g as i32,
            None => 0,
        }
    }).collect()
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::gcd_values;

    #[test]
    fn official1() {
        let nums = vec![2, 3, 4];
        let queries = vec![0, 2, 2];
        assert_eq!(vec![1, 2, 2], gcd_values(nums, queries));
    }

    #[test]
    fn official2() {
        let nums = vec![4, 4, 2, 1];
        let queries = vec![5, 3, 1, 0];
        assert_eq!(vec![4, 2, 1, 1], gcd_values(nums, queries));
    }

    #[test]
    fn official3() {
        let nums = vec![2, 2];
        let queries = vec![0, 0];
        assert_eq!(vec![2, 2], gcd_values(nums, queries));
    }
}
