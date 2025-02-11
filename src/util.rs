use std::io::{self, Write};
use std::fs::File;

use bevy::{math::Vec3, prelude::Image, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}}};

use crate::constants::CHUNK_SIZE;

pub fn flatten_index(x: i32, y: i32) -> usize {
    let index = ((CHUNK_SIZE as i32 / 2) - y) * CHUNK_SIZE as i32 + (x + CHUNK_SIZE as i32 / 2);
    return index as usize;
}

pub fn grid_to_image(grid: &Vec<u8>, width: u32, height: u32, _perlin_mask: Option<&Vec<f32>>) -> Image {
    if grid.len() != (width * height) as usize {
        panic!("Grid and image dimensions do not match");
    }
    Image::new(Extent3d { width, height, depth_or_array_layers: 1 },
        TextureDimension::D2,
        grid.clone(),
        TextureFormat::R8Unorm,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD
    )
}

pub fn c_to_tl(entity_position_c: &Vec3, width: f32, height: f32) -> (f32, f32) {
    (entity_position_c.x + CHUNK_SIZE/2. - width/2., entity_position_c.y - CHUNK_SIZE/2. * -1. - height/2.)
}

pub fn tl_to_c(x: f32, y: f32, width: f32, height: f32) -> Vec3 {
    Vec3 {
        x: x + width/2. - CHUNK_SIZE as f32/2.,
        y: (y + height/2.) * -1. + CHUNK_SIZE/2.,
        z: 0.
    }
}

pub fn flatten_index_standard_grid(x: &usize, y: &usize, grid_width: usize) -> usize {
    #[cfg(debug_assertions)]
    {
        if y >= &grid_width {
            panic!("Whoopsie: y {} is out of range of 0 .. {} \nCalled with x={}, y={}", y, grid_width, x, y);
        }
    }
    y * grid_width + x
}

pub fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f32 {
    ((x1 as f32 - x2 as f32).powi(2) + (y1 as f32 - y2 as f32).powi(2)).sqrt()
}

