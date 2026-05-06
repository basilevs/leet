
    pub fn rotate_the_box(mut box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        if box_grid.is_empty() {
            return box_grid;
        }
        if box_grid[0].is_empty() {
            return vec![];
        }
        let m = box_grid.len();
        let n = box_grid[0].len();
        for row in box_grid.iter_mut() {
            let mut count = 0_usize;
            for i in 0..n {
                if row[i] == '#' {
                    count += 1;
                    row[i] = '.';
                }
                if row[i] == '*' {
                    row[(i-count)..i].fill('#');
                    count = 0;
                }
            }
            row[(n-count)..].fill('#');
        }
        // dbg!(&box_grid);
        (0..n).map(|i| {
            (0..m).map(|j| box_grid[m-j-1][i]).collect()
        }).collect()
    }


// #[cfg(test)]
// fn to_vector(input: &[&[&str]]) -> Vec<Vec<char>> {
//     input.iter().map(|row| row.iter().map(|s| s.chars().next().unwrap()).collect::<Vec<_>>()).map(Vec::from).collect()
// }

#[cfg(test)]
fn array_to_vector<const N: usize, const M: usize>(input: [[&str; M]; N]) -> Vec<Vec<char>>
{
    input.iter().map(|row| row.iter().map(|s| s.chars().next().unwrap()).collect::<Vec<_>>()).map(Vec::from).collect()
}



#[test]
fn official1() {
    assert_eq!(array_to_vector(
        [["."],
         ["#"],
         ["#"]]),rotate_the_box(array_to_vector([["#",".","#"]])) )
}

#[test]
fn official2() {
    assert_eq!(array_to_vector(
        [["#","."],
         ["#","#"],
         ["*","*"],
         [".","."]]),rotate_the_box(array_to_vector(
        [["#",".","*","."],
         ["#","#","*","."]])) )
}

#[test]
fn official3() {
    assert_eq!(array_to_vector(
        [[".","#","#"],
         [".","#","#"],
         ["#","#","*"],
         ["#","*","."],
         ["#",".","*"],
         ["#",".","."]]),rotate_the_box(array_to_vector(
        [["#","#","*",".","*","."],
         ["#","#","#","*",".","."],
         ["#","#","#",".","#","."]])) )
}