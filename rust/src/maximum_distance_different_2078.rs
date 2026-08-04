    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let len: i32 = i32::try_from(colors.len()).expect("Input is too long");
        assert!(len >= 2);
        if colors.first() != colors.last() {
            return len - 1;
        }
        let color = colors.first().unwrap().clone();
        let head_len = colors.iter().position(|x| *x != color).unwrap();
        debug_assert!(head_len > 0);
        debug_assert!(head_len < colors.len());
        let tail_len = colors.iter().rev().position(|x| *x != color).unwrap();
        debug_assert!(tail_len > 0);
        debug_assert!(tail_len < colors.len());
        len - 1 - i32::try_from(head_len.min(tail_len)).expect("Overflow")
    }

#[test]
fn official1() {
    assert_eq!(3, max_distance([1,1,1,6,1,1,1].to_vec()));
}

#[test]
fn official2() {
    assert_eq!(4, max_distance([1,8,3,8,3].to_vec()));
}

#[test]
fn official3() {
    assert_eq!(1, max_distance([0,1].to_vec()));
}

#[test]
fn longer_tail() {
    assert_eq!(5, max_distance([1,1,8,3,8,1,1,1].to_vec()));
}
