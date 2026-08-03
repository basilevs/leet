// https://leetcode.com/problems/stone-game-iii

pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
    // dp[i] = max relative score achievable by the current player starting from index i.
    // Walking i from the back, the candidates at index i are
    //     sum(stone_value[i..=k]) - dp[k + 1]   for k in i..min(i + 3)
    //     stone_value[i] - dp[i + 1] // k == i
    //     stone_value[i] + stone_value[i + 1] - dp[i + 2] // k == i + 1
    //     stone_value[i] + stone_value[i + 1] + stone_value[i + 2] - dp[i + 3] // k == i + 2
    // which all share the factor stone_value[i], leaving three carried terms:
    //     a[i] = -dp[i + 1]                                                                           (k == i,     always valid)
    //     b[i] = stone_value[i + 1] - dp[i + 2] = stone_value[i + 1] + a[i + 1]                       (k == i + 1, valid while i + 1 < n)
    //     c[i] = stone_value[i + 1] + stone_value[i + 2] - dp[i + 3] = stone_value[i + 1] + b[i + 1]  (valid while i + 2 < n)
    // The maximum of these three candidates is the new dp[i], and the new a[i] is -dp[i].
    // All can be expressed in terms of the previous iteration, so we can just carry three values down the array.
    let (mut a, mut b, mut c) = (0i32, None, None);
    for &v in stone_value.iter().rev() {
        // `a` is unconditional, so it seeds the fold and there is no empty-max case.
        let dp = v + b.into_iter().chain(c).fold(a, i32::max);
        // shift the window down by one stone: the old k == i and k == i + 1
        // candidates become the new k == i + 1 and k == i + 2 ones
        (a, b, c) = (-dp, Some(v + a), b.map(|b| v + b));
    }
    let dp0 = -a;
    if dp0 > 0 {
        "Alice".to_string()
    } else if dp0 < 0 {
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
