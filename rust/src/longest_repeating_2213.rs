// https://leetcode.com/problems/longest-substring-of-one-repeating-character

    pub fn longest_repeating(s: String, query_characters: String, query_indices: Vec<i32>) -> Vec<i32> {
        dbg!(&s, &query_characters, &query_indices);
        todo!("training scaffold: implement solution")
    }

#[cfg(test)]
mod tests {
    use super::longest_repeating;

    #[test]
    fn official1() {
        assert_eq!(
            vec![3, 3, 4],
            longest_repeating(
                "babacc".to_string(),
                "bcb".to_string(),
                vec![1, 3, 3]
            )
        );
    }

    #[test]
    fn official2() {
        assert_eq!(
            vec![2, 3],
            longest_repeating(
                "abyzz".to_string(),
                "aa".to_string(),
                vec![2, 1]
            )
        );
    }
}
