pub fn find_min(nums: Vec<i32>) -> i32 {
    let partition_value = *nums.first().unwrap();
    let i = nums.partition_point(|&x| x >= partition_value);
    if i < nums.len() {
        nums[i]
    } else {
        partition_value
    }
}

#[test]
fn official1() {
    assert_eq!(1, find_min(vec![3,4,5,1,2]));
}

#[test]
fn official2() {
    assert_eq!(0, find_min(vec![4,5,6,7,0,1,2]));
}

#[test]
fn official3() {
    assert_eq!(11, find_min(vec![11,13,15,17]));
}