// https://leetcode.com/problems/remove-covered-intervals

pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    let n: i32 = intervals.len().try_into().expect("intervals.length <= 1000");
    intervals.sort_unstable_by_key(|i| (i[0], -i[1]));
    let mut covered = 0;
    let mut removed = 0;
    for i in intervals {
        if i[1] <= covered {
            removed += 1;
        } else {
            covered = i[1];
        }
    }
    n - removed
}

#[cfg(test)]
mod tests {
    use super::remove_covered_intervals;

    fn to_vector<const N: usize, const M: usize>(input: [[i32; M]; N]) -> Vec<Vec<i32>> {
        input.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    fn official1() {
        #[rustfmt::skip]
        let intervals = [
            [1, 4],
            [3, 6],
            [2, 8],
        ];
        assert_eq!(2, remove_covered_intervals(to_vector(intervals)));
    }

    #[test]
    fn official2() {
        #[rustfmt::skip]
        let intervals = [
            [1, 4],
            [2, 3],
        ];
        assert_eq!(1, remove_covered_intervals(to_vector(intervals)));
    }
}
