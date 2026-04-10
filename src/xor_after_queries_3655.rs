const M: i64 = 10i64.pow(9) + 7;

pub fn xor_after_queries_sliced(nums: &mut [i32], queries: &[Vec<i32>]) -> i32 {
    for query in queries {
        let (l, r, k, v) = {
            (
                query[0] as usize,
                query[1] as usize,
                query[2] as usize,
                i64::from(query[3]),
            )
        };
        for idx in (l..=r).step_by(k) {
            let num = &mut nums[idx];
            let temp = (i64::from(*num) * v) % M;
            *num = temp as i32;
        }
    }
    nums.into_iter().fold(0, |a, b| a ^ *b)
}