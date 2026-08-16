// https://leetcode.com/problems/stone-game-ix

use std::{cell::RefCell, collections::HashMap};

thread_local! {
    static CACHE: RefCell<HashMap<GameState, bool>> = RefCell::new(HashMap::new());
}

pub fn stone_game_ix(stones: Vec<i32>) -> bool {
    let mut freq = [0u32; 3];
    for &stone in &stones {
        freq[(stone % 3) as usize] += 1;
    }
    CACHE.with_borrow_mut(|cache| {
        can_win(GameState::new(freq, true), cache)
    })
}

fn can_win(state: GameState, cache: &mut HashMap<GameState, bool>) -> bool {
    if state.is_empty() {
        !state.turn
    } else {
        if let Some(&result) = cache.get(&state) {
            result
        } else {
            let result = state.valid_moves()
                .any(|next_state| !can_win(next_state, cache));
            dbg!(&state, result);
            cache.insert(state, result);
            result
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GameState {
    stones: [u32; 3],
    remainder: u32,
    turn: bool,
}

impl GameState {
    fn new(stones: [u32; 3], turn: bool) -> Self {
        GameState { stones, remainder: 0, turn }
    }

    fn is_empty(&self) -> bool {
        self.stones.iter().all(|&s| s == 0)
    }

    fn valid_moves(&self) -> impl Iterator<Item=GameState> {
        (0..=2)
            .filter_map(move |stone_type| {
                if self.stones[stone_type] == 0 {
                    return None;
                }
                let remainder = (self.remainder + stone_type as u32) % 3;
                if remainder == 0 {
                    return None;
                }
                let mut stones = self.stones;
                stones[stone_type] -= 1;
                
                Some(GameState { stones, remainder, turn: !self.turn })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::stone_game_ix;
    use itertools::Itertools;

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
                    // dbg!(turn, current_sum, stones, true);
                    return true;
                }
            }
            // dbg!(turn, current_sum, stones, false);
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

    #[test]
    fn one_to_three() {
        assert!(!naive_stone_game_ix(vec![1, 2, 3]));
        assert!(!stone_game_ix(vec![1, 2, 3]));
    } 

    #[test]
    fn one_three() {
        assert!(!naive_stone_game_ix(vec![1, 3]));
        assert!(!stone_game_ix(vec![1, 3]));
    } 

    #[test]
    fn one_two_four_five() {
        assert!(naive_stone_game_ix(vec![1, 2, 4, 5]));
        assert!(stone_game_ix(vec![1, 2, 4, 5]));
    } 

    #[test]
    fn powerset_1_to_10() {
        for subset in (1..=10).powerset() {
            assert_eq!(
                naive_stone_game_ix(subset.clone()),
                stone_game_ix(subset.clone()),
                "mismatch for {subset:?}"
            );
        }
    }
}
