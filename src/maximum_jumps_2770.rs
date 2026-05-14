    pub fn maximum_jumps(nums: Vec<i32>, mut target: i32) -> i32 {
        target = target.abs();
        if nums.is_empty() {
            return -1;
        }
        let mut distances = vec![-1; nums.len()];
        distances[0] = 0;
        for (i, &value) in nums.iter().enumerate().skip(1) {
            let mut max = -1;
            for (j, &distance) in distances[0..i].iter().enumerate() {
                if distance < 0 {
                    continue;
                }
                if (nums[j] - value).abs() > target {
                    continue;
                }
                max = max.max(distance + 1);
            }
            distances[i] = max;
        }
        distances.last().copied().unwrap_or(-1)
    }

#[test]
fn official1() {
    assert_eq!(3, maximum_jumps(vec![1,3,6,4,1,2], 2));
}

#[test]
fn official2() {
    assert_eq!(5, maximum_jumps(vec![1,3,6,4,1,2], 3));
}

#[test]
fn official3() {
    assert_eq!(-1, maximum_jumps(vec![1,3,6,4,1,2], 0));
}

