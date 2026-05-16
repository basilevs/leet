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
        f.debug_struct("VecList")
            .field("buffer", &self.buffer)
            .field("head", &self.head)
            .field("tail", &self.tail)
            .finish()
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

    pub fn get(&self, n: usize) -> &T {
        &self.buffer[n].data
    }

    pub fn get_mut(&mut self, n: usize) -> &mut T {
        &mut self.buffer[n].data
    }

    pub fn previous(&self, n: usize) -> Option<usize> {
        self.buffer[n].prev
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

    pub fn swap_order(&mut self, a: usize, b: usize) {
        self.assert_valid_node(a);
        self.assert_valid_node(b);
        if a == b {
            return;
        }
        dbg!(a, b, &self);
        if let Some(next) = self.buffer[a].next {
            if next == b {
                self.cut(b);
                self.paste_before(b, a);
            } else {
                self.cut(a);
                self.paste_after(a, b);
                self.cut(b);
                self.paste_before(b, next);
            }
        } else if let Some(prev) = self.buffer[a].prev {
            if prev == b {
                self.cut(b);
                self.paste_after(b, a);
            } else {
                self.cut(a);
                self.paste_after(a, b);
                self.cut(b);
                self.paste_after(b, prev);
            }
        } else {
            unreachable!();
        }
        
        self.update_ends(a);
        self.update_ends(b);
        self.assert_valid_node(a);
        self.assert_valid_node(b);
        dbg!("After swap", a, b, &self);

    }

    fn cut(&mut self, n: usize) {
        let node = self.buffer.get_mut(n).expect("Node should exist");
        let prev =  replace(&mut node.prev, None);
        let next =  replace(&mut node.next, None);
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
        let prev =  replace(&mut self.buffer[before].prev, Some(n));
        if let Some(prev) = prev {
            self.buffer[prev].next = Some(n);
        }
        self.buffer[n].next = Some(before);
        self.buffer[n].prev = prev;
    }

    fn paste_after(&mut self, n: usize, after: usize) {
        assert_ne!(n, after);
        dbg!("paste_after enter", n, after, &self);
        let next =  replace(&mut self.buffer[after].next, Some(n));
        if let Some(next) = next {
            self.buffer[next].prev = Some(n);
        }
        self.buffer[n].prev = Some(after);
        self.buffer[n].next = next;
        dbg!("paste_after exit", n, after, &self);
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
