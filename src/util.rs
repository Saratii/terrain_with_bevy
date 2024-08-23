use bevy::{math::Vec3, prelude::Image, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}}};

use crate::{constants::{WINDOW_HEIGHT, WINDOW_WIDTH}, render::render_grid, components::Pixel};

pub fn flatten_index(x: i32, y: i32) -> usize {
    let index = ((WINDOW_HEIGHT as i32 / 2) - y) * WINDOW_WIDTH as i32 + (x + WINDOW_WIDTH as i32 / 2);
    return index as usize;
}

pub fn grid_to_image(grid: &Vec<Pixel>, width: u32, height: u32) -> Image {
    let mut image_buffer: Vec<u8> = vec![255; WINDOW_WIDTH * WINDOW_HEIGHT * 4];
    render_grid(grid, &mut image_buffer);    
    Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        image_buffer,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD
    )
}

pub fn c_to_tl(entity_position_c: &Vec3, width: f32, height: f32) -> (f32, f32){
    (entity_position_c.x + (WINDOW_HEIGHT/2) as f32 - width/2., (entity_position_c.y - (WINDOW_WIDTH/2) as f32) * -1. - height/2.)
}

pub fn flatten_index_standard_grid(x: &usize, y: &usize, grid_width: usize) -> usize {
    y * grid_width + x
}

pub fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f32 {
    ((x1 as f32 - x2 as f32).powi(2) + (y1 as f32 - y2 as f32).powi(2)).sqrt()
}