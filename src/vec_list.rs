use std::{fmt::Debug, mem::replace};

struct VecListNode<T> {
    pub data: T,
    next: Option<usize>,
    prev: Option<usize>,
}

impl <T: Debug> Debug for VecListNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VecListNode").field("next", &self.next).field("prev", &self.prev).field("data", &self.data).finish()
    }
}

pub struct VecList<T> {
    buffer: Vec<VecListNode<T>>,
    head: Option<usize>,
    tail: Option<usize>,
}

impl<T: Debug> Debug for VecList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_list();
        for i in self.iter() {
            f.entry(&i);
        }
        f.finish()
    }
}

pub struct ListIter<'a, T> {
    data: &'a VecList<T>,
    index: Option<usize>,
}

impl<'a, T: Debug> Iterator for ListIter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index?;
        let node = &self.data.buffer[index];
        self.index = node.next;
        Some((index, &node.data))
    }
}

impl<T: Debug> VecList<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        VecList {
            buffer: Vec::with_capacity(capacity),
            head: None,
            tail: None,
        }
    }

    pub fn head(&self) -> Option<usize> {
        self.head
    }

    pub fn tail(&self) -> Option<usize> {
        self.tail
    }

    pub fn iter(&self) -> ListIter<'_, T> {
        ListIter { data: self, index: self.head }
    }

    pub fn get(&self, n: usize) -> &T {
        &self.buffer[n].data
    }

    pub fn get_mut(&mut self, n: usize) -> &mut T {
        &mut self.buffer[n].data
    }

    pub fn previous(&self, n: usize) -> Option<usize> {
        self.buffer[n].prev
    }

    pub fn move_before(&mut self, n: usize, before: usize) {
        self.assert_valid_node(n);
        if n == before {
            return;
        }
        self.cut(n);
        self.paste_before(n, before);
        self.update_ends(n);
        self.assert_valid_node(n);
        self.assert_valid_node(before);
    }

    /// Move node `n` to the head of the list.
    pub fn move_to_head(&mut self, n: usize) {
        let next = self.buffer[n].next;
        let prev = self.buffer[n].prev;
        self.assert_valid_node(n);

        if let Some(prev) = prev {
            self.assert_valid_node(prev);
            self.buffer[prev].next = next;
            if let Some(next_node) = next {
                self.buffer[next_node].prev = Some(prev);
            }
            self.buffer[n].prev = None;
            self.buffer[n].next = self.head;
            self.buffer[self.head.unwrap()].prev = Some(n);
            self.update_ends(n);
            self.update_ends(prev);
            self.assert_valid_node(n);
            self.assert_valid_node(prev);
        } else {
            debug_assert_eq!(Some(n), self.head);
        }
    }

    fn cut(&mut self, n: usize) {
        let node = self.buffer.get_mut(n).expect("Node should exist");
        let prev = node.prev.take();
        let next = node.next.take();
        if let Some(next) = next {
            self.buffer[next].prev = prev;
            self.update_ends(next);
        }
        if let Some(prev) = prev {
            self.buffer[prev].next = next;
            self.update_ends(prev);
        }
    }

    fn paste_before(&mut self, n: usize, before: usize) {
        assert_ne!(n, before);
        // dbg!(&self.buffer);
        let prev =  replace(&mut self.buffer[before].prev, Some(n));
        if let Some(prev) = prev {
            self.buffer[prev].next = Some(n);
        }
        self.buffer[n].next = Some(before);
        self.buffer[n].prev = prev;
        // dbg!(&self.buffer);
    }

    /// Push a new node at the head and return its index.
    /// If at capacity, also return evicted tail
    pub fn push_head_evicting(&mut self, data: T) -> (usize, Option<T>) {
        if self.buffer.capacity() > self.buffer.len() {
            self.buffer.push(VecListNode {
                data,
                next: self.head,
                prev: None,
            });
            let result = self.buffer.len() - 1;
            debug_assert_eq!(self.head.is_none(), self.tail.is_none());
            if let Some(head) = self.head {
                self.buffer[head].prev = Some(result);
            }
            self.update_ends(result);
            (result, None)
        } else {
            let result = self.tail.unwrap();
            let old = std::mem::replace(&mut self.buffer[result].data, data);
            self.move_to_head(result);
            (result, Some(old))
        }
    }

    /// Push a new node at the tail and return its index.
    /// If at capacity, also return evicted tail value
    pub fn push_tail_evicting(&mut self, data: T) -> (usize, Option<T>) {
        if self.buffer.capacity() > self.buffer.len() {
            self.buffer.push(VecListNode {
                data,
                next: None,
                prev: self.tail,
            });
            let result = self.buffer.len() - 1;
            debug_assert_eq!(self.head.is_none(), self.tail.is_none());
            if let Some(tail) = self.tail {
                self.buffer[tail].next = Some(result);
            }
            self.update_ends(result);
            (result, None)
        } else {
            let result = self.tail.unwrap();
            let old = std::mem::replace(&mut self.buffer[result].data, data);
            (result, Some(old))
        }
    }

    #[cfg(debug_assertions)]
    pub fn assert_valid_node(&self, n: usize) {
        let node = &self.buffer[n];
        assert_ne!(Some(n), node.next);
        assert_ne!(Some(n), node.prev);
        if let Some(next) = node.next {
            assert_ne!(node.next, node.prev, "index={}, {:?}", n, node);
            assert_eq!(Some(n), self.buffer[next].prev, "next.prev does not match");
        }
        if let Some(prev) = node.prev {
            assert_eq!(Some(n), self.buffer[prev].next, "prev.next does not match");
        }
        if let Some(next) = self.buffer[n].next {
            assert_eq!(Some(n), self.buffer[next].prev, "index={}, {:?}", n, &self);
        }
        assert_eq!(self.head == Some(n), self.buffer[n].prev.is_none());
        assert_eq!(self.tail == Some(n), self.buffer[n].next.is_none());
    }

    #[cfg(not(debug_assertions))]
    pub fn assert_valid_node(&self, _n: usize) {}

    fn update_ends(&mut self, n: usize) {
        let node = self.buffer.get(n).expect("Known key");
        if node.prev.is_none() {
            self.head = Some(n);
        }
        if node.next.is_none() {
            self.tail = Some(n);
        }

    }
}

#[test]
fn do_not_swap_unrelated_entries() {
    let mut subject = VecList::with_capacity(4);
    let (n1, _) = subject.push_tail_evicting(1);
    subject.push_tail_evicting(2);
    subject.push_tail_evicting(3);
    let (n4, _) = subject.push_tail_evicting(4);
    assert_eq!(vec![1,2,3,4], subject.iter().map(|x| *x.1).collect::<Vec<_>>());
    subject.move_before(n4, n1);
    assert_eq!(vec![4,1,2,3], subject.iter().map(|x| *x.1).collect::<Vec<_>>());
}

#[test]
fn move_to_head() {
    let mut subject = VecList::with_capacity(4);
    subject.push_tail_evicting(1);
    let (n2, _) = subject.push_tail_evicting(2);
    subject.push_tail_evicting(3);
    let (n4, _) = subject.push_tail_evicting(4);
    assert_eq!(vec![1,2,3,4], subject.iter().map(|x| *x.1).collect::<Vec<_>>());
    subject.move_to_head(n4);
    assert_eq!(vec![4,1,2,3], subject.iter().map(|x| *x.1).collect::<Vec<_>>());
    subject.move_to_head(n2);
    assert_eq!(vec![2,4,1,3], subject.iter().map(|x| *x.1).collect::<Vec<_>>());
}