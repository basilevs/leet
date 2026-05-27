
pub fn number_of_special_chars(word: String) -> i32 {
    enum State {
        NONE,
        LOWER,
        BOTH,
        FAIL
    }
    use State::*;
    let mut states = [const {NONE}; 26];

    for c in word.chars() {
        if c.is_ascii_lowercase() {
            let i = c as usize - 'a' as usize;
            states[i] = match states[i] {
                NONE => LOWER,
                LOWER => LOWER,
                BOTH => FAIL,
                FAIL => FAIL,
            };
        } else if c.is_ascii_uppercase() {
            let i = c as usize - 'A' as usize;
            states[i] = match states[i] {
                NONE => FAIL,
                LOWER => BOTH,
                BOTH => BOTH,
                FAIL => FAIL,
            };            
        }
    }

    states.iter().filter(|&x| matches!(x, State::BOTH)).count() as i32
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