// https://leetcode.com/problems/maximum-building-height

pub fn max_building(n: i32, mut restrictions: Vec<Vec<i32>>) -> i32 {

    restrictions.sort_unstable_by_key(|x| x[0] + x[1]);

    let mut max = 0;
    let mut iter = restrictions.into_iter();
    let mut x = 1;
    let mut y = 0;
    loop {
        let Some(restriction) = iter.next() else {
            break max.max(n - x + y);
        };
        
        
        let delta = restriction[0] - x + restriction[1] - y;
        
        dbg!(x, y, restriction[0], restriction[1], delta);
        debug_assert!(delta >= 0);
        x += delta / 2;
        y += delta / 2;
        
        if y >= restriction[1] {
            max = max.max(y);
            dbg!(x, y, restriction[0], restriction[1], delta, max);
            x = restriction[0];
            y = restriction[1];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::max_building;

fn to_vector<const N: usize>(input: &[[i32; N]]) -> Vec<Vec<i32>> {
    input.into_iter().map(Vec::from).collect()
}

#[test]
fn official1() {
    assert_eq!(2, max_building(5, to_vector(&[[2, 1], [4, 1]])));
}

#[test]
fn official2() {
    assert_eq!(5, max_building(6, vec![]));
}

#[test]
fn official3() {
    assert_eq!(5, max_building(10, to_vector(&[[5, 3], [2, 5], [7, 4], [10, 3]])));
}

#[test]
fn official12() {
    assert_eq!(2, max_building(10, to_vector(&[[6,2],[9,1],[5,2],[3,0],[10,2],[2,4],[7,0],[8,0],[4,4]])));
}


}