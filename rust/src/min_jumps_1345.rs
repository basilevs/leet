use std::collections::{HashMap, VecDeque};
use itertools::chain;

    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        if arr.len() <= 1 {
            return 0;
        }
        let mut index: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &v) in arr.iter().enumerate() {
            index.entry(v).or_default().push(i);
        }
        let end = arr.len() - 1;
        let mut queue = VecDeque::new();
        queue.push_back(0_usize);
        let mut distances:Vec<u16> = vec![u16::MAX; arr.len()];
        distances[0] = 0;
        loop {
            let current:usize = queue.pop_front().expect("can't find the path");
            let neighbor_options = [current.checked_sub(1), current.checked_add(1).filter(|&x| x <= end)];
            let teleports: Vec<usize> = index.remove(&arr[current]).unwrap_or_default();
            let next_distance = distances[current] + 1;

            for &i in chain(neighbor_options.iter().flatten(), &teleports) {
                if i == end {
                    return next_distance as _;
                }
                if distances[i] == u16::MAX {
                    distances[i] = next_distance;
                    queue.push_back(i);
                }
            }

        }
    
    }

#[cfg(test)]
fn min_jumps_arr<const N: usize>(arr: [i32; N]) -> i32 {
    min_jumps(arr.to_vec())
}

#[test]
fn official1() {
    assert_eq!(3, min_jumps_arr([100, -23, -23, 404, 100, 23, 23, 23, 3, 404]));
}

#[test]
fn official2() {
    assert_eq!(0, min_jumps_arr([7]));
}

#[test]
fn official3() {
    assert_eq!(1, min_jumps_arr([7, 6, 9, 6, 9, 6, 9, 7]));
}

#[test]
fn adjacent_only_path() {
    assert_eq!(4, min_jumps_arr([1, 2, 3, 4, 5]));
}

#[test]
fn all_values_same() {
    assert_eq!(1, min_jumps_arr([5, 5, 5, 5]));
}

#[test]
fn negative_values_same_jump() {
    assert_eq!(1, min_jumps_arr([-1, -2, -3, -1]));
}

#[test]
fn value_bounds_still_work() {
    assert_eq!(1, min_jumps_arr([100_000_000, -100_000_000, 100_000_000]));
}

