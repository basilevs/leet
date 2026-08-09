// https://leetcode.com/problems/find-the-lexicographically-smallest-valid-sequence

    pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {

        let mut dp: = vec![]
        dbg!(&word1, &word2);
        todo!("training scaffold: implement solution");
    }

#[cfg(test)]
mod tests {
    use super::valid_sequence;

    #[test]
    fn official1() {
        assert_eq!(vec![0, 1, 2], valid_sequence("vbcca".to_string(), "abc".to_string()));
    }

    #[test]
    fn official2() {
        assert_eq!(vec![1, 2, 4], valid_sequence("bacdc".to_string(), "abc".to_string()));
    }

    #[test]
    fn official3() {
        assert_eq!(Vec::<i32>::new(), valid_sequence("aaaaaa".to_string(), "aaabc".to_string()));
    }

    #[test]
    fn official4() {
        assert_eq!(vec![0, 1], valid_sequence("abc".to_string(), "ab".to_string()));
    }
}
