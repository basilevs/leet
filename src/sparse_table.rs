use std::ops::Range;

// https://cp-algorithms.com/data_structures/sparse-table.html
pub struct SparseTable<T, F>
where
    F: Fn(T, T) -> T,
{
    data: Vec<Vec<T>>,
    merge: F,
}

impl<T, F> SparseTable<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    pub fn new(input: Vec<T>, merge: F) -> Self {
        let n = input.len();
        let rows = n.checked_ilog2().unwrap_or(0) as usize + 1;
        let mut data = Vec::with_capacity(rows);
        data.push(input);
        let mut i = 1;
        while (1 << i) <= n {
            let prev = &data[i - 1];
            let row: Vec<T> = (0..=n - (1 << i))
                .map(|j| merge(prev[j], prev[j + (1 << (i - 1))]))
                .collect();
            data.push(row);
            i += 1;
        }
        Self { data, merge }
    }

    // Idempotent range query over the half-open range `[start, end)`.
    pub fn query(&self, range: Range<usize>) -> T {
        let Range { start, end } = range;
        let i = (end - start).ilog2() as usize;
        (self.merge)(self.data[i][start], self.data[i][end - (1 << i)])
    }
}

#[cfg(test)]
mod tests {
    use crate::sparse_table::SparseTable;

    #[test]
    fn min() {
        let subject = SparseTable::new(vec![3, 2, 1, 4, 5], i32::min);
        assert_eq!(5, subject.query(4..5));
        assert_eq!(3, subject.query(0..1));
        assert_eq!(1, subject.query(0..5));
        assert_eq!(2, subject.query(0..2));
    }
    
}
