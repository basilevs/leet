// https://leetcode.com/problems/rank-transform-of-an-array

pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
    let mut sorted: Vec<usize> = (0..arr.len()).collect();
    sorted.sort_unstable_by_key(|&i| arr[i]);
    let mut rank = 0;
    let mut rank_value = i32::MIN;
    for i in 0..sorted.len() {
        let current_value = arr[sorted[i]];
        if current_value > rank_value {
            rank += 1;
            rank_value = current_value;
        }
        arr[sorted[i]] = rank;
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::array_rank_transform;

    #[test]
    fn official1() {
        assert_eq!(vec![4, 1, 2, 3], array_rank_transform(vec![40, 10, 20, 30]));
    }

    #[test]
    fn official2() {
        assert_eq!(vec![1, 1, 1], array_rank_transform(vec![100, 100, 100]));
    }

    #[test]
    fn official3() {
        assert_eq!(
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3],
            array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12])
        );
    }

    //[40,10,20,30]
    #[test]
    fn tens() {
        assert_eq!(vec![4, 1, 2, 3], array_rank_transform(vec![40, 10, 20, 30]));
    }

    // [100,100,100]
    #[test]
    fn hundreds() {
        assert_eq!(vec![1, 1, 1], array_rank_transform(vec![100, 100, 100]));
    }

    // [37,12,28,9,100,56,80,5,12]
    #[test]
    fn random() {
        assert_eq!(
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3],
            array_rank_transform(vec![37, 12, 28, 9,100, 56, 80, 5, 12]));
    }

    // []
    #[test]
    fn empty() {
        assert_eq!(vec![i32::MIN; 0], array_rank_transform(vec![]));
    }

    // [-1000000000,-1000000000,-1000000000,1000000000,1000000000,1000000000]
    #[test]
    fn edges() {
        assert_eq!(
            vec![1, 1, 1, 2, 2, 2],
            array_rank_transform(vec![-1000000000, -1000000000, -1000000000, 1000000000, 1000000000, 1000000000])
        );
    }
    // [0,0,0,-1]
    #[test]
    fn negative() {
        assert_eq!(
            vec![2, 2, 2, 1],
            array_rank_transform(vec![0, 0, 0, -1])
        );
    }
}
