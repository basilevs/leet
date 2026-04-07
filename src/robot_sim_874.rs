use std::collections::{HashMap, HashSet};

/// https://leetcode.com/problems/walking-robot-simulation
pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    hashed(commands, obstacles)
}

pub fn naive_factored(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    execute(&commands, &naive_unobstructed_length(&obstacles))
}

pub fn hashed(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    execute(&commands, &hashed_unobstructed_length(&obstacles))
}

// Fastest solution from LettCode
pub fn winner(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut dir = 0;
    let mut x = 0;
    let mut y = 0;
    let mut max = 0;
    let obstacles_set: HashSet<(i32, i32)> = obstacles
        .into_iter()
        .map(|v| (v[0], v[1]))
        .collect();

    for &cmd in commands.iter() {
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
fn execute<F>(commands: &Vec<i32>, unobstructed_length: &F) -> i32
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
                x += (vx * length as i8) as i32;
                y += (vy * length as i8) as i32;
                max = max.max(x * x + y * y);
            }
        }
    }
    max
}

pub fn naive(commands: &Vec<i32>, obstacles: &Vec<Vec<i32>>) -> i32 {
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

fn naive_unobstructed_length(obstacles: &Vec<Vec<i32>>) -> impl Fn(i32, i32, i8, i8) -> u8 {
    move |mut x: i32, mut y: i32, x_change: i8, y_change: i8| {
        let vx = x_change.signum();
        let vy = y_change.signum();
        assert_eq!(0, x_change * y_change);
        assert!((-9..=9).contains(&x_change));
        assert!((-9..=9).contains(&y_change));
        let units = (x_change + y_change).abs();
        let mut length = 0u8;
        for _ in 0..units {
            let next_x = x + vx as i32;
            let next_y = y + vy as i32;
            if obstacles.iter().any(|o| o[0] == next_x && o[1] == next_y) {
                break;
            }
            length += 1;
            (x, y) = (next_x, next_y);
        }
        length
    }
}

fn hashed_unobstructed_length(obstacles: &Vec<Vec<i32>>) -> impl Fn(i32, i32, i8, i8) -> u8 {
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
        .unwrap_or(change.abs() as u8)
}

fn free_length_in_sorted(obstacles: &[i32], start: i32, change: i8) -> u8 {
    assert_ne!(0, change);
    assert!((-9..=9).contains(&change));
    let sign = change.signum();
    debug_assert!(obstacles.is_sorted());
    let max_length = change.abs() as u8;
    let next_obstacle = match obstacles.binary_search(&start) {
        Ok(exact_index) => obstacles.get((exact_index as i32 + sign as i32) as usize),
        Err(insertion_index) => {
            let shift: i32 = if sign > 0 { 0 } else { -1 };
            let obstacle_index = insertion_index as i32 + shift;
            if obstacle_index < 0 {
                None
            } else {
                obstacles.get(obstacle_index as usize)
            }
        }
    };
    next_obstacle
        .map(|o| ((o - start).abs() - 1).min(max_length as i32) as u8)
        .unwrap_or(max_length)
}

#[cfg(test)]
fn assert_all_algoritms_result(commands: &Vec<i32>, obstacles: &Vec<Vec<i32>>, result: i32) {
    assert_eq!(result, execute(&commands, &naive_unobstructed_length(&obstacles)));
    assert_eq!(result, execute(&commands, &hashed_unobstructed_length(&obstacles)));
    assert_eq!(result, naive(&commands, &obstacles));
}

#[test]
fn internals() {
    assert_eq!(1, free_length_in_sorted([2].as_slice(), 0, 4))
}

#[test]
fn no_obstacles() {
    let commands = vec!(4, -1, 3);
    let obstacles = vec!();
    assert_all_algoritms_result(&commands, &obstacles, 25);
}

#[test]
fn east_obstacle() {
    assert_all_algoritms_result(&[4, -1, 4, -2, 4].into(), &[[2, 4].into()].into(), 65);
}

#[test]
fn up_down() {
    assert_all_algoritms_result(&[4, -1, -1, 4].into(), &[].into(), 16);
}
