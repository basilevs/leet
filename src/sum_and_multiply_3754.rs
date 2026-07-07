// https://leetcode.com/problems/concatenate-non-zero-digits-and-multiply-by-sum-i

pub fn sum_and_multiply(mut n: i32) -> i64 {
    let mut sum = 0;
    let mut filtered = 0;
    let mut order = 1;
    while n > 0 {
        let digit = n % 10;
        if digit != 0 {
            filtered += order * digit;
            order *= 10;
            sum += digit;
        }
        n /= 10;
    }
    i64::from(filtered) * i64::from(sum)
}

#[cfg(test)]
mod tests {
    use super::sum_and_multiply;

    #[test]
    fn official1() {
        assert_eq!(12340, sum_and_multiply(10203004));
    }

    #[test]
    fn official2() {
        assert_eq!(1, sum_and_multiply(1000));
    }

    #[test]
    fn boundary_zero() {
        assert_eq!(0, sum_and_multiply(0));
    }

    #[test]
    fn boundary_max_n() {
        assert_eq!(1, sum_and_multiply(1_000_000_000));
    }
}
