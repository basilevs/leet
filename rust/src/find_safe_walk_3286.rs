// https://leetcode.com/problems/find-a-safe-walk-through-a-grid

use std::collections::VecDeque;

pub fn find_safe_walk(mut grid: Vec<Vec<i32>>, health: i32) -> bool {
    let n = grid.len();
    let m = grid[0].len();

    // Modified Dijkstra's algorithm
    let mut queue = VecDeque::new();
    queue.push_front((health - grid[0][0], 0, 0));
    grid[0][0] = -1;


    while let Some((health, x, y)) = queue.pop_front() {
        if x == n - 1 && y == m - 1 {
            return true;
        }
        for (nx, ny) in neighbors(x, y, n, m) {
            let damage = grid[nx][ny];
            if damage < 0 {
                continue;
            }
            grid[nx][ny] = -1; // mark as visited
            if damage >= health {
                continue;
            }
            let new_health = health - damage;
            // The idea not to sort queue is from
            // https://leetcode.com/problems/find-the-safest-path-in-a-grid/solutions/5158996/crust-on2-no-dijkstra-pure-bfs-explained-6nox
            // by https://leetcode.com/u/mortonjack/
            if damage > 0 {
                queue.push_back((new_health, nx, ny));
            } else {
                queue.push_front((new_health, nx, ny));
            }
        }
    }
    false
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
fn neighbors(x: usize, y: usize, n: usize, m: usize) -> impl Iterator<Item = (usize, usize)> {
    DIRECTIONS.into_iter().filter_map(move |(dx, dy)| {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32 {
            Some((nx as usize, ny as usize))
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::find_safe_walk;

    fn to_vector<const N: usize, const M: usize>(input: [[i32; M]; N]) -> Vec<Vec<i32>> {
        input.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    fn official1() {
        #[rustfmt::skip]
        let grid = [
            [0, 1, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 0, 1, 0],
        ];
        assert!(find_safe_walk(to_vector(grid), 1));
    }

    #[test]
    fn official2() {
        let grid = [
            [0, 1, 1, 0, 0, 0],
            [1, 0, 1, 0, 0, 0],
            [0, 1, 1, 1, 0, 1],
            [0, 0, 1, 0, 1, 0],
        ];
        assert!(!find_safe_walk(to_vector(grid), 3));
    }

    #[test]
    fn official3() {
        #[rustfmt::skip]
        let grid = [
            [1, 1, 1],
            [1, 0, 1],
            [1, 1, 1],
        ];
        assert!(find_safe_walk(to_vector(grid), 5));
    }

    #[test]
    fn official541() {
        let grid = [[1, 1, 1, 1]];
        assert!(!find_safe_walk(to_vector(grid), 4));
    }
}
