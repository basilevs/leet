use std::collections::HashMap;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let ransom_note_letters = count_letters(&ransom_note);
    let magazine_letters = count_letters(&magazine);

    ransom_note_letters.iter().all(|(ch, needed)| {
        magazine_letters
            .get(ch)
            .is_some_and(|available| available >= needed)
    })
}

fn count_letters(text: &str) -> HashMap<char, usize> {
    let mut result = HashMap::with_capacity(text.len());
    for ch in text.chars() {
        *result.entry(ch).or_default() += 1;
    }
    result
}

#[test]
fn count_test() {
    let result = count_letters("aadt");
    assert_eq!(Some(2), result.get(&'a').copied());
    assert_eq!(Some(1), result.get(&'d').copied());
    assert_eq!(None, result.get(&'c').copied());
}

#[test]
fn positive() {
    assert!(can_construct(String::from("data"), String::from("aadt")));
}

#[test]
fn not_enough() {
    assert!(!can_construct(String::from("data"), String::from("adt")));
}

#[test]
fn none() {
    assert!(!can_construct(String::from("data"), String::from("aad")));
}
