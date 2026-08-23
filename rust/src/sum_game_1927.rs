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

    let mut balance = i32::try_from(left_statistics.1).unwrap() - i32::try_from(right_statistics.1).unwrap();
    let mut move_count = i32::try_from(left_statistics.0).unwrap() - i32::try_from(right_statistics.0).unwrap();
    assert_eq!(0, move_count % 2);

    if move_count < 0 {
        balance *= -1;
        move_count *= -1;
    }

    if balance + move_count / 2 * 9 > 0 {
        true
    } else {
        move_count / 2 * 9 + balance < 0
    }
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
}
