// https://leetcode.com/problems/smallest-stable-index-i
// https://leetcode.com/problems/smallest-stable-index-ii (3904)

    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let mut mins = vec![i32::MAX; nums.len()];
        let mut min = i32::MAX;
        for (i, &num) in nums.iter().enumerate().rev() {
            min = min.min(num);
            mins[i] = min;
        }
        let mut max = i32::MIN;
        for (i, &num) in nums.iter().enumerate() {
            max = max.max(num);
            if max - mins[i] <= k {
                return i as i32;
            }
        }
        -1
    }

#[cfg(test)]
mod tests {
    use super::first_stable_index;

    #[test]
    fn official1() {
        assert_eq!(3, first_stable_index(vec![5, 0, 1, 4], 3));
    }

    #[test]
    fn official2() {
        assert_eq!(-1, first_stable_index(vec![3, 2, 1], 1));
    }

    #[test]
    fn official3() {
        assert_eq!(0, first_stable_index(vec![0], 0));
    }
}
