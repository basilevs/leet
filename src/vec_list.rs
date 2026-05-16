#[derive(Debug)]
struct VecListNode<T> {
    pub data: T,
    next: Option<usize>,
    prev: Option<usize>,
}

#[derive(Debug)]
pub struct VecList<T> {
    buffer: Vec<VecListNode<T>>,
    head: Option<usize>,
    tail: Option<usize>,
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

    /// Move node `n` to the head of the list.
    pub fn move_to_head(&mut self, n: usize) {
        let next = self.buffer[n].next;
        let prev = self.buffer[n].prev;

        if let Some(prev) = prev {
            self.buffer[prev].next = next;
            if let Some(next_node) = next {
                self.buffer[next_node].prev = Some(prev);
            } else {
                self.tail = Some(prev);
            }
            self.buffer[n].prev = None;
            self.buffer[n].next = self.head;
            self.buffer[self.head.unwrap()].prev = Some(n);
            self.head = Some(n);
        } else {
            debug_assert_eq!(Some(n), self.head);
        }
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
                self.head = Some(result);
            } else {
                self.head = Some(result);
                self.tail = Some(result);
            }
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
        if let Some(next) = self.buffer[n].next {
            assert_eq!(Some(n), self.buffer[next].prev);
        }
        assert_eq!(self.head == Some(n), self.buffer[n].prev.is_none());
        assert_eq!(self.tail == Some(n), self.buffer[n].next.is_none());
    }

    #[cfg(not(debug_assertions))]
    pub fn assert_valid_node(&self, _n: usize) {}
}
