use std::{collections::{BTreeMap, HashMap}, ops::RangeInclusive, sync::Arc, thread::{self, JoinHandle}};

const M: i64 = 10i64.pow(9) + 7;
pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    for query in queries.into_iter() {
        let &[l, r, k, v] = query.as_slice() else {
            panic!("All queries are expected to consist of 4 integers");
        };
        let l = usize::try_from(l).expect("expected 0 <= l <= r");
        let r = usize::try_from(r).expect("expected 0 <= r < n");
        let k = usize::try_from(k).expect("expected 1 <= k <= n");
        let v = i64::from(v);
        for idx in (l..=r).step_by(k) {
            let num = &mut nums[idx];
            let temp = (i64::from(*num) * v) % M;
            // dbg!(&idx, &num, &v, &temp);
            *num = temp.try_into().expect("Narrowing can not happen for a number that is a modulo of M");
        }
    }
    nums.into_iter().reduce(|a, b| a ^ b).unwrap_or(0)
}

pub fn xor_after_queries_unchecked(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    for query in queries {
        let (l, r, k, v) = {
            (
                query[0] as usize,
                query[1] as usize,
                query[2] as usize,
                i64::from(query[3]),
            )
        };
        for idx in (l..=r).step_by(k) {
            let num = &mut nums[idx];
            let temp = (i64::from(*num) * v) % M;
            *num = temp as i32;
        }
    }
    nums.into_iter().reduce(|a, b| a ^ b).unwrap_or(0)
}

pub fn apply_query(nums: &mut [i32], l: usize, r: usize, k:usize, v: i32) {
    for idx in (l..=r.min(nums.len()-1)).step_by(k) {
        let num = &mut nums[idx];
        let temp = (i64::from(*num) * i64::from(v)) % M;
        *num = temp as i32;
    }
}

pub fn process_chunk(nums: &mut [i32], start: usize, queries: &[Vec<i32>]) -> i32 {
    for query in queries {
        let (l, r, k, v) = {
            (
                query[0] as usize,
                query[1] as usize,
                query[2] as usize,
                query[3],
            )
        };
        let Some(r) = r.checked_sub(start) else {
            continue;
        };
        if l >= start + nums.len() {
            continue;
        }
        let l = l.checked_sub(start).unwrap_or((k + l % k - start % k) % k);
        apply_query(nums, l, r, k, v);
    }
    nums.into_iter().fold(0, |a, b| a ^ *b)
}

#[derive(PartialEq, Eq, Hash)]
struct Key {
    range: RangeInclusive<usize>,
    step: usize,
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.step != other.step {
            return None
        }
        Some(self.cmp(other))
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        assert_eq!(self.step, other.step);
        self.range.start().cmp(other.range.start()).then(self.range.end().cmp(other.range.end()))
    }
}

struct Query {
    key: Key,
    value: i32
}

impl Query {
    fn move
}


fn remove_overlaps(queries: Vec<Query>) -> Vec<Query> {
    let mut by_step:HashMap<usize, Vec<Query>> = HashMap::with_capacity(queries.len());
    for i in queries {
        by_step.entry(i.key.step).or_insert_with(|| vec!()).push(i);
    }
    for same_step_queries in by_step.values_mut() {
        *same_step_queries = remove_overlaps(same_step_queries);
    }
    by_step.into_values().flatten().collect()
}

fn remove_overlaps(queries: Vec<Query>) -> Vec<Query> {
    let mut result:BTreeMap<Key, Query> = BTreeMap::new();
    for i in queries {
        let neigbours: Vec<(&Key, &mut Query)> = result.range_mut(&i.key..&i.key.upper_bound()).collect();
        assert!(neigbours.len() <= 2);
        i.

        // let previous = result.lower_bound(i.key);
        // result.entry((i.key.l, i.key.r)).or_insert_with(default)
    }
    result.into_values().collect()
}

const NEIGHBOUR_LIST: [(i8, i8); 2] = [(1,0), (0, -1) ];

impl Key {
    fn from(query: &Vec<i32>) -> Key {
        assert!(query[0] <= query[1]);
        Key {l: query[0] as usize, r: query[1] as usize, k: query[2] as usize}
    }

    fn upper_bound(&self) -> Key {
        Key {range: RangeInclusive::new(self.range.end()+1, self.range.end() + 1), step: self.step}
    }

    fn len(&self) -> usize {
        (self.r - self.l) / self.k
    }

