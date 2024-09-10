use bevy::{asset::{Assets, Handle}, prelude::{Image, Query, ResMut, With, Without}};

use crate::components::{DirtVariant, GravelVariant, Grid, ImageBuffer, PickaxeTag, Pixel, ShovelTag, TerrainGridTag};

pub fn render_grid(grid: &Vec<Pixel>, image_buffer: &mut Vec<u8>) {
    for i in 0..grid.len() {
        match &grid[i] {
            Pixel::Ground(variant) => {
                match variant{
                    DirtVariant::Dirt1 => {
                        image_buffer[4*i] = 88;
                        image_buffer[4*i+1] = 57;
                        image_buffer[4*i+2] = 39;
                        image_buffer[4*i+3] = 255;
                    },
                    DirtVariant::Dirt2 => {
                        image_buffer[4*i] = 92;
                        image_buffer[4*i+1] = 64;
                        image_buffer[4*i+2] = 51;
                        image_buffer[4*i+3] = 255;
                    },
                    DirtVariant::Dirt3 => {
                        image_buffer[4*i] = 155;
                        image_buffer[4*i+1] = 118;
                        image_buffer[4*i+2] = 83;
                        image_buffer[4*i+3] = 255;
                    },
                }
            },
            Pixel::Gravel(variant) => {
                match variant{
                    GravelVariant::Gravel1 => {
                        image_buffer[4*i] = 115;
                        image_buffer[4*i+1] = 115;
                        image_buffer[4*i+2] = 115;
                        image_buffer[4*i+3] = 255;
                    },
                    GravelVariant::Gravel2 => {
                        image_buffer[4*i] = 72;
                        image_buffer[4*i+1] = 72;
                        image_buffer[4*i+2] = 72;
                        image_buffer[4*i+3] = 255;
                    },
                    GravelVariant::Gravel3 => {
                        image_buffer[4*i] = 220;
                        image_buffer[4*i+1] = 210;
                        image_buffer[4*i+2] = 195;
                        image_buffer[4*i+3] = 255;
                    },
                }
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
            Pixel::Red => {
                image_buffer[4*i] = 255;
                image_buffer[4*i+1] = 0;
                image_buffer[4*i+2] = 0;
                image_buffer[4*i+3] = 255;
            },
            Pixel::SellBox => {
                image_buffer[4*i] = 106;
                image_buffer[4*i+1] = 13;
                image_buffer[4*i+2] = 173;
                image_buffer[4*i+3] = 255;
            },
            Pixel::RefinedCopper => {
                image_buffer[4*i] = 205;
                image_buffer[4*i+1] = 127;
                image_buffer[4*i+2] = 50;
                image_buffer[4*i+3] = 255;
            },
            Pixel::Black => {
                image_buffer[4*i] = 0;
                image_buffer[4*i+1] = 0;
                image_buffer[4*i+2] = 0;
                image_buffer[4*i+3] = 255;
            },
            Pixel::PlayerSkin => {
                image_buffer[4*i] = 210;
                image_buffer[4*i+1] = 180;
                image_buffer[4*i+2] = 140;
                image_buffer[4*i+3] = 255;
            },
            Pixel::Chalcopyrite => {
                image_buffer[4*i] = 196;
                image_buffer[4*i+1] = 145;
                image_buffer[4*i+2] = 2;
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