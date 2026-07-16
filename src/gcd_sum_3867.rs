// https://leetcode.com/problems/sum-of-gcd-of-formed-pairs

pub fn gcd_sum(mut nums: Vec<i32>) -> i64 {
    let mut max = 0;
    for num in nums.iter_mut() {
        max = (max).max(*num);
        *num = gcd(max, *num)
    }
    nums.sort_unstable();
    nums.iter().zip(nums.iter().rev()).take(nums.len()/2).map(|(a, b)| gcd(*a, *b) as i64).sum()
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
