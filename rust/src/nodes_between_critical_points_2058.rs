// https://leetcode.com/problems/find-the-minimum-and-maximum-number-of-nodes-between-critical-points

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

#[must_use]
pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
    let Some(head) = head else {
        return vec![-1, -1];
    };

    let mut first = None;
    let mut last: Option<i32> = None;
    let mut min_distance = i32::MAX;

    // A critical point needs both neighbours, so the walk starts at the second
    // node and stops at the last one. Nodes are dropped as they are passed.
    let mut previous = head.val;
    let mut current = head.next;
    let mut index = 1;

    while let Some(node) = current {
        if let Some(next) = node.next.as_deref() {
            let local_maxima = node.val > previous && node.val > next.val;
            let local_minima = node.val < previous && node.val < next.val;
            if local_maxima || local_minima {
                if let Some(previous_critical) = last.replace(index) {
                    min_distance = min_distance.min(index - previous_critical);
                } else {
                    first = Some(index);
                }
            }
        }

        previous = node.val;
        current = node.next;
        index += 1;
    }

    match (first, last) {
        // Equal bounds mean a single critical point, which has no distinct pair.
        (Some(first), Some(last)) if first < last => vec![min_distance, last - first],
        _ => vec![-1, -1],
    }
}

#[cfg(test)]
fn list(values: &[i32]) -> Option<Box<ListNode>> {
    values.iter().rev().fold(None, |next, &val| {
        let mut node = ListNode::new(val);
        node.next = next;
        Some(Box::new(node))
    })
}

#[test]
fn official1() {
    assert_eq!(vec![-1, -1], nodes_between_critical_points(list(&[3, 1])));
}

#[test]
fn official2() {
    assert_eq!(
        vec![1, 3],
        nodes_between_critical_points(list(&[5, 3, 1, 2, 5, 1, 2]))
    );
}

#[test]
fn official3() {
    assert_eq!(
        vec![3, 3],
        nodes_between_critical_points(list(&[1, 3, 2, 2, 3, 2, 2, 2, 7]))
    );
}

#[test]
fn single_critical_point() {
    assert_eq!(
        vec![-1, -1],
        nodes_between_critical_points(list(&[2, 2, 1, 3]))
    );
}

#[test]
fn empty_list() {
    assert_eq!(vec![-1, -1], nodes_between_critical_points(None));
}
