// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
}
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // Borrow checker makes multiple cursor approach very hard (unless cloning)
        // Traverse multiple times separately
        let mut length = 0;
        {
            let mut cursor = head.as_ref();
            while cursor.is_some() {
                cursor = cursor.and_then(|n| n.next.as_ref());
                length += 1;
            }
        }
        
        if length < 2 || k % length == 0{
            return head;
        }
        let mut suffix_length = length - k % length;
        let mut cursor_mut = head.as_mut();
        while cursor_mut.is_some() && suffix_length > 1 {
            suffix_length -= 1;
            cursor_mut = cursor_mut.and_then(|n| n.next.as_mut());
        }
        let mut prefix = cursor_mut.and_then(|n| n.next.take());
        assert!(prefix.is_some());
        cursor_mut = prefix.as_mut();
        while cursor_mut.as_ref().and_then(|n| n.next.as_ref()).is_some() {
            cursor_mut = cursor_mut.and_then(|n| n.next.as_mut());
        }
        assert!(cursor_mut.as_ref().unwrap().next.is_none());
        cursor_mut.unwrap().next = head;
        prefix

    }


#[cfg(test)]
fn create_list(data: &[i32]) -> Option<Box<ListNode>> {
    if data.is_empty() {
        None
    } else {
        Some(Box::new(ListNode{next: create_list(&data[1..]), val: data[0]}))
    }
}

#[test]
fn official1() {
    assert_eq!(create_list(&[4,5,1,2,3]), rotate_right(create_list(&[1,2,3,4,5]), 2));
}

#[test]
fn official2() {
    assert_eq!(create_list(&[2,0,1]), rotate_right(create_list(&[0,1,2]), 4));
}

#[test]
fn t1() {
    assert_eq!(create_list(&[0,1,2]), rotate_right(create_list(&[0,1,2]), 0));
}

#[test]
fn error2() {
    assert_eq!(create_list(&[]), rotate_right(create_list(&[]), 0));
}

#[test]
fn error207() {
    assert_eq!(create_list(&[1]), rotate_right(create_list(&[1]), 0));
}