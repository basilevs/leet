    pub fn rotated_digits(n: i32) -> i32 {
        if n <= 10 {
            return match n {
                0 | 1 => 0,
                2 | 3 | 4 => 1,
                5 => 2,
                6 | 7 | 8 => 3,
                9 | 10 => 4,
                _ => unreachable!()
            }
        }

        let digit_count = n.ilog10();
        let last_digit = n / 10_i32.pow(digit_count);
        debug_assert!(last_digit < 10);
        debug_assert!(last_digit > 0);
        let remainder = n - last_digit * 10_i32.pow(digit_count);

        last_digit * 7_i32.pow(digit_count) + rotated_digits(remainder) + rotated_digits(last_digit)
    }



// fn rotations_by_digit_count(digits: u32) -> i32 {
//     if digits > 0 {
//         7_i32.pow(digits) - 3_i32.pow(digits)
//     } else {
//         1
//     }
// }

#[cfg(test)]
fn naive(n: i32) -> i32 {
    (0..=n).into_iter().filter(is_good).count() as i32
}

#[cfg(test)]
fn is_good(i: &i32) -> bool {
    let mut i = *i;
    let mut result = false;
    while i > 0 {
        let digit = i % 10;
        result |= match digit {
            0 | 8 | 1 => false,
            2 | 5 | 6 | 9 => true,
            _ => return false
        };
        i /= 10;
    }
    result
}

#[test]
fn official1() {
    assert_eq!(4, rotated_digits(10));
}

#[test]
fn official2() {
    assert_eq!(0, rotated_digits(1));
}

#[test]
fn official3() {
    assert_eq!(1, rotated_digits(2));
}

#[test]
fn t_naive_comparison() {
    for n in 1..1000 {
        assert_eq!(naive(n), rotated_digits(n), "testing {}", n);
    }
}

#[test]
fn t21() {
    assert_eq!(10, rotated_digits(21));
}

