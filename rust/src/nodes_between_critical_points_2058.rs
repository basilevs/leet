// https://leetcode.com/problems/find-the-minimum-and-maximum-number-of-nodes-between-critical-points/

use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn nodes_between_critical_points(
        head: Option<Rc<RefCell<ListNode>>>,
    ) -> Vec<i32> {
        let mut cur = match &head {
            Some(node) => node.borrow_mut().next.take(),
            None => return vec![-1, -1],
        };

        let mut prev_val = head
            .as_ref()
            .map(|node| node.borrow().val)
            .unwrap_or_default();
        let mut i = 1;
        let mut first_critical = -1;
        let mut last_critical = -1;
        let mut min_distance = -1;

        while let Some(node) = cur.clone() {
            let borrowed = node.borrow();
            let v = borrowed.val;
            let nxt = borrowed.next.as_ref().map(|n| n.borrow().val);

            if let Some(nxt_val) = nxt {
                if (v > prev_val && v > nxt_val) || (v < prev_val && v < nxt_val) {
                    if first_critical == -1 {
                        first_critical = i;
                    } else if min_distance == -1 || i - last_critical < min_distance {
                        min_distance = i - last_critical;
                    }
                    last_critical = i;
                }
            }

            prev_val = v;
            drop(borrowed);
            cur = node.borrow_mut().next.take();
            i += 1;
        }

        if min_distance == -1 {
            vec![-1, -1]
        } else {
            vec![min_distance, last_critical - first_critical]
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(vals: Vec<i32>) -> Option<Rc<RefCell<ListNode>>> {
        let mut head = None;
        for &val in vals.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Rc::new(RefCell::new(node)));
        }
        head
    }

    #[test]
    fn official1() {
        let head = create_list(vec![3, 1]);
        assert_eq!(vec![-1, -1], Solution::nodes_between_critical_points(head));
    }

    #[test]
    fn official2() {
        let head = create_list(vec![5, 3, 1, 2, 5, 1, 2]);
        assert_eq!(vec![1, 3], Solution::nodes_between_critical_points(head));
    }

    #[test]
    fn official3() {
        let head = create_list(vec![1, 3, 2, 2, 3, 2, 2, 2, 7]);
        assert_eq!(vec![3, 3], Solution::nodes_between_critical_points(head));
    }
}
