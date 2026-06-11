// https://leetcode.com/problems/number-of-ways-to-assign-edge-weights-i

use std::{collections::HashSet, mem::swap};

pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
    let mut adjacent = vec![Vec::new(); edges.len()*2+1]; // pessimistic allocation
    let mut n = 0_usize;
    for edge in edges {
        if let &[u, v] = edge.as_slice() {
            adjacent[u as usize].push(v); // first element unused
            adjacent[v as usize].push(u);
            n = n.max(u as usize).max(v as usize);
        }
    }
    
    let mut depth = 0_u32;

    // BFS
    let mut visited = vec![false; n + 1]; // first element unused
    let mut current_layer = HashSet::with_capacity(n / 4);
    current_layer.insert(1i32);
    visited[0..=1].fill(true);
    let mut next_layer = HashSet::with_capacity(n / 4);

    loop {
        for &node in current_layer.iter() {
            debug_assert!(node > 0);
            for &child in &adjacent[node as usize] {
                let v = &mut visited[child as usize];
                if !*v {
                    next_layer.insert(child);
                    *v = true;
                }
            }
        }
        if next_layer.is_empty() {
            break;
        }
        depth += 1;
        current_layer.clear();
        swap(&mut current_layer, &mut next_layer);
    }
    if depth < 1 {
        0
    } else {
        let mut result = 1;
        let modulo = 10i32.pow(9)+7;
        for _ in 1..depth {
            result *= 2;
            result %= modulo;
        }
        result
    }

}

#[test]
fn official1() {
    assert_eq!(1, assign_edge_weights(vec![vec![1, 2]]));
}

#[test]
fn official2() {
    assert_eq!(2, assign_edge_weights(vec![vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]]));
}

#[test]
fn deep_path() {
    // Path 1-2-3-...-n at the constraint boundary n = 1e5: max depth = n - 1 = 99_999.
    // Spec answer = 2^(depth-1) mod (1e9+7) = 2^99_998 mod (1e9+7) = 151_930_880.
    let n = 100_000;
    let edges: Vec<Vec<i32>> = (1..n).map(|i| vec![i, i + 1]).collect();
    assert_eq!(151_930_880, assign_edge_weights(edges));
}

