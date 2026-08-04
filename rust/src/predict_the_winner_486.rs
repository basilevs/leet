// https://leetcode.com/problems/predict-the-winner

pub fn predict_the_winner(nums: Vec<i32>) -> bool {
    let mut state = State {
        memo: (0..nums.len()).map(|i| vec![None; i + 1]).rev().collect(),
        nums,
    };
    // dbg!(&state.memo);
    state.compute(0, state.nums.len() - 1) >= 0
}

struct State {
    // Oops, the cacnonical solution does unconditional DP, not memoization.
    memo: Vec<Vec<Option<i32>>>,
    nums: Vec<i32>,
}

impl State {
    fn get(&self, left: usize, right: usize) -> Option<i32> {
        debug_assert!(left <= right);
        self.memo[left][right - left]
    }

    // maximum player one's score minus player two's score
    pub fn compute(&mut self, left: usize, right: usize) -> i32 {
        if let Some(score) = self.get(left, right) {
            return score;
        }
        
        let score = if left == right {
            self.nums[left]
        } else if left + 1 == right {
            self.nums[left].abs_diff(self.nums[right]) as i32
        } else {
            let left_score = self.nums[left] - self.compute(left + 1, right);
            let right_score = self.nums[right] - self.compute(left, right - 1);
            left_score.max(right_score)
        };

        self.memo[left][right - left] = Some(score);
        score
    }

}

#[cfg(test)]
mod tests {
    use super::predict_the_winner;

    #[test]
    fn official1() {
        let nums = vec![1, 5, 2];
        assert_eq!(false, predict_the_winner(nums));
    }

    #[test]
    fn official2() {
        let nums = vec![1, 5, 233, 7];
        assert_eq!(true, predict_the_winner(nums));
    }
}
