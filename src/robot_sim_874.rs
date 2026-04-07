use std::collections::{HashMap, HashSet};

/// https://leetcode.com/problems/walking-robot-simulation
#[must_use]
pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    hashed(commands, obstacles)
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn naive_factored(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    execute(&commands, &naive_unobstructed_length(&obstacles))
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn hashed(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    execute(&commands, &hashed_unobstructed_length(&obstacles))
}

// Fastest solution from LettCode
#[must_use]
pub fn winner(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut dir = 0;
    let mut x = 0;
    let mut y = 0;
    let mut max = 0;
    let obstacles_set: HashSet<(i32, i32)> = obstacles.into_iter().map(|v| (v[0], v[1])).collect();

    for cmd in commands {
        if cmd == -2 {
            dir = (dir + 3) % 4;
        } else if cmd == -1 {
            dir = (dir + 1) % 4;
        } else {
            let (dx, dy) = dirs[dir];
            for _ in 0..cmd {
                let nx = x + dx;
                let ny = y + dy;
                if obstacles_set.contains(&(nx, ny)) {
                    break;
                }
                x = nx;
                y = ny;
                max = max.max(x * x + y * y);
            }
        }
    }
    max
}

/// * unobstructed_length(x, y, x_change, y_change) returns a length of unobstructed path
fn execute<F>(commands: &[i32], unobstructed_length: &F) -> i32
where
    F: Fn(i32, i32, i8, i8) -> u8,
{
    // Current position
    let mut x = 0;
    let mut y = 0;
    // Current facing direction
    let mut vx = 0i8;
    let mut vy = 1i8;
    let mut max = 0;
    for command in commands {
        assert_eq!(1, vx * vx + vy * vy);
        match command {
            // turn right
            // https://en.wikipedia.org/wiki/Rotation_matrix
            -1 => {
                (vx, vy) = (vy, -vx);
            }
            // turn left
            -2 => {
                (vx, vy) = (-vy, vx);
            }
            // move in facing direction
            units => {
                assert!((1..=9).contains(units));
                let units = i8::try_from(*units).expect("units are in [1, 9]");
                let x_change = vx * units;
                let y_change = vy * units;
                let length = unobstructed_length(x, y, x_change, y_change);
                x += i32::from(vx * length as i8);
                y += i32::from(vy * length as i8);
                max = max.max(x * x + y * y);
            }
        }
    }
    max
}

#[must_use]
pub fn naive(commands: &[i32], obstacles: &[Vec<i32>]) -> i32 {
    // Current position
    let mut x = 0;
    let mut y = 0;
    // Current facing direction
    let mut vx = 0;
    let mut vy = 1;
    let mut max = 0;
    for command in commands {
        assert_eq!(1, vx * vx + vy * vy);
        match command {
            // turn right
            // https://en.wikipedia.org/wiki/Rotation_matrix
            -1 => {
                (vx, vy) = (vy, -vx);
            }
            // turn left
            -2 => {
                (vx, vy) = (-vy, vx);
            }
            // move in facing direction
            units => {
                for _ in 0..*units {
                    let next_x = x + vx;
                    let next_y = y + vy;
                    if obstacles.iter().any(|o| o[0] == next_x && o[1] == next_y) {
                        break;
                    }
                    (x, y) = (next_x, next_y);
                }
                max = max.max(x * x + y * y);
            }
        }
    }
    max
}

fn naive_unobstructed_length(obstacles: &[Vec<i32>]) -> impl Fn(i32, i32, i8, i8) -> u8 + '_ {
    move |mut x: i32, mut y: i32, x_change: i8, y_change: i8| {
        let vx = x_change.signum();
        let vy = y_change.signum();
        assert_eq!(0, x_change * y_change);
        assert!((-9..=9).contains(&x_change));
        assert!((-9..=9).contains(&y_change));
        let units = (x_change + y_change).abs();
        let mut length = 0u8;
        for _ in 0..units {
            let next_x = x + i32::from(vx);
            let next_y = y + i32::from(vy);
            if obstacles.iter().any(|o| o[0] == next_x && o[1] == next_y) {
                break;
            }
            length += 1;
            (x, y) = (next_x, next_y);
        }
        length
    }
}

