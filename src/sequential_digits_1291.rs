// https://leetcode.com/problems/sequential-digits


#[rustfmt::skip]
const SEQUENTIAL_DIGITS: [i32; 36] = [
    12, 23, 34, 45, 56, 67, 78, 89,
    123, 234, 345, 456, 567, 678, 789,
    1234, 2345, 3456, 4567, 5678, 6789,
    12345, 23456, 34567, 45678, 56789,
    123456, 234567, 345678, 456789,
    1234567, 2345678, 3456789,
    12345678, 23456789,
    123456789
];

pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    let start = SEQUENTIAL_DIGITS.binary_search(&low).unwrap_or_else(|x| x);
    let end = SEQUENTIAL_DIGITS.binary_search(&high).map(|x| x + 1).unwrap_or_else(|x| x);
    SEQUENTIAL_DIGITS[start..end].to_vec()
}

#[cfg(test)]
mod tests {
    use super::sequential_digits;

    #[test]
    fn official1() {
        let low = 100;
        let high = 300;
        let expected = vec![123, 234];
        assert_eq!(expected, sequential_digits(low, high));
    }

    #[test]
    fn official2() {
        let low = 1000;
        let high = 13000;
        let expected = vec![1234, 2345, 3456, 4567, 5678, 6789, 12345];
        assert_eq!(expected, sequential_digits(low, high));
    }
}
