// https://leetcode.com/problems/count-subarrays-with-majority-element-ii

pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i64 {
    let mut freq = vec![0_u32; 2*nums.len() + 1];

    let mut result = 0_i64;
    let mut prefix_sum = 0_i32;
    let mut count_of_lesser_prefixes = 0_u32;
    let n = nums.len();

    freq[nums.len()] += 1; 

    for x in nums {
        if x == target {
            count_of_lesser_prefixes += freq[(prefix_sum + n as i32) as usize];
            prefix_sum += 1;
        } else {
            prefix_sum -= 1;
            count_of_lesser_prefixes -= freq[(prefix_sum + n as i32) as usize];
        };
        result += count_of_lesser_prefixes as i64;
        freq[(prefix_sum + n as i32) as usize] += 1;
        dbg!(prefix_sum, count_of_lesser_prefixes, result, &freq);
    }

    result
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
