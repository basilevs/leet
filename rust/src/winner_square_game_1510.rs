// https://leetcode.com/problems/stone-game-iv

use std::{collections::HashMap, cell::RefCell};

thread_local! {
    static CACHE: RefCell<HashMap<i32, bool>> = RefCell::new(HashMap::new());
}

pub fn winner_square_game(n: i32) -> bool {
    {
        let result_option = CACHE.with(|cache| cache.borrow().get(&n).copied());
        if let Some(r) = result_option {
            return r;
        }
    }

    let sqrt = (n as f64).sqrt() as i32;
    let mut result = false;
    if is_square(n) {
        result = true;
    } else {
        for i in 1..=sqrt {
            let square = i * i;
            if !winner_square_game(n - square) {
                result = true;
                break;
            }
        }
    }

    CACHE.with(|c| c.borrow_mut().insert(n, result));
    result
}

fn is_square(n: i32) -> bool {
    n >= 0 && {
        let r = n.isqrt();
        r * r == n
    }
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
}
