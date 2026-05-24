use std::collections::{BTreeMap};

pub struct SummaryRanges {
    // key=start, value=end, inclusive
    intervals: BTreeMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {

    pub fn new() -> Self {
        Self {intervals: BTreeMap::new()}
    }
    
    pub fn add_num(&mut self, value: i32) {
        let start = if let Some((&k, &v)) = self.intervals.range(..value).next_back() {
            debug_assert!(k <= v);
            debug_assert!(k < value);
            if value <= v {
                return;
            } else if value == v + 1 {
                self.intervals.remove(&k);
                k
            } else {
                value
            }
        } else {
            value
        };
        let end = if let Some((&k, &v)) = self.intervals.range(value..).next() {
            debug_assert!(k <= v);
            debug_assert!(k >= value);
            if k == value {
                v
            } else if value + 1 == k {
                self.intervals.remove(&k);
                v
            } else {
                value
            }
        } else {
            value
        };

        self.intervals.insert(start, end);

    }
    
    pub fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.intervals.iter().map(|(&k, &v)| vec![k,v]).collect()
    }
}

/**
 * Your SummaryRanges object will be instantiated and called as such:
 * let obj = SummaryRanges::new();
 * obj.add_num(value);
 * let ret_2: Vec<Vec<i32>> = obj.get_intervals();
 */

#[cfg(test)]
fn to_vector(input: &[[i32; 2]]) -> Vec<Vec<i32>> {
    input.iter().map(Vec::from).collect()
}

#[test]
fn official1() {
    // SummaryRanges summaryRanges = new SummaryRanges();
    let mut summary_ranges = SummaryRanges::new();
    // summaryRanges.addNum(1);      // arr = [1]
    summary_ranges.add_num(1);
    // summaryRanges.getIntervals(); // return [[1, 1]]
    assert_eq!(to_vector(&[[1, 1]]), summary_ranges.get_intervals());
    // summaryRanges.addNum(3);      // arr = [1, 3]
    summary_ranges.add_num(3);
    // summaryRanges.getIntervals(); // return [[1, 1], [3, 3]]
    assert_eq!(to_vector(&[[1, 1], [3, 3]]), summary_ranges.get_intervals());
    // summaryRanges.addNum(7);      // arr = [1, 3, 7]
    summary_ranges.add_num(7);
    // summaryRanges.getIntervals(); // return [[1, 1], [3, 3], [7, 7]]
    assert_eq!(to_vector(&[[1, 1], [3, 3], [7, 7]]), summary_ranges.get_intervals());
    // summaryRanges.addNum(2);      // arr = [1, 2, 3, 7]
    summary_ranges.add_num(2);
    // summaryRanges.getIntervals(); // return [[1, 3], [7, 7]]
    assert_eq!(to_vector(&[[1, 3], [7, 7]]), summary_ranges.get_intervals());
    // summaryRanges.addNum(6);      // arr = [1, 2, 3, 6, 7]
    summary_ranges.add_num(6);
    // summaryRanges.getIntervals(); // return [[1, 3], [6, 7]]
    assert_eq!(to_vector(&[[1, 3], [6, 7]]), summary_ranges.get_intervals());
    
}