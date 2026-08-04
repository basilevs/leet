// https://leetcode.com/problems/partition-array-according-to-given-pivot

pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut lesser = 0;
    let mut equal = 0;
    for &n in &nums {
        if n == pivot {
            equal += 1;
        } else if n < pivot {
            lesser += 1;
        }
    }

    let mut result = vec![pivot; nums.len()];
    let (less_slots, rest) = result.split_at_mut(lesser);
    let (_pivots, greater_slots) = rest.split_at_mut(equal);
    let mut less_slots = less_slots.iter_mut();
    let mut greater_slots = greater_slots.iter_mut();

    for n in nums {
        if n < pivot {
            *less_slots.next().unwrap() = n;
        } else if n > pivot {
            *greater_slots.next().unwrap() = n;
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
