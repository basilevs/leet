// https://leetcode.com/problems/maximum-building-height


struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn grow(self, increase: i32) -> Interval {
        Interval { start: self.start - increase, end: self.end + increase }
    }

    fn touches

}

pub fn max_building(n: i32, mut restrictions: Vec<Vec<i32>>) -> i32 {
    restrictions.sort_unstable_by_key(|x| x[1]);
    let mut left_position = 0;
    let mut left_height = 0;
    let mut right_position = n - 1;
    let mut right_height = n - 1;
    for r in restrictions {
        let position = r[0] - 1;
        let height = r[1];
        dbg!(left_position, left_height, position, height, right_position, right_height);
        debug_assert!(left_position < right_position);
        if !(left_position..=right_position).contains(&position) {
            continue;
        }

        let left_candidate_position = 

        if height < position - left_position + left_height {
            left_height = height;
            left_position = position;
            right_height = right_height.min(right_position - left_position + left_height);
        } else if height < right_position - position + right_height {
            right_height = height;
            right_position = position;
            left_height = left_height.min(right_position - left_position + right_height);
        } else {
            break;
        }
    }
    dbg!(left_position, left_height, right_position, right_height);
    left_height.max(right_height)
}

#[test]
fn official1() {
    assert_eq!(2, max_building(5, vec![vec![2, 1], vec![4, 1]]));
}

#[test]
fn official2() {
    assert_eq!(5, max_building(6, vec![]));
}

#[test]
fn official3() {
    assert_eq!(5, max_building(10, vec![vec![5, 3], vec![2, 5], vec![7, 4], vec![10, 3]]));
}

