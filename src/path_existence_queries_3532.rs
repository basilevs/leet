// https://leetcode.com/problems/path-existence-queries-in-a-graph-i

pub fn path_existence_queries(
    _: i32,
    mut nums: Vec<i32>,
    max_diff: i32,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    debug_assert!(nums.is_sorted());
    let mut component = 0;
    let mut last_value = nums[0];
    for x in nums.iter_mut() {
        if *x - last_value > max_diff {
            component += 1;
        }
        last_value = *x;
        *x = component;
    }
    queries.into_iter().map(|q| {
        let l:usize = q[0].try_into().expect("q[0] should be a valid nums index");
        let r:usize = q[1].try_into().expect("q[1] should be a valid nums index");
        if l == r {
            true
        } else {
            nums[l] == nums[r]
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

        #[test]
    fn mismatch_values_indices() {
        let n = 4;
        let nums = vec![12, 15, 16, 18];
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