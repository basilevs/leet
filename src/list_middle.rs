use std::option::Option;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn middle_node_cloning(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;
        
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        return slow.clone();
}


pub fn middle_node_counting(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut length = 0usize;
    let mut tail = head.as_ref();
    while tail.is_some() {
        tail = tail.map(|n| n.next.as_ref()).flatten();
        length += 1;
    }
    let mut middle_index: usize = length / 2;
    while middle_index > 0 {
        middle_index -= 1;
        head = head.unwrap().next;
    }
    head
}

#[cfg(test)]
fn collect_tail_ptrs(mut node: Option<&ListNode>) -> Vec<*const ListNode> {
let mut ptrs = Vec::new();
while let Some(current) = node {
    ptrs.push(current as *const ListNode);
    node = current.next.as_deref();
}
ptrs
}

#[cfg(test)]
fn skip_nodes(mut node: Option<&ListNode>, mut skip: usize) -> Option<&ListNode> {
while skip > 0 {
    node = node.and_then(|current| current.next.as_deref());
    skip -= 1;
}
node
}

#[test]
fn odd() {
    let mut head = Box::new(ListNode::new(0));
    head = Box::new(ListNode{ val: 1, next: Some(head) });
    head = Box::new(ListNode{ val: 2, next: Some(head) });

    let expected_tail_ptrs = collect_tail_ptrs(skip_nodes(Some(head.as_ref()), 1));

    let middle = middle_node_counting(Some(head)).unwrap();
    let actual_tail_ptrs = collect_tail_ptrs(Some(middle.as_ref()));

    assert_eq!(1, middle.val);
    assert_eq!(expected_tail_ptrs, actual_tail_ptrs);
}

#[test]
fn even() {
    let mut head = Box::new(ListNode{ val: 0, next: None });
    head = Box::new(ListNode{ val: 1, next: Some(head) });
    head = Box::new(ListNode{ val: 2, next: Some(head) });
    head = Box::new(ListNode{ val: 3, next: Some(head) });
    head = Box::new(ListNode{ val: 4, next: Some(head) });
    head = Box::new(ListNode{ val: 5, next: Some(head) });

    let expected_tail_ptrs = collect_tail_ptrs(skip_nodes(Some(head.as_ref()), 3));

    let middle = middle_node_counting(Some(head)).unwrap();
    let actual_tail_ptrs = collect_tail_ptrs(Some(middle.as_ref()));

    assert_eq!(2, middle.val);
    assert_eq!(expected_tail_ptrs, actual_tail_ptrs);
}