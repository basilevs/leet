// https://leetcode.com/problems/sum-game

pub fn sum_game(num: String) -> bool {
    assert_eq!(0, num.len() % 2);

    let bytes = num.into_bytes();

    let left_statistics = statistics(&bytes[..bytes.len() / 2]);
    let right_statistics = statistics(&bytes[bytes.len() / 2..]);

    let total_question_marks = left_statistics.0 + right_statistics.0;

    if !total_question_marks.is_multiple_of(2) {
        return true;
    }

    let balance = i32::try_from(left_statistics.1).unwrap() - i32::try_from(right_statistics.1).unwrap();
    let move_count = i32::try_from(left_statistics.0).unwrap() - i32::try_from(right_statistics.0).unwrap();
    debug_assert_eq!(0, move_count % 2);

    balance + move_count / 2 * 9 != 0
}

// (question_mark_count, digit_sum)
fn statistics(num: &[u8]) -> (usize, usize) {
    let mut question_mark_count = 0;
    let mut digit_sum = 0;
    for &c in num {
        match c {
            b'?' => question_mark_count += 1,
            b'0'..=b'9' => digit_sum += (c - b'0') as usize,
            _ => panic!("invalid character in input"),
        }
    }
    (question_mark_count, digit_sum)
}

#[cfg(test)]
mod tests {
    use super::sum_game;

    #[test]
    fn official1() {
        assert!(!sum_game("5023".to_string()));
    }

    #[test]
    fn official2() {
        assert!(sum_game("25??".to_string()));
    }

    #[test]
    fn official3() {
        assert!(!sum_game("?3295???".to_string()));
    }

    // Minimax ground truth: Alice (first) wants the halves to differ, Bob wants
    // them equal. A move fills any remaining '?' on either side with a digit,
    // so only per-side sums and '?' counts matter.
    fn brute(l_sum: i32, r_sum: i32, l_q: u32, r_q: u32, alice_turn: bool) -> bool {
        if l_q == 0 && r_q == 0 {
            return l_sum != r_sum;
        }
        let mut outcomes = Vec::new();
        for d in 0..=9 {
            if l_q > 0 {
                outcomes.push(brute(l_sum + d, r_sum, l_q - 1, r_q, !alice_turn));
            }
            if r_q > 0 {
                outcomes.push(brute(l_sum, r_sum + d, l_q, r_q - 1, !alice_turn));
            }
        }
        if alice_turn {
            outcomes.into_iter().any(|alice_wins| alice_wins)
        } else {
            outcomes.into_iter().all(|alice_wins| alice_wins)
        }
    }

    fn expected(num: &str) -> bool {
        let bytes = num.as_bytes();
        let mid = bytes.len() / 2;
        let (mut l_sum, mut r_sum, mut l_q, mut r_q) = (0, 0, 0, 0);
        for (i, &c) in bytes.iter().enumerate() {
            match (c, i < mid) {
                (b'?', true) => l_q += 1,
                (b'?', false) => r_q += 1,
                (_, true) => l_sum += i32::from(c - b'0'),
                (_, false) => r_sum += i32::from(c - b'0'),
            }
        }
        brute(l_sum, r_sum, l_q, r_q, true)
    }

    #[test]
    fn brute_force_comparison() {
        let alphabet = *b"059?";
        for &len in &[2usize, 4] {
            for mut code in 0..alphabet.len().pow(len as u32) {
                let mut s = String::with_capacity(len);
                for _ in 0..len {
                    s.push(alphabet[code % alphabet.len()] as char);
                    code /= alphabet.len();
                }
                assert_eq!(expected(&s), sum_game(s.clone()), "mismatch for {s:?}");
            }
        }
    }
}
