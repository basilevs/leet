use std::collections::HashMap;

pub fn solve_queries(nums: Vec<i32>, mut queries: Vec<i32>) -> Vec<i32> {
    let mut indices_by_values: HashMap<i32, Vec<usize>> = HashMap::new();
    
    for query in queries.iter() {
        let query = *query as usize;
        indices_by_values.entry(*nums.get(query).unwrap()).or_insert_with(|| vec![]);
    }
    for (i, v) in nums.iter().enumerate() {
        let Some(indices) = indices_by_values.get_mut(v) else {
            continue;
        };
        indices.push(i);
    }
    dbg!(&indices_by_values);
    for output in queries.iter_mut() {
        let query = *output as usize;
        let value = nums[query];
        *output = {
            let indices = indices_by_values.get(&value).unwrap();
            let pos = indices.binary_search(&query).expect("Query should always be present in the index");
            let candidates = [pos.saturating_sub(1), (pos + 1) % indices.len(), 0, indices.len() - 1];
            dbg!(&value, &candidates);
            candidates
                .into_iter()
                .filter(|p| *p != pos)
                .map(|p| {
                    ((indices[p] + nums.len() - query) % nums.len()) as i32
                })
                .min()
                .unwrap_or(-1)
        }
    }
    queries
}

#[test]
fn official1() {
    assert_expected(&[2,-1,3], &[1,3,1,4,1,3,2], &[0,3,5]);
}

#[test]
fn official2() {
    assert_expected(&[-1,-1,-1,-1], &[1,2,3,4], &[0,1,2,3]);
}



#[cfg(test)]
fn assert_expected(output: &[i32], nums: &[i32], queries: &[i32]) {
    assert_eq!(output.to_vec(), solve_queries(nums.to_vec(), queries.to_vec()));
}