    fn decompose(&self) -> impl Iterator<'_, Item = (Key, Key)> {
        if self.len() < 3 {
            return [].iter();
        }
        NEIGHBOUR_LIST.iter().flat_map(|n| self.neighbour(n).iter())
    }
    
    fn neighbour(&self, n: &(i8, i8)) -> Option<(Key, Key)> {
            let Some(l_large) = self.l.checked_add_signed(isize::from(n.0) * self.k as isize) else {
                return None;
            };
            let Some(r_large) = self.l.checked_add_signed(isize::from(n.1) * self.k as isize) else {
                return None;
            };
            if l_large > r_large {
                return None
            }
            let difference = if self.l != l_large { self.l } else {self.r};
            let large = Key {l: l_large, r: r_large, k: self.k};
            let small = Key {l: difference, r:difference, k : 1};
            assert_eq!(self.len(), large.len() + small.len());
            assert_eq!(1, small.len());
            Some((large, small))
    }

}



pub fn optimize_queries(queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut by_address:HashMap<Key, i64> = HashMap::with_capacity(queries.len());
    'query_loop: for i in queries {
        let key = Key::from(&i);
        let value = i[3];

        for candidate in key.decompose() {
            if let Some(v) = by_address.get_mut(&candidate.0) {
                *v = (*v as i64 * value as i64) % M;
                let v = by_address.entry(candidate.1).or_insert(1);
                *v = (*v as i64 * value as i64) % M;
                continue 'query_loop;
            }
        }
        let v = by_address.entry(key).or_insert(1);
        *v = (*v as i64 * i[3] as i64) % M;
    }
    by_address.into_iter().map(|(k, v)| vec!(k.l as i32, k.r as i32, k.k as i32, v as i32)).collect()
}

pub fn xor_after_queries_chunked(nums: &mut [i32], queries: Vec<Vec<i32>>) -> i32 {
    let mut start_index = 0;
    let mut result = 0;
    let queries = Arc::new(queries);
    let parallelism = thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1);
    let chunk_size = 2.max(nums.len()/parallelism);
    let mut handles: Vec<JoinHandle<i32>> = Vec::new();
    for chunk in nums.chunks_mut(chunk_size) {
        let len = chunk.len();
        if len < chunk_size {
            result ^= process_chunk(chunk, start_index, &queries);
        } else {
            let start_copy = start_index;
            let queries_copy = queries.clone();
            let mut chunk_copy = Vec::from(chunk);
            handles.push(thread::spawn(move || {
                process_chunk(&mut chunk_copy, start_copy, &queries_copy)
            }));
        }
        start_index += len;
    }
    for handle in handles {
        result ^= handle.join().unwrap(); 
    }
    result
}
 
pub fn xor_after_queries_sliced(nums: &mut [i32], queries: &[Vec<i32>]) -> i32 {
    for query in queries {
        let (l, r, k, v) = {
            (
                query[0] as usize,
                query[1] as usize,
                query[2] as usize,
                query[3],
            )
        };
        apply_query(nums, l, r, k, v);
    }
    nums.into_iter().fold(0, |a, b| a ^ *b)
}


#[test]
fn official1() {
    let input = [1, 1, 1];
    let queries = [[0, 2, 1, 4]];

    assert_eq!(4, xor_after_queries(Vec::from(input), to_vector(&queries)));
    assert_eq!(4, xor_after_queries_unchecked(Vec::from(input), to_vector(&queries)));
    assert_eq!(4, xor_after_queries_chunked(Vec::from(input).as_mut_slice(), to_vector(&queries)));
}
#[test]
fn official2() {
    let input = [2, 3, 1, 5, 4];
    let queries = [[1, 4, 2, 3], [0, 2, 1, 2]];
    assert_eq!(31, xor_after_queries_chunked(&mut input.clone(), to_vector(&queries)));
    assert_eq!(
        31,
        xor_after_queries(
            Vec::from(input),
            to_vector(&queries)
        )
    );
    assert_eq!(
        31,
        xor_after_queries_unchecked(
            Vec::from(input),
            to_vector(&queries)
        )
    );
    assert_eq!(31, xor_after_queries_sliced(&mut input.clone(), to_vector(&queries).as_slice()));
}

#[test]
fn error1() {
    let queries = [[0,0,1,13],[0,0,1,17],[0,0,1,9],[0,0,1,18],[0,0,1,16],[0,0,1,6],[0,0,1,4],[0,0,1,11],[0,0,1,7],[0,0,1,18],[0,0,1,8],[0,0,1,15],[0,0,1,12]];
    assert_eq!(523618060, xor_after_queries(vec!(780), to_vector(&queries)));
    assert_eq!(523618060, xor_after_queries_unchecked(vec!(780), to_vector(&queries)));
    assert_eq!(523618060, xor_after_queries_sliced(&mut [780], to_vector(&queries).as_slice()));
    assert_eq!(523618060, xor_after_queries_chunked(&mut [780], to_vector(&queries)));
    assert_eq!(523618060, xor_after_queries_chunked(&mut [780], optimize_queries(to_vector(&queries))));
}

#[cfg(test)]
fn to_vector<const SIZE: usize>(input: &[[i32; 4]; SIZE]) -> Vec<Vec<i32>> {
    input.iter().map(Vec::from).collect()
}