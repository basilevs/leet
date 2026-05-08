use std::collections::{HashMap, HashSet, VecDeque};


    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let mut indices_by_factor: HashMap<i32, Vec<usize>> = HashMap::new();
        {
            let mut primes = HashSet::new();
            let mut factors = vec![];
            for (i, &value) in nums.iter().enumerate() {
                if value == 1 {
                    continue;
                }
                factors.clear();
                factorize(value, &mut factors);
                if factors.len() == 1 && factors[0] == value{
                    primes.insert(value);
                }
                for &factor in factors.iter() {
                    indices_by_factor.entry(factor).or_default().push(i);
                }
            }
            indices_by_factor.retain(|k, _| primes.contains(k));
        }
        let mut q = VecDeque::new();
        let mut distances: Vec<i32> = vec![i32::MAX; nums.len()];
        q.push_back(0_usize);
        distances[0] = 0;
        let end = nums.len() - 1;
        let mut update_neighbor = |current, neighbor, queue: &mut VecDeque<usize>| {
            debug_assert!(distances[current] != i32::MAX);
            let distance: i32 = distances[neighbor];
            if distance == i32::MAX {
                distances[neighbor] = distances[current] + 1;
                queue.push_back(neighbor);
            }
            neighbor == end
        };
        loop {
            let current = q.pop_front().expect("can't find the path");

            if current > 0 && update_neighbor(current, current-1, &mut q) {
                return distances[end];
            }

            if current + 1 < nums.len() && update_neighbor(current, current+1, &mut q) {
                return distances[end];
            }

            for &n in indices_by_factor.get(&nums[current]).unwrap_or(&Vec::new()) {
                if update_neighbor(current, n, &mut q) {
                    return distances[end];
                }
            }
        }
    }

const PRIMES: [i32; 180] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173,
    179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
    283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409,
    419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541,
    547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809,
    811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941,
    947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063, 1069
];



fn factorize(mut n: i32, factors_output: &mut Vec<i32> ) {
    debug_assert!(n >= 1);
    let root = (f64::sqrt(n.into()) + 1_f64) as i32;
    assert!(root < *PRIMES.last().unwrap());

    for &p in PRIMES.iter() {
        if p > root || p > n {
            break;
        }
        if n % p == 0 {
            factors_output.push(p);
            while n % p == 0 && n > 1 {
                n /= p;
            }
        }
    }
    if factors_output.is_empty() || n > 1 {
        factors_output.push(n);
    }
}

#[test]
fn official1() {
    assert_eq!(2, min_jumps(vec![1,2,4,6]));
}

#[test]
fn official2() {
    assert_eq!(2, min_jumps(vec![2,3,4,7,9]));
}

#[test]
fn official3() {
    assert_eq!(3, min_jumps(vec![4,6,5,8]));
}

#[test]
fn t1() {
    assert_eq!(1, min_jumps(vec![2,6,5,8]));
}

#[test]
fn t2() {
    assert_eq!(1, min_jumps(vec![2,2,6,5,8]));
}

