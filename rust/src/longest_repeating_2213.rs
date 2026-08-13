// https://leetcode.com/problems/longest-substring-of-one-repeating-character

use crate::segment_tree::SegmentTree;

pub fn longest_repeating(s: String, query_characters: String, query_indices: Vec<i32>) -> Vec<i32> {
    let mut tree = SegmentTree::from(s.into_bytes().iter().copied().map(|x| Segment::new(char::from(x))), merge, Segment::default());
    query_characters.into_bytes().into_iter().zip(query_indices).map(|(c, i)| {
        // dbg!(tree.query(..));
        tree.update(i as usize, Segment::new(char::from(c)));
        let full_range = tree.query(..);
        // dbg!(&full_range);
        full_range.max_count
    }).collect()
}

#[derive(Clone, Debug)]
struct Segment {
    left_letter: char,
    left_count: i32,
    right_letter: char,
    right_count: i32,
    max_letter: char,
    max_count: i32,
    homogenous: bool,
}

impl Segment {
    fn new(letter: char) -> Self {
        Segment {
            left_letter: letter,
            left_count: 1,
            right_letter: letter,
            right_count: 1,
            max_letter: letter,
            max_count: 1,
            homogenous: true,
        }
    }

    fn default() -> Self {
        Segment {
            left_letter: '\0',
            left_count: 0,
            right_letter: '\0',
            right_count: 0,
            max_letter: '\0',
            max_count: 0,
            homogenous: false,
        }
    }
}



fn merge(left: &Segment, right: &Segment) -> Segment {
    let mut max_letter = left.max_letter;
    let mut max_count = left.max_count;

    if right.max_count > max_count {
        max_letter = right.max_letter;
        max_count = right.max_count;
    }

    let mut left_count = left.left_count;
    let mut right_count = right.right_count;
    if left.right_letter == right.left_letter {
        let combined_count = left.right_count + right.left_count;
        if combined_count > max_count {
            max_letter = left.right_letter;
            max_count = combined_count;
        }
        if left.homogenous {
            left_count += right.left_count;
        }

        if right.homogenous {
            right_count += left.right_count;
        }

    }

    let result = Segment {
        left_letter: left.left_letter,
        left_count,
        right_letter: right.right_letter,
        right_count,
        max_letter,
        max_count,
        homogenous: left.homogenous && right.homogenous && left.right_letter == right.left_letter,
    };
    // dbg!(&left, &right, &result);
    result
}

#[cfg(test)]
mod tests {
    use super::longest_repeating;

    #[test]
    fn official1() {
        assert_eq!(
            vec![3, 3, 4],
            longest_repeating(
                "babacc".to_string(),
                "bcb".to_string(),
                vec![1, 3, 3]
            )
        );
    }

    #[test]
    fn official2() {
        assert_eq!(
            vec![2, 3],
            longest_repeating(
                "abyzz".to_string(),
                "aa".to_string(),
                vec![2, 1]
            )
        );
    }

    #[test]
    fn official45() {
        assert_eq!(
            vec![1,1,2,2,2,2,2,2,2,1],
            longest_repeating(
                "geuqjmt".to_string(),
                "bgemoegklm".to_string(),
                vec![3,4,2,6,5,6,5,4,3,2]
            )
        );
    }

    #[test]
    fn minimal45() {
        assert_eq!(
            vec![2],
            longest_repeating(
                "geebgoe".to_string(),
                "g".to_string(),
                vec![5]
            )
        );
    }
}
