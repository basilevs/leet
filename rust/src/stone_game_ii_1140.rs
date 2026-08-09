// https://leetcode.com/problems/stone-game-ii

pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
    recursion(&piles, 1)
}

// Maximum score achievable by first player, with the given M
fn recursion(piles: &[i32], m: usize) -> i32 {
    let total_sum = piles.iter().sum::<i32>();
    if piles.len() <= 2*m {
        return total_sum;
    }
    let result = total_sum - (1..=piles.len().saturating_sub(1).min(m*2)).map(|x| {
        recursion(&piles[x..], usize::max(m, x))
    }).min().unwrap_or(0);
    dbg!(piles, m, result);
    result

}

#[cfg(test)]
mod tests {
    use super::stone_game_ii;

    #[test]
    fn official1() {
        assert_eq!(10, stone_game_ii(vec![2, 7, 9, 4, 4]));
    }

    #[test]
    fn official2() {
        assert_eq!(104, stone_game_ii(vec![1, 2, 3, 4, 5, 100]));
    }
}
