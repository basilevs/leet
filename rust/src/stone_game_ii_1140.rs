// https://leetcode.com/problems/stone-game-ii

pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
    let n = piles.len();
    let step_span = max_m_for_length(n-1);
    dbg!(&n, &step_span, &step_span);
    let mut dp = vec![0; step_span * n];
    
    let mut tail_sum = 0;
    for start in (0..n).rev() {
        let max_m = max_m_for_length(start);
        tail_sum += piles[start];
        dbg!(&start, &max_m);
        for m in 1..=max_m {
            let min_opponent_score = (1..=(n-start).saturating_sub(1).min(m*2)).map(|x| {
                dp[to_index(start + x, usize::max(m, x), step_span)]
            }).min().unwrap_or(0);
            dp[to_index(start, m, step_span)] = tail_sum - min_opponent_score;
        }
    }
    dbg!(&dp);
    dp[to_index(0, 1, step_span)]
}

fn max_m_for_length(start: usize) -> usize {
    (start+1).ilog2() as usize + 1
}

fn to_index(start: usize, m: usize, step_span: usize) -> usize {
    debug_assert!(m <= step_span);
    debug_assert!(m > 0);
    start * step_span + m - 1
}

// Maximum score achievable by first player, with the given M
// fn recursion(piles: &[i32], m: usize) -> i32 {
//     let total_sum = piles.iter().sum::<i32>();
//     if piles.len() <= 2*m {
//         return total_sum;
//     }
//     let result = total_sum - (1..=piles.len().saturating_sub(1).min(m*2)).map(|x| {
//         recursion(&piles[x..], usize::max(m, x))
//     }).min().unwrap_or(0);
//     dbg!(piles, m, result);
//     result

// }

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
