use std::collections::BTreeMap;

struct StockSpanner {
    last_positions: BTreeMap<i32, usize>,
    position: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    fn new() -> Self {
        Self {last_positions: BTreeMap::new(), position: 0}
    }
    
    fn next(&mut self, price: i32) -> i32 {
        let range = self.last_positions.range((price+1)..);
        // dbg!(&self.last_positions, price);
        let last_occurence = range.map(|p| p.1 + 1).max().unwrap_or(0);
        let result = self.position + 1 - last_occurence;
        self.last_positions.insert(price, self.position);
        self.position += 1;
        result as i32
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */

 #[test]
 pub fn official1() {
    let mut s = StockSpanner::new();
    assert_eq!(1, s.next(100)); // return 1
    assert_eq!(1, s.next(80));  // return 1
    assert_eq!(1, s.next(60));  // return 1
    assert_eq!(2, s.next(70));  // return 2
    assert_eq!(1, s.next(60));  // return 1
    assert_eq!(4, s.next(75));  // return 4, because the last 4 prices (including today's price of 75) were less than or equal to today's price.
    assert_eq!(6, s.next(85));  // return 6
 }

  #[test]
 pub fn official23() {
    let mut s = StockSpanner::new();
    //[[],[29],[91],[62],[76],[51]]
    //[null,1,2,1,2,1]
    assert_eq!(1, s.next(29)); // return 1
    assert_eq!(2, s.next(91));  // return 1
    assert_eq!(1, s.next(62));  // return 1
    assert_eq!(2, s.next(76));  // return 2
    assert_eq!(1, s.next(51));  // return 1
 }