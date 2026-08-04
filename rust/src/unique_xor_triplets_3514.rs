// https://leetcode.com/problems/number-of-unique-xor-triplets-ii

use std::collections::HashSet;

    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let mut step: HashSet<i32> = HashSet::with_capacity(nums.len() * nums.len());
        step.extend(nums.iter());
        let mut buffer: Vec<i32> = step.iter().copied().collect();
        let distinct = buffer.clone();
        step.clear();

        for (i, &x) in distinct.iter().enumerate() {
            for &y in distinct.iter().skip(i + 1) {
                step.insert(x ^ y);
            }
        }
        
        buffer.clear();
        buffer.extend(step.iter().copied());
        buffer.push(0);
        step.clear();

        for &x in buffer.iter() {
            for &y in distinct.iter() {
                step.insert(x ^ y);
            }
        }

        step.extend(nums);

        step.len().try_into().unwrap()

    }

#[cfg(test)]
mod tests {
    use super::unique_xor_triplets;

    #[test]
    fn official1() {
        assert_eq!(2, unique_xor_triplets(vec![1, 3]));
    }

    #[test]
    fn official2() {
        assert_eq!(4, unique_xor_triplets(vec![6, 7, 8, 9]));
    }

        #[test]
    fn official254() {
        assert_eq!(15, unique_xor_triplets(vec![503,161,1144,279,513]));
    }
}
