// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number

pub fn smaller_numbers_than_current(mut nums: Vec<i32>) -> Vec<i32> {
    let mut sorted = nums.clone();
    sorted.sort_unstable();

    for i in nums.iter_mut() {
        *i = sorted.partition_point(|&x| x < *i) as i32;
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::smaller_numbers_than_current;

    #[test]
    fn official1() {
        assert_eq!(
            vec![4, 0, 1, 1, 3],
            smaller_numbers_than_current(vec![8, 1, 2, 2, 3])
        );
    }

    #[test]
    fn official2() {
        assert_eq!(
            vec![2, 1, 0, 3],
            smaller_numbers_than_current(vec![6, 5, 4, 8])
        );
    }

    #[test]
    fn official3() {
        assert_eq!(
            vec![0, 0, 0, 0],
            smaller_numbers_than_current(vec![7, 7, 7, 7])
        );
    }
}
