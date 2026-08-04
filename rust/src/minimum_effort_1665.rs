#[cfg(test)]
use itertools::Itertools;

// You are given an array tasks where tasks[i] = [actuali, minimumi]:
// - actuali is the actual amount of energy you spend to finish the ith task.
// - minimumi is the minimum amount of energy you require to begin the ith task.
// For example, if the task is [10, 12] and your current energy is 11, you cannot start this task. However, if your current energy is 13, you can complete this task, and your energy will be 3 after finishing it.
// You can finish the tasks in any order you like.
// Return the minimum initial amount of energy you will need to finish all the tasks.
    pub fn minimum_effort(mut tasks: Vec<Vec<i32>>) -> i32 {
        // let n = naive(&tasks);
        let q = max_last(&mut tasks);
        // debug_assert_eq!(n, q);
        q
    }

fn max_last(tasks: &mut [Vec<i32>]) -> i32 {
    tasks.sort_unstable_by_key(|t| (t[1] - t[0], -t[0]));
    let mut result = 0;
    // dbg!(&tasks);
    for t in tasks {
        result = t[1].max(result + t[0]);
    }
    result
}

#[cfg(test)]
fn naive(tasks: &[Vec<i32>]) -> i32 {
    if tasks.is_empty() {
        return 0;
    }

    let mut result = i32::MAX;
    for i in 0..tasks.len() {
        let remaining: Vec<Vec<i32>> = tasks[..i].iter().chain(tasks[i+1..].iter()).cloned().collect();
        let required = tasks[i][1].max(tasks[i][0] + naive(&remaining));
        result = result.min(required);
    }
    result
}

#[cfg(test)]
fn array_to_vector<const N: usize, const M: usize>(input: [[i32; M]; N]) -> Vec<Vec<i32>> {
    input.iter().map(|row| row.to_vec()).collect()
}

#[cfg(test)]
fn assert_like_naive<const N: usize, const M: usize>(input: [[i32; M]; N]) {
    let input = array_to_vector(input);
    let n = naive(&input);
    let m = minimum_effort(input.clone());
    assert_eq!(n, m, "{:?}", &input);
}

#[test]
fn official1() {
    assert_eq!(8, minimum_effort(array_to_vector([[1,2],[2,4],[4,8]])) );
}

#[test]
fn official2() {
    assert_eq!(32, minimum_effort(array_to_vector([[1,3],[2,4],[10,11],[10,12],[8,9]])) );
}

#[test]
fn official3() {
    assert_eq!(27, minimum_effort(array_to_vector([[1,7],[2,8],[3,9],[4,10],[5,11],[6,12]]) ));
}

#[test]
fn error1() {
    assert_eq!(31, minimum_effort(array_to_vector([[2,4],[10,11],[10,12],[8,9]]) ));
}

#[test]
fn error2() {
    assert_eq!(31, minimum_effort(array_to_vector([[2,4],[10,12],[8,9],[10,11]] )));
}

#[test]
fn error3() {
    assert_eq!(29, minimum_effort(array_to_vector([[10,12],[8,9],[10,11]] )));
}

#[test]
fn error4() {
    assert_like_naive([[12, 1], [2, 4], [5, 9], [8, 10], [6, 7], [3, 11]]);
}

#[test]
fn error5() {
    assert_like_naive([[1, 2], [3, 9], [6, 10], [8, 11], [5, 7], [4, 12]]);
}

#[test]
fn brute_force() {
    let len = 5_usize;
    for p in (1..(len as i32 *2+1)).permutations(12).unique() {

        let input: Vec<Vec<i32>> = p.windows(2).step_by(2).map(|t| t.to_vec()).collect();
        if input.iter().any(|t| t[0] > t[1]) {
            continue;
        }
        debug_assert!(input.len() == len);
        let naive_result = naive(&input);
        let result = minimum_effort(input.clone());
        assert_eq!(naive_result, result, "{:?}", &input);
    }
}
