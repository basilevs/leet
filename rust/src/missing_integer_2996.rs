// https://leetcode.com/problems/smallest-missing-integer-greater-than-sequential-prefix-sum

pub fn missing_integer(nums: Vec<i32>) -> i32 {
    dbg!(&nums);
    todo!("training scaffold: implement solution");
}

#[cfg(test)]
mod tests {
    use super::missing_integer;

    #[test]
    fn official1() {
        assert_eq!(6, missing_integer(vec![1, 2, 3, 2, 5]));
    }

    #[test]
    fn official2() {
        assert_eq!(15, missing_integer(vec![3, 4, 5, 1, 12, 14, 13]));
    }
}