fn hashed_unobstructed_length(obstacles: &[Vec<i32>]) -> impl Fn(i32, i32, i8, i8) -> u8 {
    let mut by_x: HashMap<i32, Vec<i32>> = HashMap::with_capacity(obstacles.len());
    let mut by_y: HashMap<i32, Vec<i32>> = HashMap::with_capacity(obstacles.len());
    for point in obstacles {
        by_x.entry(point[0]).or_default().push(point[1]);
        by_y.entry(point[1]).or_default().push(point[0]);
    }
    for ys in by_x.values_mut() {
        ys.sort();
    }
    for xs in by_y.values_mut() {
        xs.sort();
    }

    move |x: i32, y: i32, x_change: i8, y_change: i8| {
        assert!((-9..=9).contains(&x_change));
        assert!((-9..=9).contains(&y_change));
        assert_eq!(0, x_change * y_change);
        assert_ne!(0, x_change * x_change + y_change * y_change);
        if x_change == 0 {
            unobstructed_length_along_axis(&by_x, x, y, y_change)
        } else if y_change == 0 {
            unobstructed_length_along_axis(&by_y, y, x, x_change)
        } else {
            unreachable!("expected movement along a single axis");
        }
    }
}

fn unobstructed_length_along_axis(
    obstacles: &HashMap<i32, Vec<i32>>,
    fixed_coord: i32,
    original_coord: i32,
    change: i8,
) -> u8 {
    obstacles
        .get(&fixed_coord)
        .map(|os| free_length_in_sorted(os, original_coord, change))
        .unwrap_or(change.unsigned_abs())
}

fn free_length_in_sorted(obstacles: &[i32], start: i32, change: i8) -> u8 {
    assert_ne!(0, change);
    assert!((-9..=9).contains(&change));
    debug_assert!(i32::try_from(obstacles.len()).is_ok(), "obstacle slice length fits i32");
    let sign = change.signum();
    debug_assert!(obstacles.is_sorted());
    let max_length = change.unsigned_abs();
    let next_obstacle = match obstacles.binary_search(&start) {
        Ok(exact_index) => {
            let neighbour =
                i32::try_from(exact_index).expect("obstacle index fits i32") + i32::from(sign);
            usize::try_from(neighbour)
                .ok()
                .and_then(|i| obstacles.get(i))
        }
        Err(insertion_index) => {
            let shift: i32 = if sign > 0 { 0 } else { -1 };
            let obstacle_index =
                i32::try_from(insertion_index).expect("insertion index fits i32") + shift;
            usize::try_from(obstacle_index)
                .ok()
                .and_then(|i| obstacles.get(i))
        }
    };
    next_obstacle
        .map(|o| {
            u8::try_from(((o - start).abs() - 1).min(i32::from(max_length)))
                .expect("free length is bounded by max_length which is u8")
        })
        .unwrap_or(max_length)
}

#[cfg(test)]
fn assert_all_algoritms_result(commands: &[i32], obstacles: &[Vec<i32>], result: i32) {
    assert_eq!(
        result,
        execute(commands, &naive_unobstructed_length(obstacles))
    );
    assert_eq!(
        result,
        execute(commands, &hashed_unobstructed_length(obstacles))
    );
    assert_eq!(result, naive(commands, obstacles));
}

#[test]
fn internals() {
    assert_eq!(1, free_length_in_sorted([2].as_slice(), 0, 4))
}

#[test]
fn no_obstacles() {
    let commands = vec![4, -1, 3];
    let obstacles = vec![];
    assert_all_algoritms_result(&commands, &obstacles, 25);
}

#[test]
fn east_obstacle() {
    let commands = [4, -1, 4, -2, 4];
    let obstacles = vec![vec![2, 4]];
    assert_all_algoritms_result(&commands, &obstacles, 65);
}

#[test]
fn up_down() {
    let commands = [4, -1, -1, 4];
    let obstacles: Vec<Vec<i32>> = vec![];
    assert_all_algoritms_result(&commands, &obstacles, 16);
}
