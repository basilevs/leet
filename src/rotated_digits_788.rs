// Digit DP: decomposes n by leading digit, counts good numbers
// via overcounting/undercounting across digit-count boundaries.
//
// Valid digits under 180° rotation: 0, 1, 2, 5, 6, 8, 9 (7 total)
//   Same    (self-rotation):    0, 1, 8           (3 total)
//   Changed (rotates to other): 2↔5, 6↔9         (4 total)
// Invalid: 3, 4, 7
//
// "Good" = all digits valid AND at least one changed.

    pub fn rotated_digits(n: i32) -> i32 {
        if n <= 10 {
            return match n {
                0 | 1 => 0,
                2 | 3 | 4 => 1,
                5 => 2,
                6 | 7 | 8 => 3,
                9 | 10 => 4,
                _ => unreachable!(),
            };
        }

        let dc = n.ilog10();
        let top = n / 10_i32.pow(dc);
        let rest = n - top * 10_i32.pow(dc);
        // Suffixes (dc digits) containing at least one changed digit:
        let good_suffix = 7_i32.pow(dc) - 3_i32.pow(dc);
        // All valid-rotation numbers below 10^dc:
        let valid_below = count_valid(10_i32.pow(dc) - 1);

        let by_top: i32 = (1..=top)
            .map(|d| match d {
                1       => good_suffix,
                2 | 9   => good_suffix + 1,
                3 | 7   => valid_below,
                4 | 8   => 0,
                5       => 1,
                6       => valid_below + 1,
                _ => unreachable!(),
            })
            .sum();

        let by_rest = match top {
            1 | 8         => rotated_digits(rest),
            2 | 5 | 6 | 9 => count_valid(rest),
            3 | 4 | 7     => 0,
            _ => unreachable!(),
        };

        by_top + by_rest
    }

/// Counts numbers in [1, n] where every digit is valid under rotation
/// (includes numbers that rotate to themselves).
fn count_valid(n: i32) -> i32 {
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
            _ => unreachable!(),
        };
    }

    let dc = n.ilog10();
    let top = n / 10_i32.pow(dc);
    let rest = n - top * 10_i32.pow(dc);
    let full = 7_i32.pow(dc); // valid completions per valid leading digit

    let by_top: i32 = (1..=top)
        .map(|d| match d {
            1 | 2 | 6 | 9 => full,
            3 | 7          => full - 1,
            5 | 8          => 1,
            4              => 0,
            _ => unreachable!(),
        })
        .sum();

    let by_rest = match top {
        1 | 2 | 5 | 6 | 8 | 9 => count_valid(rest),
        3 | 4 | 7               => 0,
        _ => unreachable!(),
    };

    by_top + by_rest
}

#[cfg(test)]
fn naive(n: i32) -> i32 {
    (0..=n).filter(is_good).count() as i32
}

#[cfg(test)]
fn is_good(n: &i32) -> bool {
    let mut n = *n;
    let mut changed = false;
    while n > 0 {
        match n % 10 {
            0 | 1 | 8 => {}
            2 | 5 | 6 | 9 => changed = true,
            _ => return false,
        }
        n /= 10;
    }
    changed
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
    for n in 1..=10_000 {
        if is_good(&n) {
            count += 1;
        }
        assert_eq!(count, rotated_digits(n), "n={n}");
    }
}

#[test]
fn t11() {
    assert_eq!(4, rotated_digits(11));
}

#[test]
fn a11() {
    assert_eq!(8, count_valid(11));
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
