pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let partition_value = *nums.first().unwrap();
    let partition_index = nums.partition_point(|&x| x >= partition_value);
    for range in [0..partition_index, partition_index..nums.len()] {
        let start = range.start;
        if let Some(i) = nums[range].binary_search(&target).ok() {
            return (start + i) as i32;
        }
    }
    -1
}

#[test]
fn official1() {
    assert_eq!(4, search(vec![4,5,6,7,0,1,2], 0));
}

#[test]
fn official2() {
    assert_eq!(-1, search(vec![4,5,6,7,0,1,2], 3));
}

#[test]
fn official3() {
    assert_eq!(-1, search(vec![1], 0));
}

#[test]
fn official193() {
    assert_eq!(0, search(vec![3, 1], 3));
}

#[test]
fn t1() {
    assert_eq!(2, search(vec![1, 2, 3, 4], 3));
}

#[test]
fn t2() {
    assert_eq!(3, search(vec![1, 2, 3, 4], 4));
}

#[test]
fn t3() {
    assert_eq!(0, search(vec![1, 2, 3, 4], 1));
}

#[test]
fn t4() {
    assert_eq!(-1, search(vec![1, 2, 3, 4], 5));
}


