use std::io::{self, Write}; // Import the Write trait
use std::fs::File;

use bevy::{math::Vec3, prelude::Image, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}}};

use crate::{constants::{WINDOW_HEIGHT, WINDOW_WIDTH}, render::render_grid, components::Pixel};

pub fn flatten_index(x: i32, y: i32) -> usize {
    let index = ((WINDOW_HEIGHT as i32 / 2) - y) * WINDOW_WIDTH as i32 + (x + WINDOW_WIDTH as i32 / 2);
    return index as usize;
}

pub fn grid_to_image(grid: &Vec<u8>, width: u32, height: u32, perlin_mask: Option<&Vec<f32>>) -> Image {
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

pub fn c_to_tl(entity_position_c: &Vec3, width: f32, height: f32) -> (f32, f32){
    (entity_position_c.x + (WINDOW_HEIGHT/2) as f32 - width/2., (entity_position_c.y - (WINDOW_WIDTH/2) as f32) * -1. - height/2.)
}

pub fn tl_to_c(x: f32, y: f32, width: f32, height: f32) -> (f32, f32) {
    (x  + width/2. - WINDOW_HEIGHT as f32/2., (y + height/2.) * -1. + (WINDOW_WIDTH/2) as f32)
}

pub fn flatten_index_standard_grid(x: &usize, y: &usize, grid_width: usize) -> usize {
    y * grid_width + x
}

pub fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f32 {
    ((x1 as f32 - x2 as f32).powi(2) + (y1 as f32 - y2 as f32).powi(2)).sqrt()
}

pub fn write_u8s_to_file(width: usize, data: Vec<u8>, file_path: &str) -> io::Result<()> {
    // Open the file in write mode
    let mut file = File::create(file_path)?;
    
    for chunk in data.chunks(width) {
        // Join the u8 values into a space-separated string
        let line = chunk.iter()
                        .map(|&byte| byte.to_string())
                        .collect::<Vec<_>>()
                        .join("");
        
        // Write the line to the file
        writeln!(file, "{}", line)?;
    }

    Ok(())
}