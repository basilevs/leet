use crate::solution::Solution;

impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
       let mut max = i32::MIN;
       let mut max_count = 0;
       let mut seen = vec![false; nums.len() - 1];
       for v in nums {
            let idx: usize = v as usize - 1;
            if idx >= seen.len() {
                return false;
            }
            if v > max {
                max = v;
                max_count = 1;
                seen[idx] = true;
            } else if v == max {
                max_count += 1;
                if max_count > 2 {
                    return false;
                }
            } else {
                if seen[idx] {
                    return false;
                }
                seen[idx] = true;
            }
       }
       max_count == 2 && seen.iter().all(|&x| x)
    }
}

#[test]
fn official1() {
    assert!(Solution::is_good(vec![1, 3, 3, 2]));
}

#[test]
fn official2() {
    assert!(!Solution::is_good(vec![9,9]));
}