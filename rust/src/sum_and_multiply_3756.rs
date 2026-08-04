// https://leetcode.com/problems/concatenate-non-zero-digits-and-multiply-by-sum-ii

use std::{iter::successors, sync::OnceLock};

use crate::modint::ModInt;

static POW10: OnceLock<Vec<ModInt>> = OnceLock::new();

fn pow10(exp: usize) -> ModInt {
    let table = POW10.get_or_init(|| {
        successors(Some(ModInt::ONE), |&p| Some(p * ModInt::from(10)))
            .take(100_000)
            .collect::<Vec<_>>()
    });
    table[exp]
}

pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let bs = s.into_bytes();
    // (digits, sum, non_zero_counts)
    let prefixes = bs.into_iter().scan((ModInt::from(0), 0u32, 0u32), |acc, b| {
        if b != b'0' {
            acc.0 = acc.0 * ModInt::from(10) + ModInt::from(b - b'0');
            acc.1 += u32::from(b - b'0');
            acc.2 += 1;
        }
        Some(*acc)
    }).collect::<Vec<_>>();
    queries.into_iter().map(|q| {
        let l: usize = q[0].try_into().expect("0 <= l < m");
        let r: usize = q[1].try_into().expect("0 <= r < m");
        if l == 0 {
            prefixes[r].0 * ModInt::from(prefixes[r].1)
        } else {
            let (d_r, s_r, nz_c_r) = prefixes[r];
            let (d_l, s_l, nz_c_l) = prefixes[l - 1];
            (d_r - d_l * pow10((nz_c_r - nz_c_l) as usize)) * ModInt::from(s_r - s_l)
        }
    }).map(ModInt::into).collect()
}

#[cfg(test)]
mod tests {
    use super::sum_and_multiply;

    fn to_queries(input: &[[i32; 2]]) -> Vec<Vec<i32>> {
        input.iter().map(Vec::from).collect()
    }

    #[test]
    fn official1() {
        let s = "10203004".to_string();
        #[rustfmt::skip]
        let queries = [
            [0, 7],
            [1, 3],
            [4, 6],
        ];
        let expected = vec![12340, 4, 9];
        assert_eq!(expected, sum_and_multiply(s, to_queries(&queries)));
    }

    #[test]
    fn official2() {
        let s = "1000".to_string();
        #[rustfmt::skip]
        let queries = [
            [0, 3],
            [1, 1],
        ];
        let expected = vec![1, 0];
        assert_eq!(expected, sum_and_multiply(s, to_queries(&queries)));
    }

    #[test]
    fn official3() {
        let s = "9876543210".to_string();
        #[rustfmt::skip]
        let queries = [
            [0, 9],
        ];
        let expected = vec![444444137];
        assert_eq!(expected, sum_and_multiply(s, to_queries(&queries)));
    }

        #[test]
    fn t1() {
        let s = "10203004".to_string();
        #[rustfmt::skip]
        let queries = [
            [1, 3],
        ];
        let expected = vec![4];
        assert_eq!(expected, sum_and_multiply(s, to_queries(&queries)));
    }

    #[test]
    fn large_digit_sum() {
        // 40 nines: x = 40 concatenated nines, sum = 40 * 9 = 360.
        // The digit sum exceeds 255, exercising accumulation beyond a byte.
        let s = "9".repeat(40);
        // x mod (1e9+7) for "9" * 40, then multiplied by 360, taken mod (1e9+7).
        let expected = vec![643_599_584];
        assert_eq!(expected, sum_and_multiply(s, to_queries(&[[0, 39]])));
    }

    #[test]
    fn largest_input() {
        // m = 10^5, all non-zero digits, so exponents reach the top of POW10.
        // Two overlapping full-span queries push the total span past m, forcing
        // the prefix path; the first query has l > 0 to exercise pow10.
        let s = "123456789".repeat(11_112)[..100_000].to_string();
        assert_eq!(s.len(), 100_000);
        let expected = vec![925_156_883, 153_926_807];
        assert_eq!(expected, sum_and_multiply(s, to_queries(&[[1, 99_999], [0, 99_999]])));
    }
}
