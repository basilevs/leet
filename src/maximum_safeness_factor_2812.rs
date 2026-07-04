// https://leetcode.com/problems/find-the-safest-path-in-a-grid

use std::collections::{BinaryHeap, VecDeque};

pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
    // BFS to find the distance from each cell to the nearest thief
    let n = grid.len();
    let mut map = vec![vec![i32::MAX; n]; n];
    let mut queue = VecDeque::new();
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 {
                map[i][j] = 0;
                queue.push_back((i, j));
            }
        }
    }
    while let Some((x, y)) = queue.pop_front() {
        for (nx, ny) in neighbors(x, y, n) {
            if map[nx][ny] == i32::MAX {
                map[nx][ny] = map[x][y] + 1;
                queue.push_back((nx, ny));
            }
        }
    }

    drop(queue);

    // Modified Dijkstra's algorithm
    let mut queue = BinaryHeap::new();
    queue.push((map[0][0], 0, 0));

    while let Some((safety, x, y)) = queue.pop() {
        if x == n - 1 && y == n - 1 {
            return map[x][y].min(safety);
        }
        for (nx, ny) in neighbors(x, y, n) {
            if map[nx][ny] < 0 {
                continue;
            }
            if nx == n - 1 && ny == n - 1 {
                return map[n - 1][n - 1].min(safety);
            }
            queue.push((map[nx][ny].min(safety), nx, ny));
            map[nx][ny] = -1; // mark as visited
        }
    }

    unreachable!()
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
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
    use super::maximum_safeness_factor;

    fn to_vector<const N: usize, const M: usize>(input: [[i32; M]; N]) -> Vec<Vec<i32>> {
        input.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    fn official1() {
        let grid = [[1, 0, 0], [0, 0, 0], [0, 0, 1]];
        assert_eq!(0, maximum_safeness_factor(to_vector(grid)));
    }

    #[test]
    fn official2() {
        let grid = [[0, 0, 1], [0, 0, 0], [0, 0, 0]];
        assert_eq!(2, maximum_safeness_factor(to_vector(grid)));
    }

    #[test]
    fn official3() {
        let grid = [[0, 0, 0, 1], [0, 0, 0, 0], [0, 0, 0, 0], [1, 0, 0, 0]];
        assert_eq!(2, maximum_safeness_factor(to_vector(grid)));
    }

    #[test]
    fn official4() {
        let grid = [[1]];
        assert_eq!(0, maximum_safeness_factor(to_vector(grid)));
    }

    #[test]
    fn official885() {
        let grid = [[0, 0, 1], [0, 1, 1], [0, 0, 0]];
        assert_eq!(1, maximum_safeness_factor(to_vector(grid)));
    }
}
