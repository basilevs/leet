
#[must_use]
pub fn number_of_special_chars(word: String) -> i32 {
    enum State { None, Lower, Both, Fail }
    use State::*;
    let mut states = [const { None }; 26];

    for b in word.into_bytes() {
        if b.is_ascii_lowercase() {
            let i = (b - b'a') as usize;
            states[i] = match states[i] {
                None | Lower => Lower,
                Both | Fail => Fail,
            };
        } else if b.is_ascii_uppercase() {
            let i = (b - b'A') as usize;
            states[i] = match states[i] {
                Lower | Both => Both,
                None | Fail => Fail,
            };
        }
    }

    i32::try_from(states.iter().filter(|x| matches!(x, Both)).count()).expect("count fits i32")
}

#[test]
fn official1() {
    assert_eq!(3, number_of_special_chars("aaAbcBC".to_string()));
}

#[test]
fn official2() {
    assert_eq!(0, number_of_special_chars("abc".to_string()));
}
#[test]
fn official3() {
    assert_eq!(0, number_of_special_chars("AbBCab".to_string()));
}