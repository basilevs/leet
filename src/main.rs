fn main() {
    let a = (u64::MAX as f64).sqrt();
    println!("Hello World! {} {}", a, a*a - u64::MAX as f64);
}
