// https://leetcode.com/problems/longest-subsequence-with-non-zero-bitwise-xor

pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    let mut total_xor = 0;
    let mut non_zero = false;
    for &i in &nums {
        total_xor ^= i;
        non_zero |= i != 0;
    }

    if total_xor != 0 {
        return n as i32;
    }

    if !non_zero {
        return 0;
    }

    n.saturating_sub(1) as i32
}

#[cfg(test)]
mod tests {
    use super::longest_subsequence;
    use itertools::Itertools;

    #[test]
    fn official1() {
        assert_eq!(2, longest_subsequence(vec![1, 2, 3]));
    }

    #[test]
    fn official2() {
        assert_eq!(3, longest_subsequence(vec![2, 3, 4]));
    }

    fn xor_all(nums: impl Iterator<Item=i32>) -> i32 {
        nums.fold(0, |acc, x| acc ^ x)
    }

    fn naive_longest_subsequence(nums: &[i32]) -> i32 {
        let mut max_length = 0;
        for len in 1..=nums.len() {
            for subseq in nums.iter().combinations(len) {
                if xor_all(subseq.into_iter().copied()) != 0 {
                    max_length = max_length.max(len);
                }
            }
        }
        max_length as i32
    }

    #[test]
    fn compare_with_naive() {
        for nums in (0..=10).chain([0]).powerset() {
            let expected = naive_longest_subsequence(&nums);
            let actual = longest_subsequence(nums.clone());
            assert_eq!(expected, actual, "nums: {:?}", nums);
        }
    }
}
