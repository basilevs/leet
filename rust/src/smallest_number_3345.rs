// https://leetcode.com/problems/smallest-divisible-digit-product-i

use std::iter::successors;

    pub fn smallest_number(mut n: i32, t: i32) -> i32 {
        loop {
            if (digits(n).map(i32::from).product::<i32>() % t) == 0 {
                break n
            }
            n += 1;
        }
    }

fn digits( n: i32) -> impl Iterator<Item=u8> {
    successors(Some(n), |x: &i32| (*x >= 10).then(|| x / 10)).map(|n| (n % 10) as u8)
}

#[cfg(test)]
mod tests {
    use super::smallest_number;

    #[test]
    fn official1() {
        // Input from exampleTestcases; expected output copied from examples given in content.
        assert_eq!(10, smallest_number(10, 2));
    }

    #[test]
    fn official2() {
        // Input from exampleTestcases; expected output copied from examples given in content.
        assert_eq!(16, smallest_number(15, 3));
    }
}
