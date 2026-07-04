// https://leetcode.com/problems/minimum-score-of-a-path-between-two-cities

pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let components = connected_components(n as usize, &roads);
    assert_eq!(components[0], components[(n - 1) as usize], "Cities 1 and n must be connected");
    let target_component = components[0];
    roads
        .into_iter()
        .filter(|road| components[road[0] as usize - 1] == target_component)
        .map(|road| road[2])
        .min()
        .unwrap()
}

// https://dsar.rantai.dev/docs/part-iii/chapter-14/
fn connected_components(vertice_count: usize, roads: &Vec<Vec<i32>>) -> Vec<usize> {
    let mut parents: Vec<usize> = (0..vertice_count).collect();
    let mut slice = parents.as_mut_slice();
    let mut tmp = vec![0; 2];
    for edge in roads {
        tmp[0] = edge[0] - 1;
        tmp[1] = edge[1] - 1;
        add_component(&tmp, &mut slice);
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
    use super::min_score;

    fn to_vector<const N: usize, const M: usize>(input: [[i32; M]; N]) -> Vec<Vec<i32>> {
        input.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    fn official1() {
        #[rustfmt::skip]
        let roads = [
            [1, 2, 9],
            [2, 3, 6],
            [2, 4, 5],
            [1, 4, 7],
        ];
        assert_eq!(5, min_score(4, to_vector(roads)));
    }

    #[test]
    fn official2() {
        #[rustfmt::skip]
        let roads = [
            [1, 2, 2],
            [1, 3, 4],
            [3, 4, 7],
        ];
        assert_eq!(2, min_score(4, to_vector(roads)));
    }
}
