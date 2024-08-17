// #[cfg(test)]
// mod tests {
//     use crate::world_generation::{display_2d_grid, flatten_index, move_rect, remove_rect, unflatten_grid};

//     #[test]
//     fn test_move_rect_2x2(){
//         let mut grid: Vec<Vec<u8>> = Vec::with_capacity(4 * 4 * 4);
//         for _ in 0..grid.capacity() {
//             grid[0].push(0);
//         }
//         grid[flatten_index(1, 1, 4)][0] = 255;
//         grid[flatten_index(1, 1, 4) + 1][0] = 254;
//         grid[flatten_index(1, 1, 4) + 2][0] = 253;
//         grid[flatten_index(2, 2, 4)][0] = 252;
//         grid[flatten_index(2, 2, 4) + 1][0] = 251;
//         grid[flatten_index(2, 2, 4) + 2][0] = 250;
//         grid[flatten_index(1, 2, 4)][0] = 249;
//         grid[flatten_index(1, 2, 4) + 1][0] = 248;
//         grid[flatten_index(1, 2, 4) + 2][0] = 247;
//         grid[flatten_index(2, 1, 4)][0] = 246;
//         grid[flatten_index(2, 1, 4) + 1][0] = 245;
//         grid[flatten_index(2, 1, 4) + 2][0] = 244;
//         move_rect(&1, &1, 2, 2, 0, 0, &mut grid, 4);
//         display_2d_grid(&unflatten_grid(&grid, 4, 4));
//         let expected_grid = vec![
//             255, 254, 253, 000, 246, 245, 244, 000, 000, 000, 000, 000, 000, 000, 000, 000,
//             249, 248, 247, 000, 252, 251, 250, 000, 135, 206, 235, 000, 000, 000, 000, 000,
//             000, 000, 000, 000, 135, 206, 235, 000, 135, 206, 235, 000, 000, 000, 000, 000,
//             000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000
//         ];
//         assert_eq!(grid, expected_grid);
//     }

//     #[test]
//     fn test_move_rect_1x1(){
//         let mut grid: Vec<u8> = Vec::with_capacity(2 * 1 * 4);
//         for _ in 0..grid.capacity() {
//             grid.push(0);
//         }
//         grid[flatten_index(0, 1, 1)] = 255;
//         grid[flatten_index(0, 1, 1) + 1] = 254;
//         grid[flatten_index(0, 1, 1) + 2] = 253;
//         move_rect(&0, &1, 1, 1, 0, 0, &mut grid, 1);
//         let expected = vec![255, 254, 253, 0, 135, 206, 235, 0];
//         assert_eq!(grid, expected)
//     }

//     #[test]
//     fn test_flatten_index_at_0_0_4(){
//         assert_eq!(flatten_index(0, 0, 4), 0);
//     }

//     #[test]
//     fn test_flatten_index_at_1_1_4(){
//         assert_eq!(flatten_index(1, 1, 4), 4*4 + 4*1);
//     }

//     #[test]
//     fn test_flatten_index_at_1_2_4(){
//         assert_eq!(flatten_index(1, 2, 4), 4*4*2 + 4*1);
//     }

//     #[test]
//     fn test_remove_rect(){
//         let mut grid: Vec<u8> = Vec::with_capacity(4 * 4 * 4);
//         for _ in 0..grid.capacity() {
//             grid.push(0);
//         }
//         grid[flatten_index(1, 1, 4)] = 255;
//         grid[flatten_index(1, 1, 4) + 1] = 254;
//         grid[flatten_index(1, 1, 4) + 2] = 253;
//         grid[flatten_index(2, 2, 4)] = 252;
//         grid[flatten_index(2, 2, 4) + 1] = 251;
//         grid[flatten_index(2, 2, 4) + 2] = 250;
//         grid[flatten_index(1, 2, 4)] = 249;
//         grid[flatten_index(1, 2, 4) + 1] = 248;
//         grid[flatten_index(1, 2, 4) + 2] = 247;
//         grid[flatten_index(2, 1, 4)] = 246;
//         grid[flatten_index(2, 1, 4) + 1] = 245;
//         grid[flatten_index(2, 1, 4) + 2] = 244;
//         let expected_rect = vec![255, 254, 253, 0, 246, 245, 244, 0, 249, 248, 247, 0, 252, 251, 250, 0];
//         let expected_grid = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 135, 206, 235, 0, 135, 206, 235, 0, 0, 0, 0, 0, 0, 0, 0, 0, 135, 206, 235, 0, 135, 206, 235, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
//         assert_eq!(remove_rect(&1, &1, 2, 2, &mut grid, 4), expected_rect);
//         assert_eq!(grid, expected_grid);
//     }
// }

