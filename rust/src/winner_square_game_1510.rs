// https://leetcode.com/problems/stone-game-iv

use std::cell::RefCell;

// `CACHE[k]` — does the player to move win with `k` stones left?
// Filled bottom-up and kept between calls, so a later testcase only extends it.
thread_local! {
    static CACHE: RefCell<Vec<bool>> = const { RefCell::new(Vec::new()) };
}

pub fn winner_square_game(n: i32) -> bool {
    let n = n as usize;
    CACHE.with(|cache| {
        let mut wins = cache.borrow_mut();
        if wins.is_empty() {
            wins.push(false); // No stones left: the player to move cannot move.
        }

        for k in wins.len()..=n {
            let win = (1usize..)
                .map(|i| i * i)
                .take_while(|&square| square <= k)
                .any(|square| !wins[k - square]);
            wins.push(win);
        }

        wins[n]
    })
}

#[cfg(test)]
mod tests {
    use super::winner_square_game;

    #[test]
    fn official1() {
        assert!(winner_square_game(1));
    }

    #[test]
    fn official2() {
        assert!(!winner_square_game(2));
    }

    #[test]
    fn official3() {
        assert!(winner_square_game(4));
    }

    /// Alice wins only through a longer line: 13 -> 12, leaving Bob a losing
    /// pile. Every Bob reply (12 -> 11, 8 or 3) hands Alice a winning pile
    /// again, e.g. 13 -> 12 -> 11 -> 10 -> 9 -> 0.
    #[test]
    fn several_moves() {
        assert!(winner_square_game(13));
    }
}
