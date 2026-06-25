// https://leetcode.com/problems/count-subarrays-with-majority-element-i

pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
    let prefix_sums = nums.iter()
        .map(|&x| if x == target { 1 } else { -1 })
        .scan(0_i32, |acc, x| {
            *acc += x;
            Some(*acc)
        }).collect::<Vec<i32>>();

    let mut result = 0_usize;
    // TODO: add prefix sum freq and track active freq on sum change
    for (i, &sum) in prefix_sums.iter().enumerate() {
        let count = prefix_sums[0..i].iter().filter(|&&x| sum > x).count();
        result += count;
        if sum > 0 {
            result += 1;
        }
    }
    i32::try_from(result).unwrap()
}

#[cfg(test)]
mod tests {
    use super::count_majority_subarrays;

    #[test]
    fn official1() {
        // Input from exampleTestcases; expected output copied from examples given in content.
        assert_eq!(5, count_majority_subarrays(vec![1, 2, 2, 3], 2));
    }

    #[test]
    fn official2() {
        // Input from exampleTestcases; expected output copied from examples given in content.
        assert_eq!(10, count_majority_subarrays(vec![1, 1, 1, 1], 1));
    }

    #[test]
    fn official3() {
        // Input from exampleTestcases; expected output copied from examples given in content.
        assert_eq!(0, count_majority_subarrays(vec![1, 2, 3], 4));
    }
}
