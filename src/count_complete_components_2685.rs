// https://leetcode.com/problems/count-the-number-of-complete-components

use std::collections::HashMap;

pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n: usize = n.try_into().expect("n >= 1");
    let mut dsu = connected_components(n, &edges);
    let mut vertice_count = HashMap::new();
    dsu.iter().for_each(|&x| *vertice_count.entry(x).or_insert(0) += 1);
    let mut edge_count = HashMap::new();
    edges.iter().for_each(|edge| {
        let component = find_component(into_usize(edge[0]), &mut dsu);
        *edge_count.entry(component).or_insert(0) += 1usize;
    });
    vertice_count.into_iter().filter(|(component, count)| {
        let edges = edge_count.get(component).copied().unwrap_or(0);
        edges == count * (count - 1) / 2
    }).count() as i32
}

// https://dsar.rantai.dev/docs/part-iii/chapter-14/
fn connected_components(vertice_count: usize, roads: &Vec<Vec<i32>>) -> Vec<usize> {
    let mut parents: Vec<usize> = (0..vertice_count).collect();
    let mut slice = parents.as_mut_slice();
    for edge in roads {
        add_component(&edge, &mut slice);
    }
    for i in 0..vertice_count {
        find_component(i, &mut parents);
    }
    parents
}

fn add_component(new_component: &[i32], parents: &mut [usize]) -> usize {
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


#[cfg(test)]
mod tests {
    use super::count_complete_components;

    fn to_vector<const N: usize, const M: usize>(input: [[i32; M]; N]) -> Vec<Vec<i32>> {
        input.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    fn official1() {
        let n = 6;
        #[rustfmt::skip]
        let edges = [
            [0, 1],
            [0, 2],
            [1, 2],
            [3, 4],
        ];
        assert_eq!(3, count_complete_components(n, to_vector(edges)));
    }

    #[test]
    fn official2() {
        let n = 6;
        #[rustfmt::skip]
        let edges = [
            [0, 1],
            [0, 2],
            [1, 2],
            [3, 4],
            [3, 5],
        ];
        assert_eq!(1, count_complete_components(n, to_vector(edges)));
    }
}
