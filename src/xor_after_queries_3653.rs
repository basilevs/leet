use std::{collections::HashMap, iter::once, ops::RangeInclusive, sync::Arc, thread::{self, JoinHandle}};

const M: i64 = 10i64.pow(9) + 7;
pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    xor_after_queries_chunked(&mut nums, remove_overlaps(vector_to_query_vector(queries)))
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

fn process_chunk(nums: &mut [i32], start: usize, queries: &[Query]) -> i32 {
    for query in queries {
        let Some(r) = query.key.range.end().checked_sub(start) else {
            continue;
        };
        let l = query.key.range.start();
        if *l >= start + nums.len() {
            continue;
        }
        let k = query.key.step;
        let l = l.checked_sub(start).unwrap_or((k + l % k - start % k) % k);
        apply_query(nums, l, r, k, query.value);
    }
    nums.iter_mut().fold(0, |a, b| a ^ *b)
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Key {
    range: RangeInclusive<usize>,
    step: usize,
}

impl Key {
    fn as_singleton(&self) -> Option<Key> {
        if self.range.start() + self.step > *self.range.end() {
            Some(Key {range: *self.range.start()..=*self.range.start(), step: 1})
        } else {
            None
        }
    }
}

impl From<&[i32]> for Key {
    fn from(query: &[i32]) -> Key {
        assert!(query[0] <= query[1]);
        Key { range: query[0] as usize ..= query[1] as usize, step: query[2] as usize}
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Query {
    key: Key,
    value: i32
}

impl Query {
    fn from(input: &[i32]) -> Query {
        Query {key: Key::from(input), value: input[3]}
    }
    fn update(&mut self, value: i32) {
        self.value = ((self.value as i64 * value as i64) % M) as i32;
    }
    fn as_singleton(&self) -> Option<Query> {
        self.key.as_singleton().map(|k| Query {key: k, value: self.value})
    }
    // Move compatible part of input parameter, updating self, leaving sleftovers.
    fn steal(&mut self, that: &mut Query) -> bool {
        if self.key.step != that.key.step {
            return false;
        }
        if self.value == 1 {
            return false;
        }
        if that.value == 1 {
            return false;
        }
        if self.key.range.start() % self.key.step != that.key.range.start() % self.key.step {
            return false;
        }
        if !self.key.range.contains(that.key.range.start()) && !self.key.range.contains(that.key.range.end()) {
            return false;
        }
        if self.key.range == that.key.range {
            self.update(that.value);
            that.value = 1;
            return true;
        }
        if that.key.range.start() < self.key.range.start() && that.key.range.end() == self.key.range.end() { 
            self.update(that.value);
            that.key.range = *that.key.range.start()..=(self.key.range.start()-1);
            return true;
        }

        if that.key.range.end() > self.key.range.end() && that.key.range.start() == self.key.range.start() { 
            self.update(that.value);
            let range = next_mod_base(*self.key.range.end(), *self.key.range.start(), self.key.step)..=*that.key.range.end();
            if range.end() < range.start() {
                that.value = 1;
            } else {
                assert!(range.start() <= range.end());
                that.key.range = range;
            }
            return true;
        }
        false
    }
}

impl From<&[i32]> for Query {
    fn from(query: &[i32]) -> Query {
        Query { key: Key::from(query), value: query[3] }
    }
}

impl From<[i32; 4]> for Query {
    fn from(query: [i32; 4]) -> Query {
        Query { key: Key::from(query.as_slice()), value: query[3] }
    }
}


fn next_mod_base(after_excluding: usize, base: usize, modulo: usize) -> usize {
    let result = after_excluding + modulo - (after_excluding - base) % modulo;
    // dbg!(after_excluding, base, modulo, result);
    assert_eq!(base % modulo, result % modulo);
    assert!(result > after_excluding);
    result
}

fn remove_overlaps(queries: Vec<Query>) -> Vec<Query> {
    let mut by_step:HashMap<usize, Vec<Query>> = HashMap::new();
    for i in queries {
        by_step.entry(i.key.step).or_default().push(i);
    }
    let mut singletons = by_step.remove(&1).unwrap_or(vec!());
    for same_step_queries in by_step.values_mut() {
        // remove_singletons_into(same_step_queries, &mut singletons);
        while remove_overlaps_within_step(same_step_queries) {
            remove_singletons_into(same_step_queries, &mut singletons);
        }
    }
    while remove_overlaps_within_step(&mut singletons) {}
    singletons.extend(by_step.into_values().flatten());
    singletons
}

fn remove_singletons_into<S>(queries: &mut Vec<Query>, sink: &mut S)
where
    S: Extend<Query>,
{
    queries.retain(|i| {
        if let Some(candidate) = i.as_singleton() {
            sink.extend(once(candidate));
            false
        } else {
            true
        }
    });
}

fn remove_overlaps_within_step(queries: &mut Vec<Query>) -> bool {
    let mut result = false;
    queries.retain(|q| q.value != 1);
    queries.sort_by(|q1, q2| q1.key.range.end().cmp(q2.key.range.end()));
    result |= steal_in_order(queries);
    queries.sort_by(|q1, q2| q1.key.range.start().cmp(q2.key.range.start()));
    result |= steal_in_order(queries);
    result
}

fn steal_in_order(queries: &mut Vec<Query>) -> bool {
    if queries.len() <= 1 {
        return false;
    }
    let mut result = false;
    let mut receiver_index = 0;
    while receiver_index < queries.len()-1 {
        let mut receiver = queries.get(receiver_index).expect("unexpected OOB").clone();
        let mut donor_index = receiver_index + 1;
        while donor_index < queries.len()-1 {
            let donor = queries.get_mut(donor_index).expect("unexpected OOB");
            if !receiver.steal(donor) {
                break;
            }
            result = true;
            donor_index += 1;
        }
        queries[receiver_index] = receiver;
        receiver_index = donor_index;
    }
    queries.retain(|q| q.value != 1);
    result
}

fn xor_after_queries_chunked(nums: &mut [i32], queries: Vec<Query>) -> i32 {
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
    nums.iter().fold(0, |a, b| a ^ *b)
}


#[test]
fn official1() {
    let input = [1, 1, 1];
    let queries = [[0, 2, 1, 4]];
    run_all_algorithms(&input, &queries, 4);
}
#[test]
fn official2() {
    let input = [2, 3, 1, 5, 4];
    let queries = [[1, 4, 2, 3], [0, 2, 1, 2]];
    run_all_algorithms(&input, &queries, 31);
}

#[test]
fn error1() {
    let queries = [[0,0,1,13],[0,0,1,17],[0,0,1,9],[0,0,1,18],[0,0,1,16],[0,0,1,6],[0,0,1,4],[0,0,1,11],[0,0,1,7],[0,0,1,18],[0,0,1,8],[0,0,1,15],[0,0,1,12]];
    run_all_algorithms(&[780], &queries, 523618060);
}

#[test]
fn error2() {
    let queries = [[0,1,2,7],[1,1,2,11],[0,1,2,2],[1,1,1,11],[1,1,2,1],[0,0,1,9],[0,1,2,4],[1,1,1,6],[0,0,2,17]];
    run_all_algorithms(&[562,62], &queries, 4839076);
}

#[test]
fn error3() {
    let queries = [[0,0,2,14],[0,1,1,6],[0,0,1,3],[0,0,2,4],[1,1,2,2],[0,0,2,4],[1,1,1,3],[1,1,1,3],[1,1,1,2]];
    run_all_algorithms(&[931,613], &queries, 3883640);
}

#[test]
fn error4() {
    let queries = [[2,2,2,20],[0,2,1,19],[0,2,3,9],[1,2,1,11],[2,2,1,11],[0,2,2,11],[1,1,2,2],[0,1,1,14],[1,2,3,8],[2,2,1,14],[2,2,3,10],[2,2,3,1],[1,1,2,12],[0,2,1,15],[0,2,1,3],[1,1,3,15],[1,1,2,2]];
    run_all_algorithms(&[329,112,80], &queries, 426005772);
}

#[test]
fn optimize_error2() {
    let queries = [[0,1,2,7],[1,1,2,11],[0,1,2,2],[1,1,1,11],[1,1,2,1],[0,0,1,9],[0,1,2,4],[1,1,1,6],[0,0,2,17]];
    // let nums = [562,62];
    assert_optimization_correct(&queries);
    // let queries = to_query_vector(&queries);
    // let optimized = remove_overlaps(queries.clone());
    // // dbg!(&queries, &optimized, process_chunk(&mut vec!(0, 1), 0, &queries), process_chunk(&mut vec!(0, 1), 0, &optimized));
    // let actual = process_chunk(&mut Vec::from(nums), 0, &optimized);
    // let expected = process_chunk(&mut Vec::from(nums), 0, &queries);
    // assert_eq!(expected, actual);
}
#[test]
fn optimize_error3() {
    let queries = [[0,0,2,14],[0,1,1,6],[0,0,1,3],[0,0,2,4],[1,1,2,2],[0,0,2,4],[1,1,1,3],[1,1,1,3],[1,1,1,2]];
    assert_optimization_correct(&queries);
}
#[test]
fn optimize_error4() {
    let queries = [[0,1,2,7],[1,1,2,11],[0,1,2,2],[1,1,1,11],[1,1,2,1],[0,0,1,9],[0,1,2,4],[1,1,1,6],[0,0,2,17]];
    assert_optimization_correct(&queries);
}

#[cfg(test)]
fn assert_optimization_correct(queries: &[[i32; 4]]) {
    let queries = to_query_vector(queries);
    let optimized = remove_overlaps(queries.clone());
    assert!(optimized.len() <= queries.len());
    dbg!(&queries, &optimized, process_chunk(&mut vec!(0, 1), 0, &queries), process_chunk(&mut vec!(0, 1), 0, &optimized));
    let actual = process_chunk(&mut vec!(1, 1, 1), 0, &optimized);
    let expected = process_chunk(&mut vec!(1, 1, 1), 0, &queries);
    assert_eq!(expected, actual);
    let actual = process_chunk(&mut vec!(10000, 10000, 10000), 0, &optimized);
    let expected = process_chunk(&mut vec!(10000, 10000, 10000), 0, &queries);
    assert_eq!(expected, actual);
}

#[cfg(test)]
fn run_all_algorithms(nums: &[i32], queries: &[[i32; 4]], expected: i32) {
    assert_optimization_correct(queries);
    assert_eq!(expected, xor_after_queries_unchecked(Vec::from(nums), to_vector(&queries)));
    assert_eq!(expected, xor_after_queries_sliced(&mut Vec::from(nums), to_vector(&queries).as_slice()));
    assert_eq!(expected, xor_after_queries_chunked(&mut Vec::from(nums), to_query_vector(&queries)));
    let optmiized = remove_overlaps(to_query_vector(&queries));
    dbg!(queries, &optmiized);
    assert_eq!(expected, xor_after_queries_chunked(&mut Vec::from(nums), optmiized));
    assert_eq!(expected, xor_after_queries(Vec::from(nums), to_vector(queries)));
}

#[cfg(test)]
fn to_vector(input: &[[i32; 4]]) -> Vec<Vec<i32>> {
    input.iter().map(Vec::from).collect()
}

#[cfg(test)]
fn to_query_vector(input: &[[i32; 4]]) -> Vec<Query> {
    input.iter().map(|v| Query::from(v)).collect()
}

fn vector_to_query_vector(input: Vec<Vec<i32>>) -> Vec<Query> {
    input.iter().map(|v| Query::from(v)).collect()
}
