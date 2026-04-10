use std::array;

pub fn minimum_distance(nums: Vec<i32>) -> i32 {
    let mut last: [Vec<usize>; 100] = array::from_fn(|_| Vec::with_capacity(2));
    (0..nums.len()).map(|k| {
        let value = nums[k];
        let indices = last.get_mut((value-1) as usize).expect("value is greater than 100");
        if indices.len() < 2 {
            indices.push(k);
            return None
        }
        let i = indices[0];
        let j = indices[1];
        let d = (i.abs_diff(j) + j.abs_diff(k) + k.abs_diff(i)).try_into().expect("Distance is too large");
        indices.remove(0);
        indices.push(k);
        Some(d)
    }).flatten().min().unwrap_or(-1)
}

#[test]
fn official1() {
    let input = [1,2,1,1,3];
    assert_eq!(6, minimum_distance(Vec::from(input)));
}

#[test]
fn official2() {
    let input = [1,1,2,3,2,1,2];
    assert_eq!(8, minimum_distance(Vec::from(input)));
}

#[test]
fn official3() {
    let input = [1];
    assert_eq!(-1, minimum_distance(Vec::from(input)));
}

#[test]
fn push_out() {
    let input = [1,1,2,3,2,1,2,1,1,1];
    assert_eq!(4, minimum_distance(Vec::from(input)));
}