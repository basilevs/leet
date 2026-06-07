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
            Uplink{ parent: None, child: Rc::new(RefCell::new(TreeNode::new(value))) }
        }
    }
    let mut map: HashMap<i32, Uplink> = HashMap::with_capacity(descriptions.len());
    for i in descriptions {
        let parent = i[0];
        let child = i[1];
        let is_left = i[2] != 0;
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
        root = map.get_mut(&parent).unwrap();
    }
}
