    pub fn max_value(mut nums: Vec<i32>) -> Vec<i32> {
        // disjoint sets sorted by their maximums
        let mut components: Vec<Set> = vec![]; 
        for (i, &value) in nums.iter().enumerate() {
            match components.binary_search(&Set {max: value, last: usize::MAX}) {
                Ok(_) => {
                    unreachable!();
                },
                Err(j) => {
                    if j >= components.len() {
                        components.push(Set{max: value, last: i});
                    } else {
                        let &max = &components.last().unwrap().max;
                        components.drain(j..);
                        components.push(Set{max: max, last: i});
                    }
                },
            };
            // dbg!(i, &value, &components);
        }

        for (i, value) in nums.iter_mut().enumerate() {
            let upper_bound = match components.binary_search(&Set{max:*value, last: i}) {
                Ok(j) => &components[j],
                Err(j) => &components[j],
            };
            *value = upper_bound.max;
        }
        nums
    }
#[derive(PartialOrd, Ord, Eq, PartialEq, Debug)]
struct Set {
    max: i32,
    last: usize,
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
