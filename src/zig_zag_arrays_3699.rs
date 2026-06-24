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

fn zig_zag_arrays_height(n: i32, height: usize) -> u32 {
    assert_ne!(0, height, "l < r");
    assert_ne!(1, height, "height > 1");
    assert!(n > 2);
    let mut zigs1 = vec![0; height];
    let mut zigs2 = (0..(height as u32)).collect::<Vec<_>>();
    for _ in 2..n {
        std::mem::swap(&mut zigs1, &mut zigs2);
        let mut zig_acc = 0;
        for i in 0..height {
            zigs2[i] = zig_acc;
            zig_acc += zigs1[height - i - 1];
            zig_acc %= MODULO;
        }
    }
    zigs2.iter().copied().fold(0_u32, |acc, x| (acc + x) % MODULO) * 2 % MODULO
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

    #[test]
    fn official50() {
        assert_eq!(16, zig_zag_arrays(4, 3, 5));
    }

    #[test]
    fn t1() {
        assert_eq!(140, zig_zag_arrays(5, 2, 5));
    }
}
