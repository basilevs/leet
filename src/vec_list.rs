use std::fmt::Debug;

struct VecListNode<T> {
    pub data: T,
    next: Option<usize>,
    prev: Option<usize>,
}

impl <T> Debug for VecListNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("VecListNode").field(&self.next).field(&self.prev).finish()
    }
}

pub struct VecList<T> {
    buffer: Vec<VecListNode<T>>,
    head: Option<usize>,
    tail: Option<usize>,
}

impl<T> Debug for VecList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VecList")
            .field("buffer", &self.buffer)
            .field("head", &self.head)
            .field("tail", &self.tail)
            .finish()
    }
}

impl<T> VecList<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        VecList {
            buffer: Vec::with_capacity(capacity),
            head: None,
            tail: None,
        }
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
        // Yes, double reference is not stricly necessary, I'm just practicing.
        let (a_node, b_node) = if a < b {
            let (left, right) = self.buffer.split_at_mut(b);
            (&mut left[a], &mut right[0])
        } else if a > b {
            let (left, right) = self.buffer.split_at_mut(a);
            (&mut right[0], &mut left[b])
        } else {
            return;
        };

        let tmp = a_node.next;
        a_node.next = if b_node.next == Some(a) {
            Some(b)
        } else {
            b_node.next
        };
        b_node.next = if tmp == Some(b) {
            Some(a)
        } else {
            tmp
        };
        let tmp = a_node.prev;
        a_node.prev = if b_node.prev == Some(a) {
            Some(b)
        } else {
            b_node.prev
        };
        b_node.prev = if tmp == Some(b) {
            Some(a)
        } else {
            tmp
        };
        self.update_ends(a);
        self.update_ends(b);
        self.assert_valid_node(a);
        self.assert_valid_node(b);
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

    #[cfg(debug_assertions)]
    pub fn assert_valid_node(&self, n: usize) {
        let node = &self.buffer[n];
        assert_ne!(Some(n), node.next);
        assert_ne!(Some(n), node.prev);
        if let Some(next) = self.buffer[n].next {
            assert_eq!(Some(n), self.buffer[next].prev, "{:?}", &self);
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
