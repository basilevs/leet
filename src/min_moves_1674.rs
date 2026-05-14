    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let n = nums.len();
        if n < 2 || limit <= 1 {
            return 0;
        }
        let limit = usize::try_from(limit).expect("limit >= 1");
        // Count pairs by sum
        let mut sums = vec![0_usize; limit * 2 + 1];
        // Count pairs by maximum
        let mut maxima = vec![0_usize; limit + 1];
        // Count pairs by minimum
        let mut minima = vec![0_usize; limit + 1];
        for (i, &v) in nums.iter().take(n/2).enumerate() {
            let partner = nums[n - i - 1];
            let sum = usize::try_from(v + partner).expect("nums[i] > 1");
            if let Some(x) = sums.get_mut(sum) {
                *x += 1;
            }
            maxima[v.max(partner) as usize] += 1;
            minima[v.min(partner) as usize] += 1;
        }
        // A target sum for each pair is achievable by either:
        // - exact match from the start
        // - changing one value, if max + limit >= sum and min < sum
        // - changing both values

        // dbg!(&sums, &minima, &maxima);
        let mut one_achievable = 0;
        let mut max = 0;
        for i in 1..(2*limit) {
            one_achievable += minima.get(i-1).unwrap_or(&0);
            one_achievable -= i.checked_sub(limit+1).and_then(|j| maxima.get(j)).unwrap_or(&0);
            let saved_steps = one_achievable + sums[i];
            max = max.max(saved_steps);
            // dbg!(i,one_achievable,sums[i], saved_steps);
            if max == n  {
                break;
            }
        }
        n.checked_sub(max).expect("Underflow") as _
    }


#[test]
fn official1() {
    assert_eq!(1, min_moves(vec![1,2,4,3], 4));
}

#[test]
fn official2() {
    assert_eq!(2, min_moves(vec![1,2,2,1], 2));
}

#[test]
fn official3() {
    assert_eq!(0, min_moves(vec![1,2,1,2], 2));
}

#[test]
fn official7() {
    assert_eq!(4, min_moves(vec![1,3,1,1,1,2,3,
        2,3,1,3,2,1,3], 3));
}

#[test]
fn error1() {
    assert_eq!(0, min_moves(vec![1,2,3,4,5,6], 6));
}