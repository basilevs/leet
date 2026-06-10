use std::ops::{Index, Range};

struct SparseTable<T, F>
where
    F: Fn(T, T) -> T,
{
    data: Vec<Vec<T>>,
    merge: F,
}

impl<T, F> SparseTable<T, F>
where
    F: Fn(T, T) -> T,
{
    pub fn new(input: Vec<T>, merge: F) -> Self {
        todo!()
    }

}

impl<T, F> Index<Range<usize>> for SparseTable<T, F>
where
    F: Fn(T, T) -> T,
{
    type Output = T;

    fn index(&self, index: Range<usize>) -> &Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::sparse_table::SparseTable;

    #[test]
    fn min() {
        let subject = SparseTable::new(vec![3, 2, 1, 4, 5], i32::min);
        assert_eq!(5, subject[4..5]);
        assert_eq!(3, subject[0..1]);
        assert_eq!(1, subject[0..5]);
        assert_eq!(2, subject[0..2]);
    }
    
}
