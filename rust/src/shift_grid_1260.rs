// https://leetcode.com/problems/shift-2d-grid

pub fn shift_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let k: usize = usize::try_from(k).expect("k should not be negative");
    let k: usize = (m * n - k % (m * n)) % (m * n);
    if k == 0 {
        return grid;
    }
    // https://cplusplus.com/reference/algorithm/rotate/
    let mut first = 0;
    let mut middle = k;
    let mut next = middle;
    while first != next {
        let cell_i = to_coordinates(first, n, m);
        let cell_j = to_coordinates(next, n, m);
        let temp = grid[cell_j.0][cell_j.1];
        grid[cell_j.0][cell_j.1] = grid[cell_i.0][cell_i.1];
        grid[cell_i.0][cell_i.1] = temp;
        //dbg!(&layer, first, &cell_i, next, &cell_j);
        first += 1;
        next += 1;
        if next == m * n {
            next = middle;
        } else if first == middle {
            middle = next;
        }
    }

    grid
}

const fn to_coordinates(index: usize, n: usize, m: usize) -> (usize, usize) {
    let index = index % (n * m);
    let row = index / n;
    let col = index % n;
    (row, col)
}

#[cfg(test)]
mod tests {
    use super::shift_grid;

    fn to_vector<const N: usize, const M: usize>(input: [[i32; M]; N]) -> Vec<Vec<i32>> {
        input.iter().map(|row| row.to_vec()).collect()
    }

    #[rustfmt::skip]
    #[test]
    fn official1() {
        let expected = [
            [9, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
        ];
        let grid = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ];
        assert_eq!(to_vector(expected), shift_grid(to_vector(grid), 1));
    }

    #[rustfmt::skip]
    #[test]
    fn official2() {
        let expected = [
            [12, 0, 21, 13],
            [3, 8, 1, 9],
            [19, 7, 2, 5],
            [4, 6, 11, 10],
        ];
        let grid = [
            [3, 8, 1, 9],
            [19, 7, 2, 5],
            [4, 6, 11, 10],
            [12, 0, 21, 13],
        ];
        assert_eq!(to_vector(expected), shift_grid(to_vector(grid), 4));
    }

    #[rustfmt::skip]
    #[test]
    fn official3() {
        let expected = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ];
        let grid = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ];
        assert_eq!(to_vector(expected), shift_grid(to_vector(grid), 9));
    }
    
    #[rustfmt::skip]
    #[test]
    fn official20() {
        let expected = [
            [6],
            [5],
            [1],
            [2],
            [3],
            [4],
            [7],
        ];
        let grid = [
            [1],
            [2],
            [3],
            [4],
            [7],
            [6],
            [5],
        ];
        assert_eq!(to_vector(expected), shift_grid(to_vector(grid), 23));
    }
}
