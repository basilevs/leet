// https://leetcode.com/problems/number-of-unique-xor-triplets-ii

use std::collections::HashSet;

    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        // x^y = !(x&y) & (x|y) = (x|y) & (!x|!y)
        // q^z = (q|z) & !(q&z)
        // x^y^z = ((x|y) & (!x|!y) | z) & !( (x|y) & (!x|!y) & z )
        //       = (x|y|z) & (!x|!y|z) & ( !(x|y) | !(!x|!y) | !z)
        //       = (x|y|z) & (!x|!y|z) &  ( !x&!y | x&y | !z )

        let mut step: HashSet<i32> = HashSet::with_capacity(nums.len() * nums.len());
        step.extend(nums.iter());
        let mut buffer: Vec<i32> = step.iter().copied().collect();
        step.clear();

        for (i, &x) in buffer.iter().enumerate() {
            for &y in buffer.iter().skip(i + 1) {
                step.insert(x ^ y);
            }
        }
        
        buffer.clear();
        buffer.extend(step.iter());
        step.clear();

        for &x in buffer.iter() {
            for &y in nums.iter() {
                step.insert(x ^ y);
            }
        }

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
}
