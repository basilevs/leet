// https://leetcode.com/problems/remove-methods-from-project

pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
    let components = connected_components(n as usize, &invocations);
    let k_component = components[k as usize];
    let k_component_size = components.iter().filter(|&&c| c == k_component).count();
    
    let mut graph = vec![vec![]; n as usize];
    for edge in invocations {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        graph[u].push(v);
    }
    
    //dfs
    let mut stack = vec![k as usize];
    let mut counter = 0;
    let mut visited = vec![false; n as usize];
    visited[k as usize] = true;
    while let Some(node) = stack.pop() {
        counter += 1;
        for &neighbor in &graph[node] {
            debug_assert!(components[neighbor] == k_component);
            if !visited[neighbor] {
                visited[neighbor] = true;
                stack.push(neighbor);
            }
        }
    }

    if k_component_size == counter {
        (0..n).filter(|&x| components[x as usize] != k_component).collect()
    } else {
        (0..n).collect()
    }

}

// https://dsar.rantai.dev/docs/part-iii/chapter-14/
fn connected_components(vertice_count: usize, roads: &Vec<Vec<i32>>) -> Vec<usize> {
    let mut parents: Vec<usize> = (0..vertice_count).collect();
    let slice = parents.as_mut_slice();
    for edge in roads {
        add_component(edge, slice);
    }
    for i in 0..vertice_count {
        find_component(i, &mut parents);
    }
    parents
}

fn add_component(new_component: &[i32], parents: &mut [usize]) -> usize {
    debug_assert!(new_component.len() >= 2);
    let component = find_component( into_usize(*new_component.first().unwrap()), parents);
    for &i in new_component[1..].iter() {
        parents[find_component(into_usize(i), parents)] = component;
    }
    component
}

fn find_component(x: usize, parent: &mut [usize]) -> usize {
    let mut root = x;
    while parent[root] != root {
        root = parent[root];
    }
    // Path compression
    let mut cur = x;
    while parent[cur] != root {
        let next = parent[cur];
        parent[cur] = root; 
        cur = next;
    }
    root
}

fn into_usize(input: i32) -> usize {
    usize::try_from(input).expect("Input can't be negative")
}


#[cfg(test)]
mod tests {
    use super::remaining_methods;

    #[test]
    fn official1() {
        let n = 4;
        let k = 1;
        let invocations = vec![
            vec![1, 2],
            vec![0, 1],
            vec![3, 2],
        ];
        assert_eq!(vec![0, 1, 2, 3], remaining_methods(n, k, invocations));
    }

    #[test]
    fn official2() {
        let n = 5;
        let k = 0;
        let invocations = vec![
            vec![1, 2],
            vec![0, 2],
            vec![0, 1],
            vec![3, 4],
        ];
        assert_eq!(vec![3, 4], remaining_methods(n, k, invocations));
    }

    #[test]
    fn official3() {
        let n = 3;
        let k = 2;
        let invocations = vec![
            vec![1, 2],
            vec![0, 1],
            vec![2, 0],
        ];
        let expected: Vec<i32> = vec![];
        assert_eq!(expected, remaining_methods(n, k, invocations));
    }
}
