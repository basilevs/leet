// https://leetcode.com/problems/construct-uniform-parity-array-ii

    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        let mut min = i32::MAX;
        let mut has_odd = false;
        for i in nums1 {
            has_odd |= i % 2 != 0;
            min = min.min(i);
        }
        !has_odd || (min % 2 != 0)
    }

#[cfg(test)]
mod tests {
    use super::uniform_array;

    #[test]
    fn official1() {
        assert!(uniform_array(vec![1, 4, 7]));
    }

    #[test]
    fn official2() {
        assert!(!uniform_array(vec![2, 3]));
    }

    #[test]
    fn official3() {
        assert!(uniform_array(vec![4, 6]));
    }
}
