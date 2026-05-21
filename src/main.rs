fn main() {
    let a = u32::MAX;
    let n = 12345;
            let arr1_prefixes: HashSet<i32> = arr1
            .iter()
            .flat_map(|&x| successors(Some(x), |&n| (n >= 10).then_some(n / 10)))
            .collect();
    println!("Hello World! {} ", n);
}
