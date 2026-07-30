// https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-i

pub fn minimum_pushes(word: String) -> i32 {
    let n = word.len() as i32;
    let eights = n / 8;
    let remainder = n % 8;
    (eights + 1) * (4 * eights + remainder)
}

#[cfg(test)]
mod tests {
    use super::minimum_pushes;

    #[test]
    fn official1() {
        // Input from exampleTestcases; expected output copied from examples given in content.
        assert_eq!(5, minimum_pushes("abcde".to_string()));
    }

    #[test]
    fn official2() {
        // Input from exampleTestcases; expected output copied from examples given in content.
        assert_eq!(12, minimum_pushes("xycdefghij".to_string()));
    }

    #[test]
    fn single_char() {
        assert_eq!(1, minimum_pushes("a".to_string()));
    }

    #[test]
    fn exactly_fills_first_row() {
        // 8 distinct letters: all one push each.
        assert_eq!(8, minimum_pushes("abcdefgh".to_string()));
    }

    #[test]
    fn one_past_first_row() {
        // 9th letter spills into the second row (two pushes).
        assert_eq!(10, minimum_pushes("abcdefghi".to_string()));
    }

    #[test]
    fn exactly_fills_two_rows() {
        // 16 distinct letters: 8 at one push, 8 at two pushes.
        assert_eq!(24, minimum_pushes("abcdefghijklmnop".to_string()));
    }

    #[test]
    fn full_alphabet() {
        // All 26 letters distinct: 8*1 + 8*2 + 8*3 + 2*4.
        assert_eq!(56, minimum_pushes("abcdefghijklmnopqrstuvwxyz".to_string()));
    }
}
