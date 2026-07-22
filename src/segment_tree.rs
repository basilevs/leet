use std::{fmt::Debug, ops::RangeBounds};

pub struct SegmentTree<T, F> 
    where F: Fn(&T, &T) -> T
{
    tree: Vec<T>,
    combine: F,
}


impl<T, F> SegmentTree<T, F> 
    where
        F: Fn(&T, &T) -> T,
        T: Clone + Debug,
{
    pub fn from(input: impl ExactSizeIterator<Item = T>, combine: F, default: T) -> Self 
    {
        let n = input.len();
        // element 0 is unused
        let mut tree = Vec::with_capacity(2*n);
        tree.resize_with(n, || default.clone());
        tree.extend(input);
        for i in (1..n).rev() {
            tree[i] = combine(&tree[2 * i], &tree[2 * i + 1]);
        }
        // dbg!(&tree);
        Self { tree, combine }
    }

    pub fn query(&self, range: impl RangeBounds<usize>) -> T {
        use std::ops::Bound;
        let n = self.tree.len() / 2;
        let mut l = match range.start_bound() {
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
            Bound::Unbounded => 0,
        } + n;
        let mut r = match range.end_bound() {
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x - 1,
            Bound::Unbounded => n - 1,
        } + n;
        if l == r {
            return self.tree[r].clone();
        }
        let mut res: Option<T> = None;
        while l <= r {
            if !l.is_multiple_of(2) {
                res = Some(match res {
                    None => self.tree[l].clone(),
                    Some(acc) => (self.combine)(&self.tree[l], &acc),
                });
                l += 1;
            }
            if r.is_multiple_of(2) {
                res = Some(match res {
                    None => self.tree[r].clone(),
                    Some(acc) => (self.combine)(&acc, &self.tree[r]),
                });
                r -= 1;
            }
            l /= 2;
            r /= 2;
        }
        res.expect("empty range").clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_element_count() {
        let input = vec![1, 2, 3, 4, 5];
        let combine = |a: &i32, b: &i32| a + b;
        let seg_tree = SegmentTree::from(input.into_iter(), combine, 0);
        assert_eq!(15, seg_tree.query(0..=4));
        assert_eq!(15, seg_tree.query(0..5));
        assert_eq!(1, seg_tree.query(0..=0));
        assert_eq!(5, seg_tree.query(4..=4));
        assert_eq!(9, seg_tree.query(1..=3));
    }

    #[test]
    fn even_element_count() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let combine = |a: &i32, b: &i32| a + b;
        let seg_tree = SegmentTree::from(input.into_iter(), combine, 0);
        assert_eq!(21, seg_tree.query(0..=5));
        assert_eq!(21, seg_tree.query(0..6));
        assert_eq!(1, seg_tree.query(0..=0));
        assert_eq!(6, seg_tree.query(5..=5));
        assert_eq!(9, seg_tree.query(1..=3));
    }

}
