// https://leetcode.com/problems/stone-game-ii

pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
    let n = piles.len();
    let step_span = max_m_for_index(n-1);
    let mut dp = vec![None; step_span * n];
    
    let mut tail_sum = 0;
    // dbg!(n);
    for start in (0..n).rev() {
        let max_m = max_m_for_index(start);
        tail_sum += piles[start];
        for m in 1..=max_m {
            let idx = to_index(start, m, step_span);
            if start + 2 * m >= n {
                dp[idx].replace(tail_sum);
                continue;
            }
            // dbg!(start, m, idx, tail_sum);
            let min_opponent_score = (1..=(n-start).saturating_sub(1).min(m*2)).map(|x| {
                debug_assert!(x <= max_m_for_index(start+x), "x={} start={} m={} max_m={}", x, start, m, max_m_for_index(start+x));
                dp[to_index(start + x, usize::max(m, x), step_span)].unwrap()
            }).min().unwrap_or(0);
            let was = dp[idx].replace(tail_sum - min_opponent_score);
            debug_assert!(was.is_none(), "start={} m={} idx={} was={:?}", start, m, idx, was);
        }
    }
    dp[to_index(0, 1, step_span)].unwrap()
}


// Maximum potential M for a move starting from a given index.
fn max_m_for_index(start: usize) -> usize {
    (start+1).div_ceil(2)
}

fn to_index(start: usize, m: usize, step_span: usize) -> usize {
    debug_assert!(m <= max_m_for_index(start), "m={} start={} max_m={}", m, start, max_m_for_index(start));
    debug_assert!(m <= step_span, "m={} step_span={} start={}", m, step_span, start);
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
    use super::{stone_game_ii};

    #[test]
    fn official1() {
        assert_eq!(10, stone_game_ii(vec![2, 7, 9, 4, 4]));
    }

    #[test]
    fn official2() {
        assert_eq!(104, stone_game_ii(vec![1, 2, 3, 4, 5, 100]));
    }

    #[test]
    fn official17() {
        assert_eq!(273, stone_game_ii(vec![86,11,7,6,46,37,72,67,33,25,54,45]));
    }

}
