// https://leetcode.com/problems/find-the-maximum-number-of-elements-in-subset

use std::{collections::HashMap};

pub fn maximum_length(nums: Vec<i32>) -> i32 {
    let mut duplicates = HashMap::with_capacity(nums.len());
    let mut count_1 = 0;
    for &num in &nums {
        if num == 1 {
            count_1 += 1;
        }
        duplicates.entry(num).and_modify(|x| *x=true).or_insert(false);
    }

    let depth_1 = (count_1 + 1 ) / 2;
    let mut memo = HashMap::with_capacity(duplicates.len());
    let depth_others = nums.iter().map(|x| measure_depth(*x, &duplicates, &mut memo)).max().unwrap_or(0);
    depth_others.max(depth_1) * 2 - 1
}

fn measure_depth(x: i32, duplicates: &HashMap<i32, bool>, memo: &mut HashMap<i32, i32>) -> i32 {
    if x < 2 {
        return 1;
    }
    if let Some(&cached) = memo.get(&x) {
        return cached;
    }
    let depth =  if let Some(&duplicated) = duplicates.get(&x) {
        if duplicated {
            x.checked_mul(x).map(|next| 1 + measure_depth(next, duplicates, memo)).unwrap_or(1)
        } else {
            1
        }
    } else {
        0
    };

    memo.insert(x, depth);
    depth
}

#[cfg(test)]
mod tests {
    use super::maximum_length;

    #[test]
    fn official1() {
        assert_eq!(3, maximum_length(vec![5,4,1,2,2]));
    }

    #[test]
    fn official2() {
        assert_eq!(1, maximum_length(vec![1,3,2,4]));
    }

    #[test]
    fn official718() {
        assert_eq!(9, maximum_length(vec![1,1,1,1,1,1,1,1,1,1,2,4,8,16,32,64,128,256,512,1024]));
    }

    #[test]
    fn official216() {
        assert_eq!(5, maximum_length(vec![15,15,225,225,50625,50625]));
    }
}