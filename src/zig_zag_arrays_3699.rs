// https://leetcode.com/problems/number-of-zigzag-arrays-i

pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
    let height  = usize::try_from(r-l).expect("l < r") + 1;
    assert_ne!(0, height, "l < r");
    if height == 1 {
        return 1;
    }
    i32::try_from(zig_zag_arrays_height(n, height)).unwrap()
}

const MODULO: u32 = 1_000_000_007;
fn add(a: u32, b: &u32) -> u32 {
    (a % MODULO + b % MODULO) % MODULO
}

fn sub(a: u32, b: u32) -> u32 {
    (MODULO + a % MODULO - b % MODULO) % MODULO
}


fn zig_zag_arrays_height(n: i32, height: usize) -> u32 {
    dbg!(n, height); // suppress unused variable warnings for function arguments
    assert_ne!(0, height, "l < r");
    assert_ne!(1, height, "height > 1");
    assert!(n > 2);
    let mut prev_step2 = vec![0; height];
    let mut prev_step1 = vec![1; height];
    let mut result = vec![height as u32; height];
    for _ in 2..n {
        dbg!(&result);
        std::mem::swap(&mut prev_step1, &mut prev_step2);
        std::mem::swap(&mut prev_step1, &mut result);
        result.fill(0);
        let mut prev_step1_below = 0_u32;
        let mut prev_step2_below = 0_u32;
        let prev_step1_sum = prev_step1.iter().fold(0, add);
        let prev_step2_sum = prev_step2.iter().fold(0, add);
        let mut prev_step1_above = prev_step1_sum;
        let mut prev_step2_above = prev_step2_sum;
        for i in 0..height {
            let monothonic = ((u64::from(prev_step1_below) * u64::from(prev_step2_below) + u64::from(prev_step1_above) * u64::from(prev_step2_above)) % u64::from(MODULO)) as u32;
            let all_trajectories = (u64::from(prev_step1_sum) * u64::from(prev_step2_sum) % u64::from(MODULO)) as u32;
            result[i] = sub(all_trajectories, monothonic);
            
            if i < height {
                prev_step1_below = add(prev_step1_below, &prev_step1[i]);
            }
            if i > 0 {
                prev_step1_above = sub(prev_step1_above, prev_step1[i-1]);
                prev_step2_below = add(prev_step2_below, &prev_step2[i-1]);
            }
            if i > 1 {
                prev_step2_above = sub(prev_step2_above, prev_step2[i-2]);
            }
        }
    }


    result.iter().fold(0, add)
}

#[cfg(test)]
mod tests {
    use super::zig_zag_arrays;

    #[test]
    fn official1() {
        assert_eq!(2, zig_zag_arrays(3, 4, 5));
    }

    #[test]
    fn official2() {
        assert_eq!(10, zig_zag_arrays(3, 1, 3));
    }
}
