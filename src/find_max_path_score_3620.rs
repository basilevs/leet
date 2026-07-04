// https://leetcode.com/problems/network-recovery-pathways

use std::{cmp::Reverse, collections::{BinaryHeap}};

pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
    let n = online.len();
    let mut min_price= i32::MAX;
    let mut max_price= i32::MIN;
    let mut adjacent = vec![Vec::new(); n];
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        let price = edge[2];
        if online[u] && online[v] {
            adjacent[u].push((v, price));
            min_price = min_price.min(price);
            max_price = max_price.max(price);
        }
    }

    let mut queue = BinaryHeap::with_capacity(n);
    let mut node_cost = vec![i64::MAX; n];
    let mut reachable = |minimum_price: &i32| -> bool {
        queue.clear();
        node_cost.fill(i64::MAX);
        node_cost[0] = 0;
        queue.push(Reverse((0, 0)));
        while let Some(Reverse((cost, node))) = queue.pop() {
            if node == n - 1 {
                return true;
            }
            for &(next_node, price) in &adjacent[node] {
                let next_cost = cost + price as i64;
                if price >= *minimum_price && next_cost <= k  && next_cost < node_cost[next_node] {
                    node_cost[next_node] = next_cost;
                    queue.push(Reverse((next_cost, next_node)));
                }
            }
        }
        false
    };

    if !reachable(&min_price) {
        return -1;
    }

    while min_price <= max_price {
        let mid = (min_price + max_price) / 2;
        if reachable(&mid) {
            min_price = mid + 1;
        } else {
            max_price = mid - 1;
        }
    }
    
    max_price

}



#[cfg(test)]
mod tests {
    use super::find_max_path_score;

    fn to_vector<const N: usize, const M: usize>(input: [[i32; M]; N]) -> Vec<Vec<i32>> {
        input.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    fn official1() {
        #[rustfmt::skip]
        let edges = [
            [0, 1,  5],
            [1, 3, 10],
            [0, 2,  3],
            [2, 3,  4],
        ];
        assert_eq!(3, find_max_path_score(to_vector(edges), vec![true, true, true, true], 10));
    }

    #[test]
    fn official2() {
        #[rustfmt::skip]
        let edges = [
            [0, 1, 7],
            [1, 4, 5],
            [0, 2, 6],
            [2, 3, 6],
            [3, 4, 2],
            [2, 4, 6],
        ];
        assert_eq!(6, find_max_path_score(to_vector(edges), vec![true, true, true, false, true], 12));
    }

    #[test]
    fn absent_path() {
        #[rustfmt::skip]
        let edges = [
            [0, 1, 5],
            [1, 2, 7],
        ];
        assert_eq!(
            -1,
            find_max_path_score(to_vector(edges), vec![true, false, true], 20)
        );
    }
}
