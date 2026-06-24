// https://leetcode.com/problems/number-of-zigzag-arrays-ii

const MODULO: u64 = 1_000_000_007;

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
    let mut sum = 0u64;
    for row in &powered {
        let mut acc = 0u128;
        for (j, &cell) in row.iter().enumerate() {
            acc += u128::from(cell) * j as u128;
        }
        sum = (sum + (acc % u128::from(MODULO)) as u64) % MODULO;
    }
    i32::try_from(2 * sum % MODULO).unwrap()
}

fn transition(height: usize) -> Vec<Vec<u64>> {
    (0..height)
        .map(|i| (0..height).map(|j| u64::from(i + j >= height)).collect())
        .collect()
}

fn mat_mul(a: &[Vec<u64>], b: &[Vec<u64>]) -> Vec<Vec<u64>> {
    let size = a.len();
    let mut out = vec![vec![0u64; size]; size];
    for i in 0..size {
        for k in 0..size {
            if a[i][k] == 0 {
                continue;
            }
            let aik = u128::from(a[i][k]);
            for j in 0..size {
                let v = u128::from(out[i][j]) + aik * u128::from(b[k][j]);
                out[i][j] = (v % u128::from(MODULO)) as u64;
            }
        }
    }
    out
}

fn mat_pow(mut base: Vec<Vec<u64>>, mut exp: u64) -> Vec<Vec<u64>> {
    let size = base.len();
    let mut result: Vec<Vec<u64>> = (0..size)
        .map(|i| (0..size).map(|j| u64::from(i == j)).collect())
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
