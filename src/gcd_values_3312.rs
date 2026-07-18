// https://leetcode.com/problems/sorted-gcd-pair-queries

use std::{debug_assert, sync::OnceLock};

pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
    let mut gcd_stat=vec![0u64; 50001];
    for (i, &a) in nums.iter().enumerate() {
        for &b in nums.iter().skip(i+1) {
            let g = gcd_optimized(a, b);
            gcd_stat[g as usize] += 1;
        }
    }
    // Visit queries in ascending order (via sorted indices) so we can sweep gcd_stat
    // once with a running count instead of materializing prefix sums.
    let mut order: Vec<usize> = (0..queries.len()).collect();
    order.sort_unstable_by_key(|&i| queries[i]);
    let mut ans = vec![0i32; order.len()];
    let mut g = 0usize;
    let mut acc = 0u64;
    for &qi in &order {
        let q = queries[qi] as u64;
        while acc <= q {
            acc += gcd_stat[g];
            g += 1;
        }
        ans[qi] = (g - 1) as i32;
    }
    ans
}

static GCD_TABLE: OnceLock<Vec<Vec<i32>>> = OnceLock::new();

fn gcd_optimized(mut a: i32, mut b: i32) -> i32 {
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    let table = GCD_TABLE.get_or_init(|| compute_gcd_table(5000));
    while b > 0 && ((a as usize) + 1 >= table.len()) {
        let temp = a;
        a = b;
        b = temp % b;
    }
    if b == 0 {
        return a;
    }
    debug_assert!(b <= a);
    table[b as usize][a as usize - b as usize]
}

fn compute_gcd_table(m: usize) -> Vec<Vec<i32>> {
    #[allow(clippy::needless_range_loop)]
    {
        let mut gcd_table = vec![vec![]; m + 1];
        gcd_table[0] = (0i32..=m as i32).collect();
        for i in 1..=m {
            let row = &mut gcd_table[i];
            for j in i..=m {
                row.push(gcd(i as i32, j as i32));
            }
        }
        gcd_table
    }
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
    use super::gcd_optimized;

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

    #[test]
    fn edge_all_coprime() {
        // All numbers are coprime (GCD of any pair is 1)
        let nums = vec![3, 5, 7];
        let queries = vec![0, 1, 2];
        assert_eq!(vec![1, 1, 1], gcd_values(nums, queries));
    }

    #[test]
    fn edge_all_same() {
        // All numbers are identical
        let nums = vec![6, 6, 6];
        let queries = vec![0, 1, 2];
        assert_eq!(vec![6, 6, 6], gcd_values(nums, queries));
    }

    #[test]
    fn over_large() {
        let nums = vec![6000, 6000, 2];
        let queries = vec![0, 1, 2];
        assert_eq!(vec![2, 2, 6000], gcd_values(nums, queries));
    }

        #[test]
    fn edge_large() {
        let nums = vec![5000, 5001, 5003];
        let queries = vec![0, 1, 2];
        assert_eq!(vec![1, 1, 1], gcd_values(nums, queries));
    }

    #[test]
    fn gcd_edge() {
        assert_eq!(5001, gcd_optimized(5001, 10002));
        assert_eq!(1, gcd_optimized(5001, 1));
    }

}
