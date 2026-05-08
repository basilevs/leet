    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        
    }

#[test]
fn official1() {
    assert_eq!(2, min_jumps(vec![1,2,4,6]));
}

#[test]
fn official2() {
    assert_eq!(2, min_jumps(vec![2,3,4,7,9]));
}

#[test]
fn official3() {
    assert_eq!(3, min_jumps(vec![4,6,5,8]));
}
