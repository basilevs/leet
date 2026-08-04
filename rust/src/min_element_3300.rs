pub fn min_element(nums: Vec<i32>) -> i32 {
    nums.into_iter().map(digits_sum).min().unwrap_or(0)
}

fn digits_sum(mut i: i32) -> i32  {
    let mut sum = 0;
    while i > 0 {
        sum += i % 10;
        i /= 10;
    }
    sum
}

#[test]
fn official1() {
    assert_eq!(1, min_element(vec![10, 12, 13, 14]));
}

#[test]
fn official2() {
    assert_eq!(1, min_element(vec![1, 2, 3, 4]));
}

#[test]
fn official3() {
    assert_eq!(10, min_element(vec![999, 19, 199]));
}
