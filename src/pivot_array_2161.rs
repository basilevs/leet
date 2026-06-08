// https://leetcode.com/problems/partition-array-according-to-given-pivot

pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut lesser = 0;
    let mut equal = 0;
    for &i in nums.iter() {
        if i == pivot {
            equal += 1_usize;
        } else if i < pivot {
            lesser += 1_usize;
        }
    }
    if lesser >= nums.len() {
        return nums;
    }
    if lesser == 0 && equal == 0 {
        return nums;
    }

    let mut result = vec![pivot; nums.len()];

    let mut lesser_cursor = 0;
    let mut greater_cursor = lesser + equal;

    for i in nums {
        if i < pivot {
            result[lesser_cursor] = i;
            lesser_cursor += 1;
        } else if i > pivot {
            result[greater_cursor] = i;
            greater_cursor += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::pivot_array;

    #[test]
    fn official1() {
        let nums = vec![9, 12, 5, 10, 14, 3, 10];
        let pivot = 10;
        let expected = vec![9, 5, 3, 10, 10, 12, 14];

        assert_eq!(expected, pivot_array(nums, pivot));
    }

    #[test]
    fn official2() {
        let nums = vec![-3, 4, 3, 2];
        let pivot = 2;
        let expected = vec![-3, 2, 4, 3];

        assert_eq!(expected, pivot_array(nums, pivot));
    }
}
