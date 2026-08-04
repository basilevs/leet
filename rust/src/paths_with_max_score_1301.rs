// https://leetcode.com/problems/number-of-paths-with-max-score

use crate::modint::ModInt;

pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
    let n = board.len();
    let mut dp = vec![(0,ModInt::ZERO); n.pow(2)];

    // (score, count)
    dp[n.pow(2) - 1] = (0, ModInt::ONE); // start from the end
    for i in (0..(2*n-2)).rev() {
        let y1 = i.min(n - 1);
        let y2 = (i+1).saturating_sub(n); 
        for y in y2..=y1 {
            let x = i - y;
            let c = board[x].as_bytes()[y];
            if c == b'X' {
                continue;
            }
            let digit = if c == b'E'{
                0
            } else {
                (c - b'0') as i32
            };
            debug_assert!(digit <= 9);
            debug_assert!(digit >=0);
            let mut score = 0;
            let mut count = ModInt::ZERO;
            for (nx, ny) in neighbors(x, y, n) {
                let (nscore, ncount) = dp[nx * n + ny];
                if score < nscore {
                    score = nscore;
                    count = ncount;
                } else if score == nscore {
                    count += ncount;
                }
            }
            if score == 0 && count == ModInt::ZERO {
                continue;
            }
            dp[x * n + y] = (score + digit, count);
        }
    }
    let (score, count) = dp[0];
    vec![score, count.into()]
}

const DIRECTIONS: [(i32, i32); 3] = [(0, 1), (1, 0), (1, 1)];
fn neighbors(x: usize, y: usize, n: usize) -> impl Iterator<Item = (usize, usize)> {
    DIRECTIONS.into_iter().filter_map(move |(dx, dy)| {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
            Some((nx as usize, ny as usize))
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::paths_with_max_score;

    fn to_vector<const N: usize>(input: [&str; N]) -> Vec<String> {
        input.into_iter().map(String::from).collect()
    }

    #[test]
    fn official1() {
        #[rustfmt::skip]
        let board = [
            "E23",
            "2X2",
            "12S"
        ];
        assert_eq!(vec![7, 1], paths_with_max_score(to_vector(board)));
    }

    #[test]
    fn official2() {
        #[rustfmt::skip]
        let board = [
            "E12",
            "1X1",
            "21S"
        ];
        assert_eq!(vec![4, 2], paths_with_max_score(to_vector(board)));
    }

    #[test]
    fn official3() {
        #[rustfmt::skip]
        let board = [
            "E11",
            "XXX",
            "11S"
        ];
        assert_eq!(vec![0, 0], paths_with_max_score(to_vector(board)));
    }
}
