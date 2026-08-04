// https://leetcode.com/problems/create-binary-tree-from-descriptions/

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    struct Uplink {
        parent: Option<i32>,
        child: Rc<RefCell<TreeNode>>,
    }
    impl Uplink {
        fn new(value: i32) -> Uplink {
            Uplink { parent: None, child: Rc::new(RefCell::new(TreeNode::new(value))) }
        }
    }
    let mut map: HashMap<i32, Uplink> = HashMap::with_capacity(descriptions.len() + 1);
    for desc in descriptions {
        let parent = desc[0];
        let child = desc[1];
        let is_left = desc[2] != 0;
        let child_uplink = map.entry(child).or_insert_with(|| Uplink::new(child));
        let old_parent = child_uplink.parent.replace(parent);
        debug_assert_eq!(None, old_parent);
        let child_node = child_uplink.child.clone();
        let mut parent_node = map.entry(parent).or_insert_with(|| Uplink::new(parent)).child.borrow_mut();
        let child_ref = if is_left { &mut parent_node.left } else { &mut parent_node.right };
        let old = child_ref.replace(child_node);
        debug_assert_eq!(None, old);
    }
    let mut root = map.values().next().unwrap();
    loop {
        let Some(parent) = root.parent else {
            break Some(root.child.clone());
        };
        root = map.get(&parent).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    fn leaf(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        node(val, None, None)
    }

    #[test]
    fn official1() {
        // Example 1 from LeetCode 2196:
        //          50
        //         /  \
        //        20   80
        //       /  \  /
        //      15  17 19
        let descriptions = vec![
            vec![20, 15, 1],
            vec![20, 17, 0],
            vec![50, 20, 1],
            vec![50, 80, 0],
            vec![80, 19, 1],
        ];
        let expected = node(50,
            node(20, leaf(15), leaf(17)),
            node(80, leaf(19), None),
        );
        assert_eq!(expected, create_binary_tree(descriptions));
    }

    #[test]
    fn official2() {
        // Example 2 from LeetCode 2196:
        //     1
        //    /
        //   2
        //    \
        //     3
        //    /
        //   4
        let descriptions = vec![
            vec![1, 2, 1],
            vec![2, 3, 0],
            vec![3, 4, 1],
        ];
        let expected = node(1,
            node(2, None, node(3, leaf(4), None)),
            None,
        );
        assert_eq!(expected, create_binary_tree(descriptions));
    }

    #[test]
    fn single_edge_left() {
        let descriptions = vec![vec![1, 2, 1]];
        let expected = node(1, leaf(2), None);
        assert_eq!(expected, create_binary_tree(descriptions));
    }

    #[test]
    fn root_described_first() {
        // Root (50) appears as a parent before its own parent edge would, but
        // here 50 is the root because no edge lists it as a child.
        let descriptions = vec![
            vec![50, 20, 1],
            vec![50, 80, 0],
        ];
        let expected = node(50, leaf(20), leaf(80));
        assert_eq!(expected, create_binary_tree(descriptions));
    }
}
