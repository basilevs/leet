// https://leetcode.com/problems/find-missing-elements

use itertools::Itertools;

pub fn find_missing_elements(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    nums.into_iter()
        .tuple_windows()
        .flat_map(|(a, b)| (a + 1)..b)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::find_missing_elements;

    #[test]
    fn official1() {
        assert_eq!(vec![3], find_missing_elements(vec![1, 4, 2, 5]));
    }

    #[test]
    fn official2() {
        assert_eq!(vec![] as Vec<i32>, find_missing_elements(vec![7, 8, 6, 9]));
    }

    #[test]
    fn official3() {
        assert_eq!(vec![2, 3, 4], find_missing_elements(vec![5, 1]));
    }
}
