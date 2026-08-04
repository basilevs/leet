// https://leetcode.com/problems/smallest-subsequence-of-distinct-characters

pub fn smallest_subsequence(s: String) -> String {
    let bs = s.into_bytes();
    let mut last = [0usize; 26];
    for (i, current) in bs.iter().enumerate() {
        last[to_index(*current)] = i;
    }
    
    let mut in_stack = [false; 26];
    let mut stack: Vec<u8> = Vec::with_capacity(26);
    for (i, &current) in bs.iter().enumerate() {
        let index = to_index(current);
        if in_stack[index] {
            continue;
        }
        while let Some(&top) = stack.last() {
            if top > current && last[to_index(top)] > i {
                stack.pop();
                in_stack[to_index(top)] = false;
            } else {
                break;
            }
        }
        stack.push(current);
        in_stack[index] = true;
    }
    String::from_utf8(stack).unwrap()
}

fn to_index(c: u8) -> usize {
    usize::from(c) - usize::from(b'a')
}

#[cfg(test)]
mod tests {
    use super::smallest_subsequence;

    #[test]
    fn official1() {
        assert_eq!("abc", smallest_subsequence("bcabc".to_string()));
    }

    #[test]
    fn official2() {
        assert_eq!("acdb", smallest_subsequence("cbacdcbc".to_string()));
    }
}
