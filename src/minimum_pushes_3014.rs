// https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-i

pub fn minimum_pushes(word: String) -> i32 {
    let mut freq = [0u8; 26];
    for b in word.bytes() {
        freq[(b - b'a') as usize] += 1;
    }
    freq.sort_unstable();

    let mut used_slots = 0i32;
    let mut result = 0i32;
    for f in freq {
        if f > 0 {
            result += (used_slots/8 + 1) * i32::from(f);
            used_slots += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::minimum_pushes;

    #[test]
    fn official1() {
        // Input from exampleTestcases; expected output copied from examples given in content.
        assert_eq!(5, minimum_pushes("abcde".to_string()));
    }

    #[test]
    fn official2() {
        // Input from exampleTestcases; expected output copied from examples given in content.
        assert_eq!(12, minimum_pushes("xycdefghij".to_string()));
    }
}
