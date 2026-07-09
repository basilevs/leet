// https://leetcode.com/problems/path-existence-queries-in-a-graph-i

use std::collections::BTreeSet;

pub fn path_existence_queries(
    _: i32,
    nums: Vec<i32>,
    max_diff: i32,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    debug_assert!(nums.is_sorted());
    let gaps: BTreeSet<i32> = nums.windows(2).filter(|w| w[1] - w[0] > max_diff).map(|w| w[0]).collect();
    queries.into_iter().map(|q| {
        let (l, r) = (q[0], q[1]);
        if l == r {
            true
        } else {
            let (l, r) = (nums[l as usize], nums[r as usize]);
            gaps.range(l..r).next().is_none()
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::path_existence_queries;

    fn to_queries(input: &[[i32; 2]]) -> Vec<Vec<i32>> {
        input.iter().map(Vec::from).collect()
    }

    #[test]
    fn official1() {
        let n = 2;
        let nums = vec![1, 3];
        let max_diff = 1;
        #[rustfmt::skip]
        let queries = [
            [0, 0],
            [0, 1],
        ];

        let expected = vec![true, false];
        assert_eq!(
            expected,
            path_existence_queries(n, nums, max_diff, to_queries(&queries))
        );
    }

    #[test]
    fn official2() {
        let n = 4;
        let nums = vec![2, 5, 6, 8];
        let max_diff = 2;
        #[rustfmt::skip]
        let queries = [
            [0, 1],
            [0, 2],
            [1, 3],
            [2, 3],
        ];

        let expected = vec![false, false, true, true];
        assert_eq!(
            expected,
            path_existence_queries(n, nums, max_diff, to_queries(&queries))
        );
    }
}