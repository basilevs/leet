// https://leetcode.com/problems/weighted-word-mapping

pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
    let mut result = String::with_capacity(words.len());
    for word in words {
        let mut sum = 0_u8;
        for &b in word.as_bytes() {
            sum += u8::try_from(weights[usize::from(b - b'a')] % 26).unwrap();
            sum %= 26;
        }
        result.push(char::from(b'z' - sum));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::map_word_weights;

    fn to_vector<const N: usize>(input: [&str; N]) -> Vec<String> {
        input.into_iter().map(String::from).collect()
    }

    #[test]
    fn official1() {
        assert_eq!(
            "rij",
            map_word_weights(
                to_vector(["abcd", "def", "xyz"]),
                vec![
                    5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7,
                    2,
                ],
            )
        );
    }

    #[test]
    fn official2() {
        assert_eq!(
            "yyy",
            map_word_weights(
                to_vector(["a", "b", "c"]),
                vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
            )
        );
    }

    #[test]
    fn official3() {
        assert_eq!(
            "g",
            map_word_weights(
                to_vector(["abcd"]),
                vec![
                    7, 5, 3, 4, 3, 5, 4, 9, 4, 2, 2, 7, 10, 2, 5, 10, 6, 1, 2, 2, 4, 1, 3, 4, 4, 5,
                ],
            )
        );
    }
}

