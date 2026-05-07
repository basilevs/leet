    pub fn max_value(mut nums: Vec<i32>) -> Vec<i32> {
        // disjoint components identified by their maximums
        let mut components = vec![]; 
        for &value in nums.iter() {
            match components.binary_search(&value) {
                Ok(i) => {
                    let &max = components.last().unwrap();
                    components.drain((i+1)..);
                    components.push(max);
                },
                Err(i) => {
                    if i >= components.len() {
                        components.push(value);
                    } else {
                        let &max = components.last().unwrap();
                        components.drain(i..);
                        components.push(max);
                    }
                },
            };
            dbg!(&value, &components);
        }

        for value in nums.iter_mut() {
            let upper_bound = match components.binary_search(value) {
                Ok(i) => components[i],
                Err(i) => components[i],
            };
            *value = upper_bound;
        }
        nums
    }

#[test]
fn official1() {
    assert_eq!(vec![2, 2, 3], max_value(vec![2,1,3]));
}

#[test]
fn official2() {
    assert_eq!(vec![3, 3, 3], max_value(vec![2,3,1]));
}

#[test]
fn official962() {
    assert_eq!(vec![11,18,18], max_value(vec![11,18,11]));
}




