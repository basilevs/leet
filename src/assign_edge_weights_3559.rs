
use std::mem::swap;
use itertools::chain;

// https://leetcode.com/problems/number-of-ways-to-assign-edge-weights-ii
use crate::{solution::Solution, sparse_table::SparseTable};
impl Solution {
const MOD: i64 = 1_000_000_007;

    fn qpow(x: usize, mut y: usize) -> i32 {
        let mut res = 1_i64;
        let mut x = x as i64 % Self::MOD;
        while y > 0 {
            if y & 1 == 1 {
                res = (res * x) % Self::MOD;
            }
            x = (x * x) % Self::MOD;
            y >>= 1;
        }
        i32::try_from(res).unwrap()
    }

    /// Returns nodeid and its depth in euler traversal order
    fn tree_euler_tour(adjacent: &Vec<Vec<usize>>, cur:usize, prev: usize, depth:usize) -> Box<dyn Iterator<Item=(usize, usize)> + '_ > {
        let result = adjacent[cur]
            .iter()
            .filter(move |&x| *x != prev)
            .map(move |&x| 
                chain(Self::tree_euler_tour(adjacent, x, cur, depth + 1), Some((cur, depth))))
            .flatten();
        Box::new(chain(Some((cur, depth)), result))
    }

    pub fn assign_edge_weights(edges: Vec<Vec<i32>>,  queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n + 1];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            g[u].push(v);
            g[v].push(u);
        }

        let mut first: Vec<Option<usize>>= vec![None; n + 1];
        let mut depths = Vec::with_capacity(2 * n);
        for (i, (node, depth)) in Self::tree_euler_tour(&g, 1, 0, 0).enumerate() {
            first[node].get_or_insert(i);
            depths.push(depth);
        }

        let depths_table = SparseTable::new(depths.clone(), usize::min);
        queries.iter().map(|q| {
            let mut a = first[q[0] as usize].unwrap();
            let mut b = first[q[1] as usize].unwrap();
            if a == b {
                0
            } else {
                if a > b {
                    swap(&mut a, &mut b);
                }
                let ancestor_depth = depths_table.query(a..(b+1));
                let a_depth = depths[a];
                let b_depth = depths[b];
                let diff = a_depth - ancestor_depth + b_depth - ancestor_depth;
                Self::qpow(2, diff - 1)
            }

        }).collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;
fn to_vector(input: &[[i32; 2]]) -> Vec<Vec<i32>> {
    input.iter().map(Vec::from).collect()
}

#[test]
fn official1() {
    assert_eq!(
        vec![0, 1],
        Solution::assign_edge_weights(vec![vec![1, 2]], vec![vec![1, 1], vec![1, 2]])
    );
}

#[test]
fn official2() {
    assert_eq!(
        vec![2, 1, 4],
        Solution::assign_edge_weights(
            vec![vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]],
            vec![vec![1, 4], vec![3, 4], vec![2, 5]],
        )
    );
}

#[test]
fn official584() {
    let (edges, queries, expected) = load("assign_edge_weights_3559_test584.txt");
    assert_eq!(
        expected.to_vec(),
        Solution::assign_edge_weights(
            to_vector(&edges),
            to_vector(&queries),
        )
    );
}

#[test]
fn official585() {
    let (edges, queries, expected) = load("assign_edge_weights_3559_test585.txt");
    assert_eq!(
        expected.to_vec(),
        Solution::assign_edge_weights(
            to_vector(&edges),
            to_vector(&queries),
        )
    );
}

}