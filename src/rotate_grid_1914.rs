    pub fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        if grid.is_empty() {
            return grid;
        }
        let grid_height = grid.len();
        let grid_width = grid[0].len();

        let layer_count = grid_height.min(grid_width) / 2;
        let max_layer_length = 2 * (grid_height + grid_width) - 4;

        // It is possible to avoid additional memory allocation
        // But that requires two additional nested loops to prevent overwrites
        let mut layer_buffer = vec![0; max_layer_length];

        for layer in 0..layer_count {
            let layer_height = grid_height - 2 * layer;
            let layer_width  = grid_width - 2 * layer;
            let layer_length = 2 * (layer_width + layer_height) - 4;

            for i in 0..layer_length {
                let cell = layer_offset_to_cell(grid_height, grid_width, layer, i.try_into().unwrap());
                layer_buffer[i] = grid[cell.0][cell.1];
            }
            
            for i in 0..layer_length {
                let cell = layer_offset_to_cell(grid_height, grid_width, layer, i32::try_from(i).unwrap()-k);
                grid[cell.0][cell.1] = layer_buffer[i];
            }
        }
        grid
    }



// returns y,x
fn layer_offset_to_cell(grid_height: usize, grid_width: usize, layer: usize, clockwise_offset: i32) -> (usize, usize) {
    debug_assert!(2 * layer < grid_width);
    debug_assert!(2 * layer < grid_height);

    let layer_height = grid_height - 2 * layer;
    let layer_width  = grid_width - 2 * layer;
    debug_assert!(layer_width >= 2);
    debug_assert!(layer_height >= 2);
    let layer_length: i32 = 2 * (layer_width + layer_height) as i32 - 4;
    let positive_offset =  usize::try_from((clockwise_offset % layer_length + layer_length) % layer_length).unwrap();

    //   Offsets map:
    //
    //.  0  1  2  3
    //.  9        4
    //.  8  7  6  5

    let corner1 = layer_width - 1;
    let corner2 = corner1 + layer_height - 1;
    let corner3 = corner2 + layer_width - 1;
    let right_column = grid_width - 1 - layer;
    let bottom_row = grid_height - 1 - layer;
    debug_assert!(corner3 < layer_length as _);
    if positive_offset <= corner1 {
        (layer, layer + positive_offset)
    } else if positive_offset <= corner2 {
        (layer + positive_offset - corner1, right_column)
    } else if positive_offset <= corner3 {
        (bottom_row, right_column - (positive_offset - corner2))
    } else {
        (bottom_row - (positive_offset - corner3), layer)
    } 

}

#[cfg(test)]
fn array_to_vector<const N: usize, const M: usize>(input: [[i32; M]; N]) -> Vec<Vec<i32>>
{
    input.iter().map(|row| row.to_vec()).collect()
}

#[test]
fn official1() {
    assert_eq!(array_to_vector([[10,20],[40,30]]), rotate_grid(array_to_vector([[40,10],[30,20]]), 1));
}

#[test]
fn official2() {
    assert_eq!(array_to_vector([[3,4,8,12],[2,11,10,16],[1,7,6,15],[5,9,13,14]]), rotate_grid(array_to_vector([[1,2,3,4],[5,6,7,8],[9,10,11,12],[13,14,15,16]]), 2));
}

#[test]
fn t1() {
    assert_eq!(array_to_vector([[3,6,5],[2,1,4]]), rotate_grid(array_to_vector([[1,2,3],[4,5,6]]), 2));
}

#[test]
fn t2() {
    assert_eq!(array_to_vector([[3,6,9],[2,5,8],[1,4,7]]), rotate_grid(array_to_vector([[1,2,3],[4,5,6],[7,8,9]]), 2));
}

