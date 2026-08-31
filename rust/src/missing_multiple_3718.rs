// https://leetcode.com/problems/smallest-missing-multiple-of-k

pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
    let mut multiples_present = vec![false; 100 / k as usize];
    for &i in &nums {
        if i >= k && i % k == 0 {
            multiples_present[(i / k) as usize - 1] = true;
        }
    }
    (multiples_present.iter().position(|&x| !x).unwrap_or(multiples_present.len()) + 1) as i32 * k
}

#[cfg(test)]    
mod tests {
    use super::missing_multiple;

    #[test]
    fn official1() {
        assert_eq!(10, missing_multiple(vec![8, 2, 3, 4, 6], 2));
    }

    #[test]
    fn official2() {
        assert_eq!(5, missing_multiple(vec![1, 4, 7, 10, 15], 5));
    }

    #[test]
    fn official358() {
        assert_eq!(132, missing_multiple(vec![42,13,99,13,71,32,64,32,63,44,6,22,8,2,55,88,43,40,71,80,95,32,46,19], 44));
    }
}
