// https://leetcode.com/problems/network-recovery-pathways

pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
    dbg!(edges, online, k);
    todo!("training scaffold: implement solution");
}

#[cfg(test)]
mod tests {
    use super::find_max_path_score;

    fn to_vector<const N: usize, const M: usize>(input: [[i32; M]; N]) -> Vec<Vec<i32>> {
        input.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    fn official1() {
        #[rustfmt::skip]
        let edges = [
            [0, 1,  5],
            [1, 3, 10],
            [0, 2,  3],
            [2, 3,  4],
        ];
        assert_eq!(3, find_max_path_score(to_vector(edges), vec![true, true, true, true], 10));
    }

    #[test]
    fn official2() {
        #[rustfmt::skip]
        let edges = [
            [0, 1, 7],
            [1, 4, 5],
            [0, 2, 6],
            [2, 3, 6],
            [3, 4, 2],
            [2, 4, 6],
        ];
        assert_eq!(6, find_max_path_score(to_vector(edges), vec![true, true, true, false, true], 12));
    }
}
