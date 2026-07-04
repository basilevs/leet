// https://leetcode.com/problems/minimum-score-of-a-path-between-two-cities

pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    dbg!(n, roads);
    todo!("training scaffold: implement solution");
}

#[cfg(test)]
mod tests {
    use super::min_score;

    fn to_vector<const N: usize, const M: usize>(input: [[i32; M]; N]) -> Vec<Vec<i32>> {
        input.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    fn official1() {
        #[rustfmt::skip]
        let roads = [
            [1, 2, 9],
            [2, 3, 6],
            [2, 4, 5],
            [1, 4, 7],
        ];
        assert_eq!(5, min_score(4, to_vector(roads)));
    }

    #[test]
    fn official2() {
        #[rustfmt::skip]
        let roads = [
            [1, 2, 2],
            [1, 3, 4],
            [3, 4, 7],
        ];
        assert_eq!(2, min_score(4, to_vector(roads)));
    }
}
