#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn mirror_frequency(s: String) -> i32 {
    let mut letter_frequencies = [0; 13];
    let mut digit_frequencies = [0; 5];
    let letter_start = 'a' as usize;
    let digit_start = '0' as usize;
    for ch in s.chars() {
        let ch = ch as usize;
        if ch >= letter_start {
            update(&mut letter_frequencies, ch.strict_sub(letter_start));
        } else if ch >= digit_start {
            update(&mut digit_frequencies, ch.strict_sub(digit_start));
        }
    }

    letter_frequencies.into_iter().map(&i32::abs).sum::<i32>()
        + digit_frequencies.into_iter().map(&i32::abs).sum::<i32>()
}

fn update(frequencies: &mut [i32], index: usize) {
    if index < frequencies.len() {
        frequencies[index] += 1;
    } else if index >= frequencies.len() && index < frequencies.len() * 2 {
        // 9 -> 0
        // 8 -> 1
        frequencies[frequencies.len() * 2 - 1 - index] -= 1;
    }
}

#[cfg(test)]
fn mirror_frequency_ref(s: &str) -> i32 {
    mirror_frequency(String::from(s))
}

#[test]
fn tests() {
    assert_eq!(3, mirror_frequency_ref("ab1z9"));
    assert_eq!(2, mirror_frequency_ref("4m7n"));
    assert_eq!(0, mirror_frequency_ref("byby"));
    assert_eq!(1, mirror_frequency_ref("z"));
    assert_eq!(0, mirror_frequency_ref("az"));
    assert_eq!(0, mirror_frequency_ref("09"));
    assert_eq!(1, mirror_frequency_ref("a"));
    assert_eq!(0, mirror_frequency_ref(":"));
    assert_eq!(2, mirror_frequency_ref("ab"));
    assert_eq!(2, mirror_frequency_ref("aa"));
}

#[test]
fn ascii() {
    assert_eq!(97, "az".chars().nth(0).unwrap() as usize);
    assert_eq!(97, String::from("az").chars().nth(0).unwrap() as usize);
}
