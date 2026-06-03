use std::collections::{BTreeSet};

use itertools::{Itertools, chain};

pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut obstacles = BTreeSet::new();
    obstacles.insert(0);
    let mut results = vec![];
    for query in queries {
        match query[0] {
            1 => {
                obstacles.insert(query[1]);
            }
            2 => {
                // dbg!(&obstacles);
                let length = query[2];
                let position = query[1];
                let mut found = false;
                let max_start_position = position - length;
                let last = obstacles.last().copied().unwrap_or(0);
                if last <= max_start_position {
                    found = true;
                } else {
                    for (&prev, &next) in chain(obstacles.range(..position), [&position]).tuple_windows() {
                        let unobstructed_length = next - prev;
                        if length <= unobstructed_length {
                            found = true;
                            break;
                        }
                    }
                }
                results.push(found);
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
