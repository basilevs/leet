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
        // Nodes are collected from both ends of the range towards the middle, so
        // the two frontiers need separate accumulators to keep `combine`
        // arguments in range order: everything taken on the left goes before the
        // accumulated prefix, everything on the right goes after the suffix.
        let mut prefix: Option<T> = None;
        let mut suffix: Option<T> = None;
        while l <= r {
            if !l.is_multiple_of(2) {
                prefix = Some(match prefix {
                    None => self.tree[l].clone(),
                    Some(acc) => (self.combine)(&acc, &self.tree[l]),
                });
                l += 1;
            }
            if r.is_multiple_of(2) {
                suffix = Some(match suffix {
                    None => self.tree[r].clone(),
                    Some(acc) => (self.combine)(&self.tree[r], &acc),
                });
                r -= 1;
            }
            l /= 2;
            r /= 2;
        }
        match (prefix, suffix) {
            (Some(p), Some(s)) => (self.combine)(&p, &s),
            (Some(p), None) => p,
            (None, Some(s)) => s,
            (None, None) => panic!("empty range"),
        }
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
        assert_eq!(12, seg_tree.query(2..=4));
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
        assert_eq!(12, seg_tree.query(2..=4));
    }

    fn concat_tree(elements: &[&str]) -> SegmentTree<String, impl Fn(&String, &String) -> String> {
        let input: Vec<String> = elements.iter().map(|s| s.to_string()).collect();
        SegmentTree::from(
            input.into_iter(),
            |a: &String, b: &String| format!("{a}{b}"),
            String::new(),
        )
    }

    /// Concatenation is associative but not commutative, so it detects partial
    /// results being combined out of range order — something `+` cannot catch.
    #[test]
    fn non_commutative_combine() {
        let seg_tree = concat_tree(&["a", "b", "c", "d"]);
        assert_eq!("abcd", seg_tree.query(0..=3));
        assert_eq!("abc", seg_tree.query(0..=2));
        assert_eq!("bcd", seg_tree.query(1..=3));
        assert_eq!("bc", seg_tree.query(1..=2));
    }

    /// An odd length leaves the bottom row of the tree ragged, so ranges are
    /// assembled from nodes at differing depths.
    #[test]
    fn non_commutative_combine_odd_element_count() {
        let seg_tree = concat_tree(&["a", "b", "c", "d", "e"]);
        assert_eq!("abcde", seg_tree.query(0..=4));
        assert_eq!("abcde", seg_tree.query(..));
        assert_eq!("a", seg_tree.query(0..=0));
        assert_eq!("e", seg_tree.query(4..=4));
        assert_eq!("ab", seg_tree.query(0..=1));
        assert_eq!("de", seg_tree.query(3..=4));
        assert_eq!("bcd", seg_tree.query(1..=3));
        assert_eq!("cde", seg_tree.query(2..=4));
        assert_eq!("abcd", seg_tree.query(0..4));
    }
}
