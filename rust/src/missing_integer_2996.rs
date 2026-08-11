// https://leetcode.com/problems/smallest-missing-integer-greater-than-sequential-prefix-sum

pub fn missing_integer(nums: Vec<i32>) -> i32 {
    let prefix_length = nums.windows(2).position(|w| {
        let result = w[0] + 1 != w[1];
        // dbg!(w, result);
        result
    }).unwrap_or(nums.len()-1) + 1;
    let sum = (nums[prefix_length - 1] + nums[0]) * prefix_length as i32 / 2;
    // dbg!(&nums,prefix_length,sum);
    if sum > 51_i32 {
        return sum;
    }
    let mut present = [false; 51];
    for &n in nums.iter().skip(prefix_length-1) {
        present[n as usize] = true;
    }
    let search_start = (sum as usize).max(1);
    present.iter().skip(search_start).position(|x| !x).map(|p| p + search_start).unwrap_or(present.len()) as i32
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

    #[test]
    fn official298() {
        assert_eq!(297, missing_integer(vec![29,30,31,32,33,34,35,36,37]));
    }
}
