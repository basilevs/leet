// https://leetcode.com/problems/maximize-active-section-with-trade-i

pub fn max_active_sections_after_trade(s: String) -> i32 {
    let mut shortest_1_streak = usize::MAX;
    let mut longest_0_streak = 0usize;
    let mut current_1_streak = 0usize;
    let mut current_0_streak = 0usize;
    let mut last_0_streak: usize = 0usize;
    let mut total_1s = 0usize;
    // a triplet is a streak of 0s, followed by a streak of 1s, followed by a streak of 0s
    // the best triplet is one with the largest difference between sum of 0 streaks and 1 streak
    let mut best_triplet = 0usize;
    for c in s.into_bytes() {
        if c == b'1' {
            if current_0_streak > 0 {
                longest_0_streak = longest_0_streak.max(current_0_streak);
                if last_0_streak > 0 {
                    best_triplet = best_triplet.max(last_0_streak + current_0_streak);
                }
                last_0_streak = current_0_streak;
                current_0_streak = 0;
            }
            total_1s += 1;
            current_1_streak += 1;
        } else {
            if current_1_streak > 0 {
                if last_0_streak > 0 {
                    shortest_1_streak = shortest_1_streak.min(current_1_streak);
                }
                current_1_streak = 0;
            }
            current_0_streak += 1;
        }
    }
    if current_0_streak > 0 {
        longest_0_streak = longest_0_streak.max(current_0_streak);
        if last_0_streak > 0 {
            best_triplet = best_triplet.max(last_0_streak + current_0_streak);
        }
    }

    let best_disjoint = if longest_0_streak > 0 && shortest_1_streak > 0 {
        longest_0_streak.saturating_sub(shortest_1_streak)
    } else {
        0
    };


    if longest_0_streak > 0 {
        total_1s + best_triplet.max(best_disjoint)
    } else {
        total_1s
    }.try_into().unwrap()
    
}

#[cfg(test)]
mod tests {
    use super::max_active_sections_after_trade;

    #[test]
    fn official1() {
        assert_eq!(1, max_active_sections_after_trade("01".to_string()));
    }

    #[test]
    fn official2() {
        assert_eq!(4, max_active_sections_after_trade("0100".to_string()));
    }

    #[test]
    fn official3() {
        assert_eq!(7, max_active_sections_after_trade("1000100".to_string()));
    }

    #[test]
    fn official4() {
        assert_eq!(4, max_active_sections_after_trade("01010".to_string()));
    }

    #[test]
    fn official989() {
        assert_eq!(1, max_active_sections_after_trade("100".to_string()));
    }

    #[test]
    fn official993() {
        assert_eq!(2, max_active_sections_after_trade("1001".to_string()));
    }

    #[test]
    fn t1() {
        assert_eq!(1, max_active_sections_after_trade("001".to_string()));
    }

    #[test]
    fn t2() {
        assert_eq!(2, max_active_sections_after_trade("011".to_string()));
    }

    #[test]
    fn t3() {
        assert_eq!(2, max_active_sections_after_trade("110".to_string()));
    }

    #[test]
    fn t4() {
        assert_eq!(4, max_active_sections_after_trade("0110".to_string()));
    }

}
