// https://leetcode.com/problems/distribute-elements-into-two-arrays-i

pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; nums.len()];
    
    let mut i = nums.iter().copied();
    let mut cursor1 = 0_usize;
    let mut cursor2 = nums.len() - 1;
    result[cursor1] = i.next().unwrap();
    result[cursor2] = i.next().unwrap();
    for n in i {
        if result[cursor1] > result[cursor2] {
            cursor1 += 1;
            result[cursor1] = n;
        } else {
            cursor2 -= 1;
            result[cursor2] = n;
        }
    }
    result[cursor2..].reverse();

    result    
}

#[cfg(test)]
mod tests {
    use super::result_array;

    #[test]
    fn official1() {
        assert_eq!(vec![2, 3, 1], result_array(vec![2, 1, 3]));
    }

    #[test]
    fn official2() {
        assert_eq!(vec![5, 3, 4, 8], result_array(vec![5, 4, 3, 8]));
    }
}
