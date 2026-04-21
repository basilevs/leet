
use std::collections::HashMap;

use itertools::Itertools;

    pub fn minimum_hamming_distance(source: Vec<i32>, target: Vec<i32>, allowed_swaps: Vec<Vec<i32>>) -> i32 {
        let swaps_len = allowed_swaps.len();
        let component_roots = connected_components(source.len(), allowed_swaps);
        let mut temp: HashMap<i32, i32> = HashMap::with_capacity(swaps_len);

        let hamming_by_connected_inidices = |indices| {
            temp.clear();
            for &i in &indices {
                *temp.entry(source[i]).or_default() += 1;
            }
            for &i in &indices {
                *temp.entry(target[i]).or_default() -= 1;
            }
            temp.values().copied().map(i32::abs).sum::<i32>() / 2
        };
        
        component_roots
            .into_iter()
            .enumerate()
            .into_group_map_by(|x| x.1)
            .into_values()
            .map(|tuples| tuples.into_iter().map(|x| x.0).collect_vec())
            .map(hamming_by_connected_inidices)
            .sum()

    }
    
// https://dsar.rantai.dev/docs/part-iii/chapter-14/
fn connected_components(vertice_count: usize, edges: Vec<Vec<i32>>) -> Vec<usize> {
    let mut parents = (0..vertice_count).collect_vec();
    let mut slice = parents.as_mut_slice();
    for edge in edges {
        add_component(edge, &mut slice);
    }
    for i in 0..vertice_count {
        find_component(i, &mut parents);
    }
    parents
}

fn add_component(new_component: Vec<i32>, parents: &mut [usize]) -> usize {
    debug_assert!(new_component.len() >= 2);
    let component = find_component( into_usize(*new_component.first().unwrap()), parents);
    for &i in new_component[1..].into_iter() {
        parents[find_component(into_usize(i), parents)] = component;
    }
    component
}

fn into_usize(input: i32) -> usize {
    usize::try_from(input).expect("Input can't be negative")
}

fn find_component(x: usize, parent: &mut [usize]) -> usize {
    if parent[x] != x {
        parent[x] = find_component(parent[x], parent); // Path compression
    }
    parent[x]
}

#[test]
fn official1() {
    assert_eq!(1, minimum_hamming_distance([1,2,3,4].to_vec(), [2,1,4,5].to_vec(), to_vector(&[[0,1],[2,3]])));
}

#[test]
fn official2() {
    assert_eq!(2, minimum_hamming_distance([1,2,3,4].to_vec(), [1,3,2,4].to_vec(), vec!()));
}

#[test]
fn official3() {
    assert_eq!(0, minimum_hamming_distance([5,1,2,4,3].to_vec(), [1,5,4,2,3].to_vec(), to_vector(&[[0,4],[4,2],[1,3],[1,4]])));
}


#[cfg(test)]
fn to_vector(input: &[[i32; 2]]) -> Vec<Vec<i32>> {
    input.iter().map(Vec::from).collect()
}