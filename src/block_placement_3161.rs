use std::collections::{BTreeMap, BTreeSet};

pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut obstacles = BTreeSet::new();
    // length: positions
    let mut by_length: BTreeMap<i32, BTreeSet<i32>> = BTreeMap::new();
    let mut results = vec![];
    for query in queries {
        match query[0] {
            1 => {
                let previous = obstacles.range(..query[1]).last().copied().unwrap_or(0);
                if previous == query[1] {
                    continue;
                }
                debug_assert!(previous < query[1]);
                let next = obstacles.range(previous+1..).next().copied();
                if let Some(next) = next {
                    debug_assert!(next > query[1]);
                    let length = next - previous;
                    let removed = by_length.get_mut(&length).expect("Length was visited before").remove(&previous);
                    debug_assert!(removed);
                    debug_assert!(next - query[1] > 0);
                    by_length.entry(next - query[1]).or_default().insert(query[1]);
                }
                debug_assert!(query[1] - previous > 0);
                by_length.entry(query[1] - previous).or_default().insert(previous);
                obstacles.insert(query[1]);
            }
            2 => {
                dbg!(&obstacles, &by_length);
                let length = query[2];
                let position = query[1];
                if position >= obstacles.last().copied().unwrap_or(0) + length {
                    results.push(true);
                } else {
                    results.push(by_length.range(length..).any(|(_, positions)| positions.first().map_or(false, |&p| p + length <= position)));
                }
            }
            _ => unreachable!(),
        }
    }
    results
}


// Example 1:
// Input: queries = [[1,2],[2,3,3],[2,3,1],[2,2,2]]
// Output: [false,true,true]
// Explanation:
// For query 0, place an obstacle at x = 2. A block of size at most 2 can be placed before x = 3.

#[test]
fn official1() {
    assert_eq!(vec![false,true,true], get_results(vec![vec![1,2],vec![2,3,3],vec![2,3,1],vec![2,2,2]]));
}

// Example 2:
// Input: queries = [[1,7],[2,7,6],[1,2],[2,7,5],[2,7,6]]
// Output: [true,true,false]
// Explanation:
//     Place an obstacle at x = 7 for query 0. A block of size at most 7 can be placed before x = 7.
//     Place an obstacle at x = 2 for query 2. Now, a block of size at most 5 can be placed before x = 7, and a block of size at most 2 before x = 2.
#[test]
fn official2() {
    assert_eq!(vec![true,true,false], get_results(vec![vec![1,7],vec![2,7,6],vec![1,2],vec![2,7,5],vec![2,7,6]]));
}

#[test]
fn official432() {
    assert_eq!(vec![true,false], get_results(vec![vec![1,8],vec![1,1],vec![1,9],vec![2,5,1],vec![2,15,8]]));
}

#[test]
fn official473() {
    assert_eq!(vec![true], get_results(vec![vec![1,4],vec![2,2,1]]));
}

#[test]
fn official557() {
    assert_eq!(vec![false,true], get_results(vec![vec![1,4],vec![2,11,11],vec![1,8],vec![2,4,4]]));
}

#[test]
fn official576() {
    assert_eq!(vec![false,true], get_results(vec![vec![1,10],vec![1,8],vec![2,1,3],vec![2,6,1]]));
}

#[test]
fn t1() {
    assert_eq!(vec![true], get_results(vec![vec![1,4],vec![1,8],vec![2,4,4]]));
}

#[test]
fn t2() {
    assert_eq!(vec![true], get_results(vec![vec![1,10],vec![1,8],vec![2,6,1]]));
}
