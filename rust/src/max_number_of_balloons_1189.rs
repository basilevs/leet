// https://leetcode.com/problems/maximum-number-of-balloons

pub fn max_number_of_balloons(text: String) -> i32 {
    // ablon
    let mut freq = [0; 5];
    for c in text.into_bytes() {
        match c {
            b'a' => freq[0] += 2,
            b'b' => freq[1] += 2,
            b'l' => freq[2] += 1,
            b'o' => freq[3] += 1,
            b'n' => freq[4] += 2,
            _ => {}
        }
    }
    freq.into_iter().min().unwrap()/2
}

#[cfg(test)]
mod tests {
    use super::max_number_of_balloons;

    #[test]
    fn official1() {
        assert_eq!(1, max_number_of_balloons("nlaebolko".to_string()));
    }

    #[test]
    fn official2() {
        assert_eq!(2, max_number_of_balloons("loonbalxballpoon".to_string()));
    }

    #[test]
    fn official3() {
        assert_eq!(0, max_number_of_balloons("leetcode".to_string()));
    }
}
