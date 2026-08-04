pub fn find_min(nums: Vec<i32>) -> i32 {
    let partition_value = *nums.first().unwrap();
    let prefix_len = nums.iter().take_while(|&x| *x == partition_value).count();
    let i = nums[prefix_len..].partition_point(|&x| x > partition_value) + prefix_len;
    if i < nums.len() {
        partition_value.min(nums[i])
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

#[test]
fn official2_154() {
    assert_eq!(0, find_min(vec![2,2,2,0,1]));
}

#[test]
fn t1() {
    assert_eq!(0, find_min(vec![0,0,0,0,0,0,0,0,0,0,0,0,1,1,2,2,2,0,0,0,0,0,0]));
}

#[test]
fn t2() {
    assert_eq!(0, find_min(vec![2,2,2,0,1,2,2,2]));
}


#[test]
fn official183() {
    assert_eq!(1, find_min(vec![3,1,3,3]));
}

