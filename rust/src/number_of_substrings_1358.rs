// https://leetcode.com/problems/number-of-substrings-containing-all-three-characters

pub fn number_of_substrings(s: String) -> i32 {
    let mut cursors = [0, 0, 0];
    let bytes = s.as_bytes();
    let mut set = [false, false, false];

    for i in 0..bytes.len() {
        let cursor_id = id(bytes[i]);
        set[cursor_id] = true;
        cursors[cursor_id] = i;
        if set.iter().all(|&x| x) {
            break;
        }
    }
    if !set.iter().all(|&x| x) {
        return 0;
    }
    
    let mut result: i32 = cursors.iter().copied().min().unwrap() as i32 + 1;
    for lead in (cursors.iter().copied().max().unwrap()+1)..bytes.len() {
        // dbg!(lead, &cursors, result);
        let cursor_id = id(bytes[lead]);
        cursors[cursor_id] = lead;
        let min_cursor_after= cursors.iter().copied().min().unwrap();
        // dbg!(&min_cursor_after);
        result += min_cursor_after as i32 + 1;
    }
    
    result
}

fn id (c: u8) -> usize {
    match c {
        b'a' => 0,
        b'b' => 1,
        b'c' => 2,
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
use super::number_of_substrings;
#[test]
fn official1() {
    assert_eq!(10, number_of_substrings("abcabc".to_string()));
}
#[test]
fn official2() {
    assert_eq!(3, number_of_substrings("aaacb".to_string()));
}

#[test]
fn official3() {
    assert_eq!(1, number_of_substrings("abc".to_string()));
}

#[test]
fn official49() {
    assert_eq!(0, number_of_substrings("abab".to_string()));
}

#[test]
fn t1() {
    assert_eq!(1, number_of_substrings("abbc".to_string()));
}

#[test]
fn t2() {
    assert_eq!(4, number_of_substrings("abbca".to_string()));
}


}