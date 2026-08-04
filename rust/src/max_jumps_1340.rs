use itertools::Itertools;

pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
    let n = arr.len();
    if n < 2 {
        return 1;
    }
    let d = usize::try_from(d).expect("d should be positive");
    let mut depth = vec![0; n];
    let mut indices_by_height_desc = (0..n).collect_vec();
    indices_by_height_desc.sort_by_key(|&i| -arr[i]);
    for i in indices_by_height_desc {
        let current_depth = depth[i];
        for j in i.saturating_add(1)..=i.saturating_add(d).min(n-1) {
            if arr[j] >= arr[i] {
                break;
            }
            depth[j] = depth[j].max(current_depth + 1);
        }
        for j in (i.saturating_sub(d)..i).rev() {
            if arr[j] >= arr[i] {
                break;
            }
            depth[j] = depth[j].max(current_depth + 1);
        }
    }
    *depth.iter().max().unwrap_or(&0) + 1
}

#[test]
fn official1() {
    assert_eq!(4, max_jumps(vec![6,4,14,6,8,13,9,7,10,6,12], 2));
}

#[test]
fn official2() {
    assert_eq!(1, max_jumps(vec![3,3,3,3,3], 3));
}

#[test]
fn official3() {
    assert_eq!(7, max_jumps(vec![7,6,5,4,3,2,1], 1));
}

#[test]
fn t1() {
    assert_eq!(1, max_jumps(vec![7], 1));
}

#[test]
fn t2() {
    assert_eq!(2, max_jumps(vec![1,2], 1));
}

#[test]
fn t3() {
    assert_eq!(2, max_jumps(vec![2,1], 1));
}
