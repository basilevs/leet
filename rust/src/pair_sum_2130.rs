// https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list/

use std::cell::RefCell;


// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

thread_local! {
    static COPY: RefCell<Vec<i32>> = RefCell::new(Vec::with_capacity(100_000));
}

pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
    COPY.with_borrow_mut(|copy| {
        copy.clear();

        while let Some(node) = head {
            copy.push(node.val);
            head = node.next;
        }
        // copy.extend(from_fn(|| {
        //     let result = head.as_ref().map(|t| t.val);
        //     head = head.take().and_then(|t| t.next);
        //     result
        // }));

        let n = copy.len();
        debug_assert_eq!(0, n % 2);
        copy.iter().rev().zip(copy.iter()).take(n / 2).map(|(x, y)| x + y).max().unwrap()
    })
}

