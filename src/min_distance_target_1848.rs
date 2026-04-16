pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
    let mut result = 0;
    
    let start: usize = start.try_into().expect("Start should be non-negative");
    let result_upper_bound = (nums.len() - start).max(start + 1);
    while result <= result_upper_bound {
        let a = nums.get(start + result).copied();
        let b = nums.get(start - result).copied();
        if a == Some(target) || b == Some(target) {
            return result as i32;
        }
        result += 1;
    }
    panic!("Target {} should exist", target);
}
