use std::{collections::HashSet, iter::successors};

fn main() {
    let n = 12345;
    let arr1_prefixes: HashSet<i32> = successors(Some(n), |&n| (n >= 10).then_some(n / 10))
    .collect();
    println!("Hello World! {:?} ", &arr1_prefixes);
}