pub fn write_u8s_to_file(width: usize, data: Vec<u8>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    for chunk in data.chunks(width) {
        let line = chunk.iter()
                        .map(|&byte| byte.to_string())
                        .collect::<Vec<_>>()
                        .join("");
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

pub fn valid_machine_spawn(_chunk_map: &Vec<Vec<u8>>, _position_g: Vec3, _width: usize, _height: usize) -> bool {
    // for y in position_g.y as i32 - height as i32/2..position_g.y as i32 + height as i32/2 {
    //     for x in position_g.x as i32 - width as i32/2..position_g.x as i32 + width as i32/2 {
    //         let chunk_x_g = get_chunk_x_g(x as f32);
    //         let chunk_y_g = get_chunk_x_g(y as f32);
    //         let chunk_x_v = get_chunk_x_v(chunk_x_g);
    //         let chunk_y_v = get_chunk_y_v(chunk_y_g);
    //         let chunk_index = flatten_index_standard_grid(&chunk_x_v, &chunk_y_v, CHUNKS_HORIZONTAL as usize);
    //         let local_x = get_local_x(x);
    //         let local_y = get_local_y(y);
    //         let local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
    //         if chunk_map[chunk_index][local_index] != SKY {
    //             return false;
    //         }
    //     }
    // }
    true
}

// pub fn world_grid_index_to_chunk_vec_index_shift(x: i32, y: i32) -> (usize, usize) {
//     ((x + CHUNKS_HORIZONTAL as i32 / 2) as usize, (y + CHUNKS_VERTICAL as i32 / 2) as usize)
// }

// pub fn get_chunk_x_v(chunk_x_g: i32) -> usize {
//     #[cfg(debug_assertions)]
//     {
//         if chunk_x_g < -CHUNKS_HORIZONTAL as i32 / 2 {
//             panic!("Whoopsie: chunk_x_g {} is out of range of {} .. {} ", chunk_x_g, -CHUNKS_HORIZONTAL as i32 / 2, CHUNKS_HORIZONTAL as i32 / 2);
//         } else
//         if   (chunk_x_g + CHUNKS_HORIZONTAL as i32 / 2) as usize > CHUNKS_HORIZONTAL as usize - 1 {
//             panic!("Whoopsie 1: x_v {} is out of range of 0 .. {} ", (chunk_x_g + CHUNKS_HORIZONTAL as i32 / 2) as usize, CHUNKS_HORIZONTAL as usize);
//         }
//     }
//     (chunk_x_g + CHUNKS_HORIZONTAL as i32 / 2) as usize
// }

// pub fn get_chunk_y_v(chunk_y_g: i32) -> usize {
//     #[cfg(debug_assertions)]
//     {
//         if chunk_y_g < -CHUNKS_VERTICAL as i32 / 2 {
//             panic!("Whoopsie: chunk_y_g {} is out of range of {} .. {} ", chunk_y_g, -CHUNKS_VERTICAL as i32 / 2, CHUNKS_VERTICAL as i32 / 2);
//         } else
//         if   (chunk_y_g + CHUNKS_VERTICAL as i32 / 2) as usize > CHUNKS_VERTICAL as usize - 1 {
//             panic!("Whoopsie 1: y_v {} is out of range of 0 .. {} ", (chunk_y_g + CHUNKS_VERTICAL as i32 / 2) as usize, CHUNKS_VERTICAL as usize);
//         }
//     }
//     (chunk_y_g + CHUNKS_VERTICAL as i32 / 2) as usize

// }

// //shifts a chunk index to the center of the world grid
// pub fn chunk_index_x_y_to_world_grid_index_shift(x: usize, y: usize) -> (i32, i32) {
//     (x as i32 - CHUNKS_HORIZONTAL as i32 / 2, y as i32 - CHUNKS_VERTICAL as i32 / 2)
// }

// pub fn chunk_index_x_to_world_grid_index_shift(x: usize) -> i32 {
//     x as i32 - CHUNKS_HORIZONTAL as i32 / 2
// }

// pub fn chunk_index_y_to_world_grid_index_shift(y: usize) -> i32 {
//     y as i32 - CHUNKS_VERTICAL as i32 / 2
// }

//global_chunk_index and top left Y to world coordinate:
pub fn get_global_y_coordinate(chunk_y_g: i32, local_y: usize) -> i32 {
    chunk_y_g * CHUNK_SIZE as i32 + CHUNK_SIZE as i32 / 2 as i32 - local_y as i32
}

pub fn get_global_x_coordinate(chunk_x_g: i32, local_x: usize) -> i32 {
    chunk_x_g * CHUNK_SIZE as i32 - CHUNK_SIZE as i32 / 2 as i32 + local_x as i32
}

pub fn get_local_x(global_x: i32) -> usize {
    let x = CHUNK_SIZE as i32 / 2 + (global_x % CHUNK_SIZE as i32);
    if x >= CHUNK_SIZE as i32{
        return (x - CHUNK_SIZE as i32) as usize;
    } else if x < 0 {
        return (CHUNK_SIZE as i32 + x) as usize;
    }
    x as usize
}

pub fn get_local_y(global_x: i32) -> usize {
    (299 - global_x).rem_euclid(600) as usize
}

pub fn get_chunk_x_g(x_g: i32) -> i32 {
    (x_g + CHUNK_SIZE as i32 / 2).div_euclid(CHUNK_SIZE as i32)
}

pub fn get_chunk_y_g(y_g: i32) -> i32 {
    (y_g + CHUNK_SIZE as i32 / 2).div_euclid(CHUNK_SIZE as i32)
}

pub fn local_to_global_x(chunk_x_g: i32, local_x: usize) -> i32 {
    chunk_x_g * CHUNK_SIZE as i32 + local_x as i32 - CHUNK_SIZE as i32 / 2
}

pub fn local_to_global_y(chunk_y_g: i32, local_y: usize) -> i32 {
    chunk_y_g * CHUNK_SIZE as i32 + CHUNK_SIZE as i32 / 2 - local_y as i32
}

#[cfg(test)]
mod tests {
    use crate::{constants::CHUNK_SIZE, util::{get_chunk_x_g, get_chunk_y_g, get_local_x, get_local_y, local_to_global_x}};

    #[test]
    fn test_get_local_y() {
        assert_eq!(get_local_y(-300), 599);
        assert_eq!(get_local_y(-301), 0);
        assert_eq!(get_local_y(0), 299);
        assert_eq!(get_local_y(300), 599);
        assert_eq!(get_local_y(299), 0);
    }

    #[test]
    fn test_get_local_x_chunk() {
        assert_eq!(get_chunk_x_g(0), 0);
        assert_eq!(get_chunk_x_g(300), 1);
        assert_eq!(get_chunk_x_g(-300), 0);
        assert_eq!(get_chunk_x_g(-301), -1);
    }

    #[test]
    fn test_get_local_y_chunk() {
        assert_eq!(get_chunk_y_g(0), 0);
        assert_eq!(get_chunk_y_g(300), 1);
        assert_eq!(get_chunk_y_g(299), 0);
        assert_eq!(get_chunk_y_g(-300), 0);
        assert_eq!(get_chunk_y_g(-301), -1);
    }

    #[test]
    fn test_get_local_x() {
        assert_eq!(get_local_x(-301), 599);
        assert_eq!(get_local_x(-400), 500);
        assert_eq!(get_local_x(0), CHUNK_SIZE as usize / 2);
        assert_eq!(get_local_x(-300), 0);
        assert_eq!(get_local_x(300 + 1 * CHUNK_SIZE as i32 + 200), 200);
        assert_eq!(get_local_x(301), 1);
        assert_eq!(get_local_x(400), 100);
        assert_eq!(get_local_x(0), 300);
        assert_eq!(get_local_x(100), 400);
        assert_eq!(get_local_x(-100), 200);
        assert_eq!(get_local_x(300 + -1 * CHUNK_SIZE as i32 + 200), 200);
        assert_eq!(get_local_x(300), 0);
    }

    // #[test]
    // fn test_get_chunk_y_v() {
    //     assert_eq!(super::get_chunk_y_v(CHUNKS_VERTICAL as i32/2), 0);
    //     assert_eq!(super::get_chunk_y_v(CHUNKS_VERTICAL as i32/-2), CHUNKS_VERTICAL as usize - 1);
    // }

    #[test]
    fn test_local_to_global_x() {
        assert_eq!(local_to_global_x(0, (CHUNK_SIZE as i32/2) as usize), 0);
    }

    #[test]
    fn test_local_to_global_y() {
        assert_eq!(super::local_to_global_y(0, (CHUNK_SIZE as i32/2) as usize), 0);
    }
}