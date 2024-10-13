use std::io::{self, Write};
use std::fs::File;

use bevy::{math::Vec3, prelude::Image, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}}};

use crate::color_map::SKY;
use crate::constants::{GRID_HEIGHT, GRID_WIDTH};

pub fn flatten_index(x: i32, y: i32) -> usize {
    let index = ((GRID_HEIGHT as i32 / 2) - y) * GRID_WIDTH as i32 + (x + GRID_WIDTH as i32 / 2);
    return index as usize;
}

pub fn grid_to_image(grid: &Vec<u8>, width: u32, height: u32, _perlin_mask: Option<&Vec<f32>>) -> Image {
    if grid.len() != (width * height) as usize {
        panic!("Grid and image dimensions do not match");
    }
    Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        grid.clone(),
        TextureFormat::R8Unorm,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD
    )
}

pub fn c_to_tl(entity_position_c: &Vec3, width: f32, height: f32) -> (f32, f32) {
    (entity_position_c.x + (GRID_HEIGHT/2) as f32 - width/2., (entity_position_c.y - (GRID_WIDTH/2) as f32) * -1. - height/2.)
}

pub fn tl_to_c(x: f32, y: f32, width: f32, height: f32) -> Vec3 {
    Vec3 {
        x: x + width/2. - GRID_HEIGHT as f32/2.,
        y: (y + height/2.) * -1. + (GRID_WIDTH/2) as f32,
        z: 0.
    }
}

pub fn flatten_index_standard_grid(x: &usize, y: &usize, grid_width: usize) -> usize {
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

pub fn valid_machine_spawn(terrain_grid: &Vec<u8>, position_tl: (f32, f32), width: usize, height: usize) -> bool {
    for i in 0..height {
        for j in 0..width {
            if terrain_grid[flatten_index_standard_grid(&(position_tl.0 as usize + j), &(position_tl.1 as usize + i), GRID_WIDTH)] != SKY {
                return false;
            }
        }
    }
    true
}