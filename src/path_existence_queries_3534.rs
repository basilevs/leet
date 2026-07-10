// https://leetcode.com/problems/path-existence-queries-in-a-graph-ii

use std::u32;

pub fn path_existence_queries(
    n: i32,
    nums: Vec<i32>,
    max_diff: i32,
    queries: Vec<Vec<i32>>,
) -> Vec<i32> {
    let n: usize = n.try_into().expect("n should be nums.len()");
    let mut sorted = (0..n).collect::<Vec<usize>>();
    sorted.sort_unstable_by_key(|&i| nums[i]);
    debug_assert!(n == nums.len());
    let mut component = 0;
    let mut sorted_distances : Vec<u32> = vec![u32::MAX; n];
    let mut sorted_components: Vec<u32> = vec![u32::MAX; n];
    let mut directly_reachable = 0usize;
    sorted_components[0] = 0;
    sorted_distances[0] = 0;
    // iter_mut avoids repeated vector indexing
    for (i, s) in sorted.iter().enumerate().skip(1) {
        if nums[*s] - nums[sorted[i-1]] > max_diff {
            component += 1;
            directly_reachable = i;
            sorted_distances[directly_reachable] = 0;
        } else {
            while nums[sorted[directly_reachable]] + max_diff < nums[*s] {
                directly_reachable += 1;
            }
            sorted_distances[i] = sorted_distances[directly_reachable] + 1;
        }
        sorted_components[i] = component;
    }

    debug_assert!(sorted_distances.iter().all(|&d| d != u32::MAX));
    debug_assert!(sorted_components.iter().all(|&c| c != u32::MAX));
    // dbg!(&sorted, &sorted_components, &sorted_distances);

    let mut distances = vec![u32::MAX; n];
    let mut components = vec![u32::MAX; n];
    for (i, &sorted_i) in sorted.iter().enumerate() {
        distances[sorted_i] = sorted_distances[i];
        components[sorted_i] = sorted_components[i];
    }

    debug_assert!(distances.iter().all(|&d| d != u32::MAX));
    debug_assert!(components.iter().all(|&c| c != u32::MAX));

    // dbg!(max_diff, &nums, &distances, &components);
    queries.into_iter().map(|q| {
        let l:usize = q[0].try_into().expect("q[0] should be a valid nums index");
        let r:usize = q[1].try_into().expect("q[1] should be a valid nums index");
        if l == r {
            0i32
        } else {
            if components[l] == components[r] {
                distances[r].abs_diff(distances[l]).max(1).try_into().expect("distance should fit in i32")
            } else {
                -1
            }
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
        let n = 5;
        let nums = vec![1, 8, 3, 4, 2];
        let max_diff = 3;
        #[rustfmt::skip]
        let queries = [
            [0, 3],
            [2, 4],
        ];

        let expected = vec![1, 1];
        assert_eq!(expected, path_existence_queries(n, nums, max_diff, to_queries(&queries)));
    }

    #[test]
    fn official2() {
        let n = 5;
        let nums = vec![5, 3, 1, 9, 10];
        let max_diff = 2;
        #[rustfmt::skip]
        let queries = [
            [0, 1],
            [0, 2],
            [2, 3],
            [4, 3],
        ];

        let expected = vec![1, 2, -1, 1];
        assert_eq!(expected, path_existence_queries(n, nums, max_diff, to_queries(&queries)));
    }

    #[test]
    fn official3() {
        let n = 3;
        let nums = vec![3, 6, 1];
        let max_diff = 1;
        #[rustfmt::skip]
        let queries = [
            [0, 0],
            [0, 1],
            [1, 2],
        ];

        let expected = vec![0, -1, -1];
        assert_eq!(expected, path_existence_queries(n, nums, max_diff, to_queries(&queries)));
    }
}