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
        let by_digit_count = 7_i32.pow(digit_count) - 3_i32.pow(digit_count);
        let any_rot_next= any_rot(10_i32.pow(digit_count)-1);

        let mut by_last = 0;
        for i in 1..=last_digit {
            by_last += match i {
                1 => by_digit_count,
                2 => by_digit_count + 1,
                3 => any_rot_next,
                4 => 0,
                5 => 1,
                6 => any_rot_next + 1,
                7 => any_rot_next,
                8 => 0,
                9 => by_digit_count + 1,
                _ => unreachable!()
            };
        }
        
        let by_remainder = match last_digit {
                1 | 8 => rotated_digits(remainder),
                2 | 5 | 6 | 9 => any_rot(remainder),
                3 | 4 | 7 => 0,
                _ => unreachable!()
        };
        // dbg!(n, by_digit_count, any_rot_next, by_last, remainder, by_remainder);
        by_last + by_remainder
    }

fn any_rot(n: i32) -> i32 {
    if n <= 10 {
        return match n {
            0 => 0,
            1 => 1,
            2 | 3 | 4 => 2,
            5 => 3,
            6 | 7 => 4,
            8 => 5,
            9 => 6,
            10 => 7,
            _ => unreachable!()
        }
    }
    let digit_count = n.ilog10();
    let last_digit = n / 10_i32.pow(digit_count);
    let remainder = n - last_digit * 10_i32.pow(digit_count);
    let by_digit_count = 7_i32.pow(digit_count);

    let mut by_last = 0;
    for i in 1..=last_digit {
        by_last += match i {
            1 => by_digit_count,
            2 => by_digit_count,
            3 => by_digit_count - 1,
            4 => 0,
            5 => 1,
            6 => by_digit_count,
            7 => by_digit_count - 1,
            8 => 1,
            9 => by_digit_count,
            _ => unreachable!()
        };
    }

    let by_remainder = match last_digit {
        1 | 8 | 2 | 5 | 6 | 9 => any_rot(remainder),
        3 | 4 | 7 => 0,
        _ => unreachable!()
    };
    // dbg!(n, by_last, by_remainder);
    by_last + by_remainder
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
    let mut count = 0;
    for n in 1..100000 {
        if is_good(&n) {
            count += 1;
        }
        assert_eq!(count, rotated_digits(n), "testing {}", n);
    }
}

#[test]
fn t11() {
    assert_eq!(4, rotated_digits(11));
}

#[test]
fn a11() {
    assert_eq!(8, any_rot(11));
}

#[test]
fn t12() {
    assert_eq!(5, rotated_digits(12));
}

#[test]
fn t20() {
    assert_eq!(9, rotated_digits(20));
}

#[test]
fn t21() {
    assert_eq!(10, rotated_digits(21));
}

#[test]
fn t29() {
    assert_eq!(15, rotated_digits(29));
}

#[test]
fn t30() {
    assert_eq!(15, rotated_digits(30));
}


#[test]
fn t1000() {
    assert_eq!(naive(1000), rotated_digits(1000));
}

#[test]
fn t2000() {
    assert_eq!(naive(2000), rotated_digits(2000));
}



