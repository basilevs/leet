// https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut length = 0;
    let mut tail = head.as_ref();
    while tail.is_some() {
        tail = tail.and_then(|n| n.next.as_ref());
        length += 1;
    }

    if length == 1 {
        return None;
    }

    let mut pre_middle = head.as_mut();
    for _ in 2..=(length / 2) {
        pre_middle = pre_middle.unwrap().next.as_mut();
    }

    if let Some(node) = pre_middle {
        node.next = node.next.take().and_then(|p| p.next);
    }

    head
}

#[cfg(test)]
fn list(values: &[i32]) -> Option<Box<ListNode>> {
    values.iter().rev().fold(None, |next, &val| {
        let mut node = ListNode::new(val);
        node.next = next;
        Some(Box::new(node))
    })
}

#[cfg(test)]
fn values(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}

#[test]
fn official1() {
    let head = list(&[1, 3, 4, 7, 1, 2, 6]);

    assert_eq!(vec![1, 3, 4, 1, 2, 6], values(delete_middle(head)));
}

#[test]
fn official2() {
    let head = list(&[1, 2, 3, 4]);

    assert_eq!(vec![1, 2, 4], values(delete_middle(head)));
}

#[test]
fn official3() {
    let head = list(&[2, 1]);

    assert_eq!(vec![2], values(delete_middle(head)));
}

#[test]
fn single_node() {
    let head = list(&[1]);

    assert!(values(delete_middle(head)).is_empty());
}
