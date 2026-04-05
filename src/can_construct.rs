use std::collections::HashMap;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let ransom_note_letters = count_letters(&ransom_note);
    let magazine_letters = count_letters(&magazine);
    dbg!(&ransom_note_letters);
    dbg!(&magazine_letters);

    return ransom_note_letters.iter().all(|(k, v)| {
        magazine_letters.get(k).map_or(false, |m| {
            dbg!(m, v);
            m >= v
        })
    });
}

fn count_letters(text: &str) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for i in text.chars() {
        result.entry(i).and_modify(|e| *e += 1).or_insert(1usize);
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
