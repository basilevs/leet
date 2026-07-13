use crate::solution::Solution;

struct BitSet {
    words: Vec<u64>,
}

impl BitSet {
    fn new(n: usize) -> Self {
        Self { words: vec![0; n.div_ceil(64)] }
    }

    fn get(&self, i: usize) -> bool {
        self.words[i / 64] & (1 << (i % 64)) != 0
    }

    fn set(&mut self, i: usize) {
        self.words[i / 64] |= 1 << (i % 64);
    }

    fn all_set(&self, n: usize) -> bool {
        let full_words = n / 64;
        let remainder = n % 64;
        self.words[..full_words].iter().all(|&w| w == u64::MAX)
            && (remainder == 0 || self.words[full_words] == (1 << remainder) - 1)
    }
}

impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
       let n = nums.len() - 1;
       let mut max = i32::MIN;
       let mut max_count = 0;
       let mut seen = BitSet::new(n);
       for v in nums {
            let idx: usize = v as usize - 1;
            if idx >= n {
                return false;
            }
            if v > max {
                max = v;
                max_count = 1;
                seen.set(idx);
            } else if v == max {
                max_count += 1;
                if max_count > 2 {
                    return false;
                }
            } else {
                if seen.get(idx) {
                    return false;
                }
                seen.set(idx);
            }
       }
       max_count == 2 && seen.all_set(n)
    }
}

#[cfg(test)]
fn is_good<const N: usize>(nums: [i32; N]) -> bool {
    Solution::is_good(nums.to_vec())
}

#[test]
fn official1() {
    assert!(!is_good([2, 1, 3]));
}


#[test]
fn official2() {
    assert!(is_good([1, 3, 3, 2]));
}

#[test]
fn official3() {
    assert!(is_good([1, 1]));
}

#[test]
fn official4() {
    assert!(!is_good([3, 4, 4, 1, 2, 1]));
}

#[test]
fn official7() {
    assert!(!is_good([9,9]));
}