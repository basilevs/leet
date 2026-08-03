// https://leetcode.com/problems/stone-game-iii

pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
    let n = stone_value.len();
    // dp[i] = max relative score achievable by the current player starting from index i
    let mut dp = vec![i32::MIN; n + 1];
    dp[n] = 0;
    for i in (0..n).rev() {
        // take stone i, i+1 or i+2
        dp[i] = (i..n.min(i+3)).map(|k| {
            let take = stone_value[i..=k].iter().sum::<i32>();
            take - dp[k + 1]
        }).max().unwrap();
    }
    if dp[0] > 0 {
        "Alice".to_string()
    } else if dp[0] < 0 {
        "Bob".to_string()
    } else {
        "Tie".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::stone_game_iii;

    #[test]
    fn official1() {
        assert_eq!("Bob", stone_game_iii(vec![1, 2, 3, 7]));
    }

    #[test]
    fn official2() {
        assert_eq!("Alice", stone_game_iii(vec![1, 2, 3, -9]));
    }

    #[test]
    fn official3() {
        assert_eq!("Tie", stone_game_iii(vec![1, 2, 3, 6]));
    }

    #[test]
    fn odd_length() {
        assert_eq!("Alice", stone_game_iii(vec![1, 2, 3]));
        assert_eq!("Bob", stone_game_iii(vec![1, 2, 3, 4, 5]));
    }
}
