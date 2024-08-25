use bevy::{asset::{Assets, Handle}, prelude::{Image, Query, ResMut, With, Without}};

use crate::components::{Grid, ImageBuffer, PickaxeTag, Pixel, ShovelTag, TerrainGridTag};

pub fn render_grid(grid: &Vec<Pixel>, image_buffer: &mut Vec<u8>) {
    for i in 0..grid.len() {
        match grid[i] {
            Pixel::Ground => {
                image_buffer[4*i] = 88;
                image_buffer[4*i+1] = 57;
                image_buffer[4*i+2] = 39;
                image_buffer[4*i+3] = 255;
            },
            Pixel::Sky => {
                image_buffer[4*i] = 135;
                image_buffer[4*i+1] = 206;
                image_buffer[4*i+2] = 235;
                image_buffer[4*i+3] = 255;
            },
            Pixel::Clear => {
                image_buffer[4*i] = 0;
                image_buffer[4*i+1] = 0;
                image_buffer[4*i+2] = 0;
                image_buffer[4*i+3] = 0;
            },
            Pixel::TranslucentGrey => {
                image_buffer[4*i] = 135;
                image_buffer[4*i+1] = 206;
                image_buffer[4*i+2] = 235;
                image_buffer[4*i+3] = 150;
            },
            Pixel::White => {
                image_buffer[4*i] = 255;
                image_buffer[4*i+1] = 255;
                image_buffer[4*i+2] = 255;
                image_buffer[4*i+3] = 255;
            },
            Pixel::Rock => {
                image_buffer[4*i] = 100;
                image_buffer[4*i+1] = 100;
                image_buffer[4*i+2] = 100;
                image_buffer[4*i+3] = 255;
            },
            Pixel::Gravel => {
                image_buffer[4*i] = 211;
                image_buffer[4*i+1] = 211;
                image_buffer[4*i+2] = 211;
                image_buffer[4*i+3] = 255;
            },
            Pixel::Red => {
                image_buffer[4*i] = 255;
                image_buffer[4*i+1] = 0;
                image_buffer[4*i+2] = 0;
                image_buffer[4*i+3] = 255;
            },
        };
    }
}

pub fn render_scene(
    mut grid_query: Query<&mut Grid, (With<TerrainGridTag>, Without<ShovelTag>)>,
    mut shovel_grid_query: Query<&mut Grid, (With<ShovelTag>, Without<TerrainGridTag>)>,
    mut grid_image_buffer_query: Query<&mut ImageBuffer, (Without<PickaxeTag>, Without<ShovelTag>)>,
    mut cursor_image_buffer_query: Query<&mut ImageBuffer, With<ShovelTag>>,
    mut images: ResMut<Assets<Image>>,
    mut grid_image_query: Query<&Handle<Image>, With<TerrainGridTag>>,
    mut cursor_image_query: Query<&Handle<Image>, With<ShovelTag>>,
){
    let mut grid_image_buffer = grid_image_buffer_query.get_single_mut().unwrap();
    let mut cursor_image_buffer = cursor_image_buffer_query.get_single_mut().unwrap();
    let shovel_grid = shovel_grid_query.get_single_mut().unwrap();
    let grid = grid_query.get_single_mut().unwrap();
    render_grid(&grid.data, &mut grid_image_buffer.data);
    render_grid(&shovel_grid.data, &mut cursor_image_buffer.data);
    if let Some(image) = images.get_mut(grid_image_query.single_mut()) {
        image.data = grid_image_buffer.data.clone();
    }
    if let Some(image) = images.get_mut(cursor_image_query.single_mut()) {
        image.data = cursor_image_buffer.data.clone()
    }
}