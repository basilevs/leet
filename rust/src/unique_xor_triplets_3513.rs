// https://leetcode.com/problems/number-of-unique-xor-triplets-i

pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {

    let n: u64 = nums.len().try_into().unwrap();
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 2;
    }
    (n+1).next_power_of_two().try_into().unwrap()

}

#[cfg(test)]
mod tests {
    use super::unique_xor_triplets;

    #[test]
    fn official1() {
        assert_eq!(2, unique_xor_triplets(vec![1, 2]));
    }

    #[test]
    fn official2() {
        assert_eq!(4, unique_xor_triplets(vec![3, 1, 2]));
    }

    #[test]
    fn official784() {
        assert_eq!(1, unique_xor_triplets(vec![1]));
    }

    #[test]
    fn t1() {
        assert_eq!(8, unique_xor_triplets(vec![3, 1, 2, 4]));
    }
}
