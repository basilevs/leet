
use itertools::chain;

use crate::array_trie::ArrayTrie;


#[must_use]
pub fn string_indices(mut words_container: Vec<String>, mut words_query: Vec<String>) -> Vec<i32> {
    assert!(words_container.len() < usize::try_from(i32::MAX).expect("words_container length constraint"));
    assert!(!words_container.is_empty());
    for i in chain(words_container.iter_mut(), words_query.iter_mut()) {
        reverse_in_place(i);
    }
    let mut sorted:Vec<usize> = (0..words_container.len()).collect();
    sorted.sort_by_key(|&i| words_container[i].len());
    let shortest = sorted[0];
    let mut trie: ArrayTrie<26, usize> = ArrayTrie::with_capacity(words_container.len());
    for i in sorted.into_iter().rev() {
        trie.insert_prefixes(words_container[i].chars().map(|c| c as u8 - b'a'), || i);
    }
    words_query.into_iter().map(|query| {
        trie.walk(query.chars().map(|c| c as u8 - b'a') ).last().unwrap_or(&shortest)
    }).map(|&i| i32::try_from(i).expect("words_container length constraint")) .collect()
}

fn reverse_in_place(input: &mut String) {
    unsafe {
        let bytes = input.as_bytes_mut();
        let n = bytes.len();
        for i in 0..bytes.len()/2 {
            bytes.swap(i, n - 1 - i);
        }
    }
}

#[cfg(test)]
fn to_vector<const N: usize>(input: [&str; N]) -> Vec<String> {
    input.into_iter().map(String::from).collect()
}
#[test]
fn official1() {
    assert_eq!(vec![1,1,1], string_indices(to_vector(["abcd","bcd","xbcd"]), to_vector(["cd","bcd","xyz"])));
}
#[test]
fn official2() {
    assert_eq!(vec![2,0,2], string_indices(to_vector(["abcdefgh","poiuygh","ghghgh"]), to_vector(["gh","acbfgh","acbfegh"])));
}