// https://leetcode.com/problems/smallest-divisible-digit-product-ii

    pub fn smallest_number(num: String, mut t: i64) -> String {
        // Primes below 10 are 2, 3, 5, 7. We can factor t into these primes and count their frequencies.
        let mut prime_freq = [0i32; 4]; // 2, 3, 5, 7
        for i in 0..4 {
            let prime = [2, 3, 5, 7][i];
            while t % prime == 0 {
                prime_freq[i] += 1;
                t /= prime;
            }
        }
        if t > 1 {
            return "-1".to_string(); // t has prime factors greater than 7
        }

        let input_bytes = num.into_bytes();
        
        for c in &input_bytes {
            let digit =  c.checked_sub(b'0').expect("Not a digit");
            debug_assert!(digit <= 9, "Not a digit");
            sub_digit_from_primes(&mut prime_freq, digit);
        }

        let mut result = Vec::with_capacity(input_bytes.len());

        for i in input_bytes.iter().rev() {
            let digit =  i.checked_sub(b'0').expect("Not a digit");
            add_digit_to_primes(&mut prime_freq, digit);
            let Some(largest_digit) =sub_largest_digit_from_primes(&mut prime_freq) else {
                result.extend(input_bytes[0..input_bytes.len() - result.len()].iter().rev().map(|&b| if b == b'0' {b'1'} else {b}));
                break;
            };
            debug_assert!(largest_digit >= digit, "Largest digit should be >= current digit");
            result.push(largest_digit + b'0');
        }
        while let Some(largest_digit) = sub_largest_digit_from_primes(&mut prime_freq) {
            result.push(largest_digit + b'0');
        }
        result.reverse();

        String::from_utf8(result).expect("Invalid UTF-8")
    }

fn sub_largest_digit_from_primes(prime_freq: &mut [i32; 4]) -> Option<u8> {
    // Handle 9
    if prime_freq[1] >= 2 {
        prime_freq[1] -= 2;
        return Some(9);
    }
    // Handle 8
    if prime_freq[0] >= 3 {
        prime_freq[0] -= 3;
        return Some(8);
    }
    // Handle 7
    if prime_freq[3] >= 1 {
        prime_freq[3] -= 1;
        return Some(7);
    }
    // Handle 6
    if prime_freq[1] >= 1 && prime_freq[0] >= 1 {
        prime_freq[1] -= 1;
        prime_freq[0] -= 1;
        return Some(6);
    }
    // Handle 5
    if prime_freq[2] >= 1 {
        prime_freq[2] -= 1;
        return Some(5);
    }
    // Handle 4
    if prime_freq[0] >= 2 {
        prime_freq[0] -= 2;
        return Some(4);
    }
    // Handle 3
    if prime_freq[1] >= 1 {
        prime_freq[1] -= 1;
        return Some(3);
    }
    // Handle 2
    if prime_freq[0] >= 1 {
        prime_freq[0] -= 1;
        return Some(2);
    }
    None
} 

fn add_digit_to_primes(prime_freq: &mut [i32; 4], digit: u8) {
    match digit {
        2 => prime_freq[0] += 1,
        3 => prime_freq[1] += 1,
        4 => prime_freq[0] += 2, // 4 is 2^2
        5 => prime_freq[2] += 1,
        6 => { prime_freq[1] += 1; prime_freq[0] += 1 },
        7 => prime_freq[3] += 1,
        8 => prime_freq[0] += 3, // 8 is 2^3
        9 => prime_freq[1] += 2, // 9 is 3^2
        _ => {},
    }
}

fn sub_digit_from_primes(prime_freq: &mut [i32; 4], digit: u8) {
    match digit {
        2 => prime_freq[0] -= 1,
        3 => prime_freq[1] -= 1,
        4 => prime_freq[0] -= 2, // 4 is 2^2
        5 => prime_freq[2] -= 1,
        6 => { prime_freq[1] -= 1; prime_freq[0] -= 1 },
        7 => prime_freq[3] -= 1,
        8 => prime_freq[0] -= 3, // 8 is 2^3
        9 => prime_freq[1] -= 2, // 9 is 3^2
        _ => {},
    }
}

#[cfg(test)]
mod tests {
    use super::smallest_number;

    #[test]
    fn official1() {
        assert_eq!("1488", smallest_number("1234".to_string(), 256));
    }

    #[test]
    fn official2() {
        assert_eq!("12355", smallest_number("12355".to_string(), 50));
    }

    #[test]
    fn official3() {
        assert_eq!("-1", smallest_number("11111".to_string(), 26));
    }

    #[test]
    fn input_zeroes() {
        assert_eq!("17", smallest_number("10".to_string(), 7));
        assert_eq!("117", smallest_number("101".to_string(), 7));
    }

    #[test]
    fn test6() {
        assert_eq!("23", smallest_number("22".to_string(), 6));
    }


    #[test]
    fn prefix_direct_copy() {
        assert_eq!("77774", smallest_number("77773".to_string(), 4));
    }
}
