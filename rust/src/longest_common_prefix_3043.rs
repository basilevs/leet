use crate::array_trie::ArrayTrie;

pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let mut prefixes1: ArrayTrie<10, ()> = ArrayTrie::with_capacity(arr1.len() * 8);
    
    for i in arr1 {
        prefixes1.insert_prefixes(digits(i), || ());
    }

    arr2
        .iter()
        .copied()
        .map(|p| prefixes1.walk(digits(p)).count())
        .max()
        .unwrap_or(0) as i32
}

const LEN: usize = (i32::MAX.ilog10() + 1) as usize;

const DENOMINATORS: [i32; LEN] = generate_denominators();

const fn generate_denominators() -> [i32; LEN] {
    let mut arr = [0; LEN];
    let mut i = 0;
    // The absolute closest we can get to .from_fn() in a const context
    while i < LEN {
        arr[i] = 10i32.pow((LEN - 1 - i) as u32);
        i += 1;
    }
    arr
}

fn digits(input: i32) -> impl Iterator<Item=u8> {
    let start_idx = if input <= 0 {
        DENOMINATORS.len()
    } else {
        let num_digits = input.ilog10() as usize + 1;
        DENOMINATORS.len().saturating_sub(num_digits)
    };

    DENOMINATORS[start_idx..].iter().scan(input, |i, &d| {
        let digit = *i / d;
        debug_assert!(digit < 10, "Bad digit: {} in {}", &digit, i);
        *i -= digit * d;
        debug_assert!(digit >= 0);
        Some(u8::try_from(digit).expect("Should be in 1..=9 by construction"))
    }).skip_while(|&i| i ==0)
}

#[cfg(test)]
use itertools::Itertools;


#[test]
fn test_digits() {
    assert_eq!(vec![1,2,3,4], digits(1234).collect_vec());
    assert_eq!(Vec::<u8>::new(), digits(0).collect_vec());
    assert_eq!(vec![1], digits(1).collect_vec());
    assert_eq!(vec![1,0,0], digits(100).collect_vec());
    assert_eq!(vec![1,0], digits(10).collect_vec());
    assert_eq!(vec![1,7], digits(17).collect_vec());
    assert_eq!(vec![1,1], digits(11).collect_vec());
    assert_eq!(vec![1,0,0,0,0,0,0,0,0], digits(100000000).collect_vec());
}

#[test]
fn official1() {
    assert_eq!(3, longest_common_prefix(vec![1,10,100], vec![1000]));
}

#[test]
fn official2() {
    assert_eq!(0, longest_common_prefix(vec![1,2,3], vec![4,4,4]));
}

#[test]
fn official5() {
    assert_eq!(1, longest_common_prefix(vec![10], vec![17, 11]));
}

#[test]
fn official6() {
    assert_eq!(0, longest_common_prefix(vec![8], vec![26]));
}

#[test]
fn official556() {
    assert_eq!(5, longest_common_prefix(vec![100000000], vec!
        [1,10,100,1000,10000,1,10,100,1000,10000,1,10,100,1000,10000,1,10,100,1000,10000,1,10,100,1000,10000,1,10,100,1000,10000,1,10,100,1000,10000,1,10,100,1000,10000,1,10,100,1000,10000,1,10,100,1000,10000]
    ));
}

