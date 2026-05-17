use std::mem::replace;

    pub fn can_reach(mut arr: Vec<i32>, start: i32) -> bool {
        let mut stack: Vec<i32> = vec![];
        // Task constratins prevent overflows in the math below
        debug_assert!(start >= 0);
        debug_assert!(start < arr.len() as i32);
        debug_assert!(arr.len() < (i32::MAX / 2) as usize);
        stack.push(start);
        loop {
            let Some(i) = stack.pop() else {
                return false;
            };
            let value = replace(&mut arr[i as usize], -1);
            if value == 0 {
                return true;
            }
            if value < 0 {
                continue;
            }
            debug_assert!(value >= 0);
            debug_assert!(value < arr.len() as _);
            for next in [i as i32 + value, i as i32 - value] {
                if next < 0 || next >= arr.len() as _  {
                    continue;
                }
                stack.push(next);
            }
        }
    }

#[cfg(test)]
fn can_reach_arr<const N: usize>(arr: [i32; N], start: i32) -> bool {
    can_reach(arr.to_vec(), start)
}

#[test]
fn official1() {
    assert!(can_reach_arr([4, 2, 3, 0, 3, 1, 2], 5));
}

#[test]
fn official2() {
    assert!(can_reach_arr([4, 2, 3, 0, 3, 1, 2], 0));
}

#[test]
fn official3() {
    assert!(!can_reach_arr([3, 0, 2, 1, 2], 2));
}

#[test]
fn zero_at_start() {
    assert!(can_reach_arr([0], 0));
}

#[test]
fn trapped_without_zero() {
    assert!(!can_reach_arr([1, 1, 1, 1], 0));
}

#[test]
fn can_reach_zero_by_backward_jump() {
    assert!(can_reach_arr([1, 2, 0, 1], 3));
}

