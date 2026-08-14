// https://leetcode.com/problems/maximum-length-substring-with-two-occurrences

pub fn maximum_length_substring(s: String) -> i32 {
    // this is a copy of solution for https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency
    let mut freq = [0; 26];
    let bytes = s.into_bytes();
    // Elements leaving the window.
    let mut back = bytes.iter();

    let mut length = 0usize;
    // Each element maps to the longest good window ending on it.
    let longest_length = bytes
        .iter()
        .map(|&num| {
            let bucket = &mut freq[usize::from(num - b'a')];
            *bucket += 1;
            length += 1;
            if *bucket > 2 {
                // Only `num` can exceed `k`, and by exactly one, so shrinking
                // until its earliest occurrence leaves the window is enough.
                loop {
                    let &evicted = back.next().expect("window is not empty");
                    freq[usize::from(evicted - b'a')] -= 1;
                    length -= 1;
                    if evicted == num {
                        break;
                    }
                }
            }
            debug_assert!(freq.iter().all(|&v| v <= 2));
            length
        })
        .max()
        .unwrap_or(0);
    i32::try_from(longest_length).unwrap_or_else(|_| {
        panic!("longest_length should be less than 10^2 but was: {longest_length}")
    })
}

#[cfg(test)]
mod tests {
    use super::maximum_length_substring;

    #[test]
    fn official1() {
        assert_eq!(4, maximum_length_substring("bcbbbcba".to_string()));
    }

    #[test]
    fn official2() {
        assert_eq!(2, maximum_length_substring("aaaa".to_string()));
    }
}
