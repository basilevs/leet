// https://leetcode.com/problems/find-the-lexicographically-smallest-valid-sequence

    pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
        let mut dp = vec![0; word1.len() + 1];
        let word1 = word1.into_bytes();
        let word2 = word2.into_bytes();
        let mut suffix_start = word2.len();
        dp[word1.len()] = suffix_start;
        for (i, &c) in word1.iter().enumerate().rev() {
            let candidate = suffix_start.saturating_sub(1);
            if c == word2[candidate] {
                suffix_start = candidate;
            }
            dp[i] = suffix_start;
        }
        
        let mut result: Vec<i32> = word1.iter().enumerate()
            .zip(dp.into_iter().skip(1))
            .scan((0usize, false),|(cursor1, wildcard_used), ((i, &c), suffix_start)| {
                if *cursor1 >= word2.len() {
                    None
                } else if word2[*cursor1] == c {
                    *cursor1 += 1;
                    Some(i as i32)
                } else if !*wildcard_used && *cursor1 + 1 >= suffix_start {
                    *wildcard_used = true;
                    *cursor1 += 1;
                    Some(i as i32)
                } else {
                    Some(-1)
                }
            })
            .filter(|&x| x != -1)
            .collect();
        if result.len() < word2.len() {
            result.clear();
        }
        result
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
