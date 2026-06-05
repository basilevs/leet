use std::iter::from_fn;

use itertools::Itertools;

pub fn total_waviness(num1: i32, num2: i32) -> i32 {
    (num1..=num2).map(waviness).sum()
}

fn waviness(n: i32) -> i32 {
    if n < 100 {
        return 0;
    }
    i32::try_from(digits(n).tuple_windows().filter(|(left, middle, right)| 
        left > middle && right > middle || left < middle && right < middle
    ).count()).expect("Too much wavess")
}

fn digits(mut n: i32) -> impl Iterator<Item=u8> {
    from_fn(move || {
        if n > 0 {
            let result = u8::try_from(n % 10).unwrap();
            n /= 10;
            Some(result)
        } else {
            None
        }
    })
}
