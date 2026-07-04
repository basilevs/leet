// https://leetcode.com/problems/number-of-zigzag-arrays-ii

use crate::modint::ModInt;

pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
    // AI converted from broken Python exponentiation and previous solution
    let height = usize::try_from(r - l).expect("l < r") + 1;
    assert!(height >= 2, "constraints guarantee l < r");
    assert!(n >= 3, "constraints guarantee n >= 3");

    // 3699 reflection DP, lifted to matrix exponentiation so it scales to n up to 1e9.
    //
    // State v_t[i] = number of zigzag arrays of length t ending at value i whose last
    // step goes up. The 3699 transition is the generalized-Fibonacci recurrence
    //     v_t[i] = sum_{j = height - i}^{height - 1} v_{t-1}[j]
    // i.e. the linear map T[i][j] = 1 iff i + j >= height. The length-2 base state is
    // v_2[i] = i. The answer for length n is 2 * sum(T^{n-2} v_2) (factor 2 for the
    // up/down symmetry).
    let steps = u64::try_from(n - 2).unwrap();
    let powered = mat_pow(transition(height), steps);
    let sum: ModInt = powered
        .iter()
        .flat_map(|row| {
            row.iter()
                .enumerate()
                .map(|(j, &cell)| cell * ModInt::from(j))
        })
        .sum();
    i32::from(ModInt::from(2) * sum)
}

fn transition(height: usize) -> Vec<Vec<ModInt>> {
    (0..height)
        .map(|i| {
            (0..height)
                .map(|j| if i + j >= height { ModInt::ONE } else { ModInt::ZERO })
                .collect()
        })
        .collect()
}

fn mat_mul(a: &[Vec<ModInt>], b: &[Vec<ModInt>]) -> Vec<Vec<ModInt>> {
    let size = a.len();
    let mut out = vec![vec![ModInt::ZERO; size]; size];
    for i in 0..size {
        for k in 0..size {
            if a[i][k] == ModInt::ZERO {
                continue;
            }
            let aik = a[i][k];
            for j in 0..size {
                out[i][j] += aik * b[k][j];
            }
        }
    }
    out
}

fn mat_pow(mut base: Vec<Vec<ModInt>>, mut exp: u64) -> Vec<Vec<ModInt>> {
    let size = base.len();
    let mut result: Vec<Vec<ModInt>> = (0..size)
        .map(|i| {
            (0..size)
                .map(|j| if i == j { ModInt::ONE } else { ModInt::ZERO })
                .collect()
        })
        .collect();
    while exp > 0 {
        if exp & 1 == 1 {
            result = mat_mul(&result, &base);
        }
        base = mat_mul(&base, &base);
        exp >>= 1;
    }
    result
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
    fn t_len4() {
        assert_eq!(16, zig_zag_arrays(4, 3, 5));
    }

    #[test]
    fn t_len5() {
        assert_eq!(140, zig_zag_arrays(5, 2, 5));
    }

    #[test]
    fn t_big() {
        assert_eq!(180_547_325, zig_zag_arrays(123_456_789, 1, 10));
    }
}
