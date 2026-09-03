// https://leetcode.com/problems/construct-uniform-parity-array-ii

    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        dbg!(&nums1);
        todo!("training scaffold: implement solution");
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
