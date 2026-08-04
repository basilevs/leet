// https://leetcode.com/problems/find-greatest-common-divisor-of-array


pub fn find_gcd(nums: Vec<i32>) -> i32 {
    use itertools::MinMaxResult::MinMax;
    use itertools::Itertools;
    let MinMax(&min, &max) = nums.iter().minmax() else {
        return 0;
    };
    gcd(min, max)
}


fn gcd(mut a: i32, mut b: i32) -> i32 {
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::find_gcd;

    #[test]
    fn official1() {
        let nums = vec![2, 5, 6, 9, 10];
        assert_eq!(2, find_gcd(nums));
    }

    #[test]
    fn official2() {
        let nums = vec![7, 5, 6, 8, 3];
        assert_eq!(1, find_gcd(nums));
    }

    #[test]
    fn official3() {
        let nums = vec![3, 3];
        assert_eq!(3, find_gcd(nums));
    }
}
