// https://leetcode.com/problems/sum-of-gcd-of-formed-pairs

use std::collections::BTreeMap;

pub fn gcd_sum(nums: Vec<i32>) -> i64 {
    let mut max = 0;
    let mut buckets: BTreeMap<i32, u32> = BTreeMap::new();
    let mut key = 0;
    let mut acc = 0;
    for num in nums {
        max = (max).max(num);
        let x = gcd(max, num);
        if x == key {
            acc += 1;
        } else {
            *buckets.entry(key).or_default() += acc;
            key = x;
            acc = 1;
        }
    }
    *buckets.entry(key).or_default() += acc;

    let mut max_key = 0;
    let mut max_count = 0;
    let mut min_key = 0;
    let mut min_count = 0;
    let mut result = 0;
    loop {
        if max_count == 0 {
            let Some(entry) = buckets.last_entry() else {
                result += (min_count as i64 / 2) * min_key as i64;
                break ;
            };
            (max_key, max_count) = entry.remove_entry();
        }
        if min_count == 0 {
            let Some(entry) = buckets.first_entry() else {
                result += (max_count as i64 / 2) * max_key as i64;
                break;
            };
            (min_key, min_count) = entry.remove_entry();
        }

        let gcd = gcd(max_key, min_key);
        let m = max_count.min(min_count);
        result += gcd as i64 * m as i64;
        max_count -= m;
        min_count -= m;
    }
    result
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::gcd_sum;

    #[test]
    fn official1() {
        assert_eq!(2, gcd_sum(vec![2, 6, 4]));
    }

    #[test]
    fn official2() {
        assert_eq!(5, gcd_sum(vec![3, 6, 2, 8]));
    }

    #[derive(serde::Deserialize, Debug)]
    struct Case {
        input: Vec<i32>,
        expected: i64,
    }

    #[allow(dead_code)]
    fn load(filename: &str) -> Case {
        use std::fs;
        use std::path::Path;

        let src_dir = Path::new(file!())
            .parent()
            .expect("Failed to get source directory");
        let test_file = src_dir.join(filename);
        let rdr = fs::File::open(&test_file)
            .unwrap_or_else(|e| panic!("Failed to open file: {:?}: {}", test_file, e));
        serde_json::from_reader(rdr).unwrap()
    }

    #[test]
    fn official995() {
        let case = load("gcd_sum_3867_test995.json");
        assert_eq!(case.expected, gcd_sum(case.input));
    }
}
