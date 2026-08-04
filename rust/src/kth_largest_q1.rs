use std::collections::BinaryHeap;

pub struct KthLargest {
    k: usize,
    heap: BinaryHeap<i32>,
}
    
impl KthLargest {

    pub fn new(k: i32, mut nums: Vec<i32>) -> Self {
        assert!(k > 0, "k should be positive");
        for i in nums.iter_mut() {
            *i = -*i;
        }
        let k = usize::try_from(k).expect("k should be positive");
        Self { k, heap: BinaryHeap::from(nums) }
    }
    
    pub fn add(&mut self, val: i32) -> i32 {
        self.heap.push(-val);
        while self.heap.len() > self.k {
            self.heap.pop();
        }
        -*self.heap.peek().unwrap()
    }
}



// Input:
// ["KthLargest", "add", "add", "add", "add", "add"]
// [[3, [4, 5, 8, 2]], [3], [5], [10], [9], [4]]
// Output: [null, 4, 5, 5, 8, 8]
// Explanation:

// KthLargest kthLargest = new KthLargest(3, [4, 5, 8, 2]);
// kthLargest.add(3); // return 4
// kthLargest.add(5); // return 5
// kthLargest.add(10); // return 5
// kthLargest.add(9); // return 8
// kthLargest.add(4); // return 8
#[test]
fn official1() {
    let mut s = KthLargest::new(3, vec![4, 5, 8, 2]);
    assert_eq!(4, s.add(3));
    assert_eq!(5, s.add(5));
    assert_eq!(5, s.add(10));
    assert_eq!(8, s.add(9));
    assert_eq!(8, s.add(4));
}

// Example 2:

// Input:
// ["KthLargest", "add", "add", "add", "add"]
// [[4, [7, 7, 7, 7, 8, 3]], [2], [10], [9], [9]]

// Output: [null, 7, 7, 7, 8]

// Explanation:
// KthLargest kthLargest = new KthLargest(4, [7, 7, 7, 7, 8, 3]);
// kthLargest.add(2); // return 7
// kthLargest.add(10); // return 7
// kthLargest.add(9); // return 7
// kthLargest.add(9); // return 8

#[test]
fn official2() {
    let mut s = KthLargest::new(4, vec![7, 7, 7, 7, 8, 3]);
    assert_eq!(7, s.add(2));
    assert_eq!(7, s.add(10));
    assert_eq!(7, s.add(9));
    assert_eq!(8, s.add(9));
}