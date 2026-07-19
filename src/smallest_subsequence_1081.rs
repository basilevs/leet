// https://leetcode.com/problems/smallest-subsequence-of-distinct-characters

pub fn smallest_subsequence(s: String) -> String {
    let bs = s.into_bytes();
    let mut maxima = [usize::MAX; 26];
    for (i, current) in bs.iter().enumerate() {
        maxima[to_index(*current)] = i;
    }
    
    let mut absent:Vec<u8> = (b'a'..=b'z').collect();
    absent.retain(|x| {
        maxima[to_index(*x)] != usize::MAX
    });
    
    let to_find = absent.len();
    let mut position = 0;
    let mut result: Vec<u8> = Vec::with_capacity(to_find);
    while !absent.is_empty() {
        for candidate in &absent {
            let candidate_position = bs.iter().enumerate().skip(position).find(|&(_, x)| x == candidate).map(|(i, _)| {
                i
            }).unwrap();
            let index = to_index(*candidate);
            if maxima.iter().enumerate().filter(|(i, _)| *i != index).all(|(_, &p)| p > candidate_position) {
                position = candidate_position + 1;
                maxima[index] = usize::MAX;
                result.push(*candidate);
                break;
            }
        }
        let resolved = result.last().unwrap();
        absent.retain(|x| x != resolved);
    }
    String::from_utf8(result).unwrap()
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
