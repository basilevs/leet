// https://leetcode.com/problems/maximum-product-of-two-digits

use std::iter::{successors};
use itertools::Itertools;

pub fn max_product(n: i32) -> i32 {
    digits(n).k_largest(2).map(i32::from).product::<i32>()
}
    
fn digits( n: i32) -> impl Iterator<Item=u8> {
    successors(Some(n), |x: &i32| x.is_positive().then(|| x / 10)).map(|n| (n % 10) as u8)
}


#[cfg(test)]
mod tests {
    use super::max_product;

    #[test]
    fn official1() {
        assert_eq!(3, max_product(31));
    }

    #[test]
    fn official2() {
        assert_eq!(4, max_product(22));
    }

    #[test]
    fn official3() {
        assert_eq!(8, max_product(124));
    }
}
