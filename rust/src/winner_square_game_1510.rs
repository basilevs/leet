// https://leetcode.com/problems/stone-game-iv

pub fn winner_square_game(n: i32) -> bool {
    dbg!(n);
    todo!("training scaffold: implement solution");
}

#[cfg(test)]
mod tests {
    use super::winner_square_game;

    #[test]
    fn official1() {
        assert_eq!(true, winner_square_game(1));
    }

    #[test]
    fn official2() {
        assert_eq!(false, winner_square_game(2));
    }

    #[test]
    fn official3() {
        assert_eq!(true, winner_square_game(4));
    }
}
