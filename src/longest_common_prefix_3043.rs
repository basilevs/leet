use std::collections::{HashSet};

pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let mut prefixes1 = HashSet::with_capacity(arr1.len()); // total count is larger, but at most 8 times as long
    for mut i in arr1 {
        while i > 0 {
            prefixes1.insert(i);
            i /= 10;
        }
    }
    
    arr2
        .iter()
        .copied()
        .map(|mut p| {
            while p > 0 {
                if prefixes1.contains(&p) {
                    break;
                }
                p /= 10;
            }
            p
        })
        .filter(|p| *p > 0)
        .map(|p| p.ilog10() + 1)
        .max()
        .unwrap_or(0) as i32
}


#[test]
fn official1() {
    assert_eq!(3, longest_common_prefix(vec![1,10,100], vec![1000]));
}

#[test]
fn official2() {
    assert_eq!(0, longest_common_prefix(vec![1,2,3], vec![4,4,4]));
}
