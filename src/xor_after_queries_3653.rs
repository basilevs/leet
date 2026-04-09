const M: i64 = 10i64.pow(9) + 7;
pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    for query in queries.into_iter() {
        let &[l, r, k, v] = query.as_slice() else {
            panic!("All queries are expected to consist of 4 integers");
        };
        let l = usize::try_from(l).expect("expected 0 <= l <= r");
        let r = usize::try_from(r).expect("expected 0 <= r < n");
        let k = usize::try_from(k).expect("expected 1 <= k <= n");
        let v = i64::from(v);
        for idx in (l..=r).step_by(k) {
            let num = &mut nums[idx];
            let temp = (i64::from(*num) * v) % M;
            dbg!(&idx, &num, &v, &temp);
            *num = temp.try_into().expect("Narrowing can not happen for a number that is a modulo of M");
        }
    }
    nums.into_iter().reduce(|a, b| a ^ b).unwrap_or(0)
}

pub fn xor_after_queries_unchecked(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
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
    nums.into_iter().reduce(|a, b| a ^ b).unwrap_or(0)
}

#[test]
fn official() {
    assert_eq!(4, xor_after_queries(vec!(1, 1, 1), to_vector(&[[0, 2, 1, 4]])));
    assert_eq!(
        4,
        xor_after_queries_unchecked(vec!(1, 1, 1), to_vector(&[[0, 2, 1, 4]]))
    );
    assert_eq!(
        31,
        xor_after_queries(
            vec!(2, 3, 1, 5, 4),
            vec!(vec!(1, 4, 2, 3), vec!(0, 2, 1, 2))
        )
    );
    assert_eq!(
        31,
        xor_after_queries_unchecked(
            vec!(2, 3, 1, 5, 4),
            vec!(vec!(1, 4, 2, 3), vec!(0, 2, 1, 2))
        )
    );
}

#[test]
fn error1() {
    let input = [[0,0,1,13],[0,0,1,17],[0,0,1,9],[0,0,1,18],[0,0,1,16],[0,0,1,6],[0,0,1,4],[0,0,1,11],[0,0,1,7],[0,0,1,18],[0,0,1,8],[0,0,1,15],[0,0,1,12]];
    assert_eq!(523618060, xor_after_queries(vec!(780), to_vector(&input)));
    assert_eq!(523618060, xor_after_queries_unchecked(vec!(780), to_vector(&input)));
}

#[cfg(test)]
fn to_vector<const SIZE: usize>(input: &[[i32; 4]; SIZE]) -> Vec<Vec<i32>> {
    input.iter().map(Vec::from).collect()
}