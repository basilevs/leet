use std::convert::identity;

    pub fn max_value(mut nums: Vec<i32>) -> Vec<i32> {
        // disjoint sets sorted by their maximums
        let mut components: Vec<Set> = Vec::with_capacity(nums.len());
        for (i, &value) in nums.iter().enumerate() {
            let j = components.binary_search(&Set { max: value, last: usize::MAX })
                .expect_err("duplicate max in components");
            let max = if j >= components.len() {
                value
            } else {
                let prev_max = components.last().unwrap().max;
                components.truncate(j);
                prev_max
            };
            components.push(Set { max, last: i });
        }

        for (i, value) in nums.iter_mut().enumerate() {
            let j = components.binary_search(&Set{max:*value, last: i}).unwrap_or_else(identity);
            *value = components[j].max;
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

#[test]
fn t1() {
    assert_eq!(vec![1, 2, 3], max_value(vec![1,2,3]));
}

#[test]
fn t2() {
    assert_eq!(vec![ 3,3,3], max_value(vec![3,2,1]));
}
