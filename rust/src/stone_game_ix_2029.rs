// https://leetcode.com/problems/stone-game-ix

use std::{cell::RefCell, collections::HashMap};

thread_local! {
    static CACHE: RefCell<HashMap<GameState, GameResult>> = RefCell::new(HashMap::new());
}

pub fn stone_game_ix(stones: Vec<i32>) -> bool {
    let mut freq = [0u32; 3];
    for &stone in &stones {
        freq[(stone % 3) as usize] += 1;
    }
    CACHE.with_borrow_mut(|cache| {
        match can_win(GameState::new(freq), cache) {
            GameResult::CurrentPlayerBadSum | GameResult::CurrentPlayerNoStones | GameResult::NextPlayerNoStones => false,
            GameResult::NextPlayerBadSum  => true,
        }
    })
}

fn can_win(state: GameState, cache: &mut HashMap<GameState, GameResult>) -> GameResult {
    if state.is_empty() {
        GameResult::CurrentPlayerNoStones
    } else if state.stones[1] == 0 && state.stones[2] == 0 {
        GameResult::CurrentPlayerBadSum
    } else {
        if let Some(result) =  cache.get(&state){
            result.clone()
        } else {
            let result = (1..=2).filter_map(|stone_type| state.take(stone_type))
                .map(|next_state| can_win(next_state, cache))
                .find(|result| matches!(result, GameResult::CurrentPlayerBadSum | GameResult::CurrentPlayerNoStones))
                .map(|result| result.flip())
                .unwrap_or(GameResult::CurrentPlayerBadSum);
            dbg!(&state, &result);
            cache.insert(state, result.clone());
            result
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GameState {
    stones: [u32; 3],
}

impl GameState {
    fn new(stones: [u32; 3]) -> Self {
        GameState { stones }
    }

    fn is_empty(&self) -> bool {
        self.stones.iter().all(|&s| s == 0)
    }

    fn take(&self, stone_type: usize) -> Option<GameState> {
        debug_assert!(stone_type < 3, "Invalid stone type");
        if self.stones[stone_type] == 0 {
            return None;
        }
        let mut new_stones = self.stones;
        new_stones[stone_type] -= 1;
        new_stones.rotate_right(stone_type);
        Some(GameState::new(new_stones))
    }
}

#[derive(Clone, Debug)]
enum GameResult {
    CurrentPlayerBadSum,
    NextPlayerBadSum,
    CurrentPlayerNoStones,
    NextPlayerNoStones,
}

impl GameResult {
    fn flip(&self) -> GameResult {
        match self {
            GameResult::CurrentPlayerBadSum => GameResult::NextPlayerBadSum,
            GameResult::NextPlayerBadSum => GameResult::CurrentPlayerBadSum,
            GameResult::CurrentPlayerNoStones => GameResult::NextPlayerNoStones,
            GameResult::NextPlayerNoStones => GameResult::CurrentPlayerNoStones,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::stone_game_ix;

    fn naive_stone_game_ix(stones: Vec<i32>) -> bool {
        fn can_win(stones: &[i32], current_sum: i32, turn: bool) -> bool {
            if stones.is_empty() {
                return !turn;
            }
            for i in 0..stones.len() {
                let mut next_stones = stones.to_vec();
                let stone = next_stones.remove(i);
                if (stone + current_sum) % 3 == 0 {
                    continue;
                }
                if !can_win(&next_stones, current_sum + stone, !turn) {
                    return true;
                }
            }
            false
        }
        can_win(&stones, 0, true)
    }

    #[test]
    fn official1() {
        assert!(naive_stone_game_ix(vec![2, 1]));
        assert!(stone_game_ix(vec![2, 1]));
    }

    #[test]
    fn official2() {
        assert!(!naive_stone_game_ix(vec![2]));
        assert!(!stone_game_ix(vec![2]));
    }

    #[test]
    fn official3() {
        assert!(!naive_stone_game_ix(vec![5, 1, 2, 4, 3]));
        assert!(!stone_game_ix(vec![5, 1, 2, 4, 3]));
    }


}
