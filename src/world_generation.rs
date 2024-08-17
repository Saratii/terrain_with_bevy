use bevy::asset::{Assets, Handle};
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, MouseButton, Or, Query, ResMut, With, Without};
use bevy::time::Time;
use bevy::window::{PrimaryWindow, Window};
use iyes_perf_ui::entries::PerfUiBundle;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::utils::default;

use bevy::{asset::AssetServer, core_pipeline::core_2d::Camera2dBundle, ecs::system::{Commands, Res}, math::Vec3, render::{render_resource::{Extent3d, TextureDimension, TextureFormat}, texture::Image}, sprite::SpriteBundle, transform::components::Transform};
use crate::components::{CursorTag, Grid, GridImageTag, ImageBuffer, PlayerTag, Position, Velocity};
use crate::constants::{GROUND_HEIGHT, MAX_LAYERS, PLAYER_HEIGHT, PLAYER_SPAWN_X, PLAYER_SPAWN_Y, PLAYER_WIDTH, SKY_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::player::{self, check_mouse_click, draw_cursor, generate_player_image};
use crate::render::render_grid;

#[derive(Clone, Debug)]
pub enum Pixel {
    Ground, 
    Sky, 
}

pub fn setup_world(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(PerfUiBundle::default());
    commands.spawn(Camera2dBundle::default());
    let grid_data = generate_grid();
    let image_buffer = grid_to_image(&grid_data);
    // commands.spawn((
    //     SpriteBundle{
    //         texture: assets.add(image_buffer.clone()),
    //         transform: Transform { translation: Vec3 { x: 0., y: 0., z: 0. }, ..default()},
    //         ..default()
    //     },
    // ));
    // commands.spawn(ImageBuffer{data: image_buffer.data});
    // commands.spawn(Grid{data: grid_data});
    commands.spawn(PlayerTag)
            .insert(Position{x: PLAYER_SPAWN_X as f32, y: PLAYER_SPAWN_Y as f32})
            .insert(Velocity{vx: 0.0, vy: 0.0})
            .insert(SpriteBundle{
                texture: assets.add(generate_player_image()),
                transform: Transform { translation: Vec3 { x: PLAYER_SPAWN_X as f32, y: PLAYER_SPAWN_Y as f32, z: 1. }, ..default()},
                ..default()
            })
            .insert(GridImageTag);
    commands.spawn(CursorTag)
            .insert(Position{x: 100.0, y: 100.0});
}

fn generate_grid() -> Vec<Pixel> {
    let mut grid: Vec<Pixel> = Vec::with_capacity(4 * WINDOW_WIDTH * WINDOW_HEIGHT);
    for _ in 0..SKY_HEIGHT * WINDOW_WIDTH{
        grid.push(Pixel::Sky);
    }
    for _ in 0..GROUND_HEIGHT * WINDOW_WIDTH{
        grid.push(Pixel::Ground);
    }
    grid
}

fn grid_to_image(grid: &Vec<Pixel>) -> Image {
    let mut image_buffer: Vec<u8> = vec![255; WINDOW_WIDTH * WINDOW_HEIGHT * 4];
    for i in 0..grid.len() {
        match grid[i] {
            Pixel::Ground => {
                image_buffer[4*i] = 88;
                image_buffer[4*i+1] = 57;
                image_buffer[4*i+2] = 39;
            },
            Pixel::Sky => {
                image_buffer[4*i] = 135;
                image_buffer[4*i+1] = 206;
                image_buffer[4*i+2] = 235;
            },
        };
    }
    Image::new(
        Extent3d {
            width: WINDOW_WIDTH as u32,
            height: WINDOW_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        image_buffer,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD
    )
}

pub fn update(
    mut grid_query: Query<&mut Grid>,
    mut images: ResMut<Assets<Image>>,
    mut image_query: Query<&Handle<Image>, With<GridImageTag>>, 
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Velocity), (With<PlayerTag>, Without<CursorTag>)>,
    time: Res<Time>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut cursor_query: Query<&mut Position, (With<CursorTag>, Without<PlayerTag>)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut image_buffer_query: Query<&mut ImageBuffer>,
) {
    // let grid = grid_query.get_single_mut().unwrap();
    // let mut image_buffer = image_buffer_query.get_single_mut().unwrap();
    let player = player_query.get_single_mut().unwrap();
    println!("x {:?}, y: {:?}", player.0.translation.x, player.0.translation.y);
    // if let Some(image) = images.get_mut(image_query.single_mut()) {
        // let mut player = player_query.get_single_mut().unwrap();
        // let mut cursor = cursor_query.get_single_mut().unwrap();
        // move_player(&mut grid.data, keys, &mut player, time);
        // draw_cursor(&mut grid.data, q_windows, &mut player, &mut cursor);
        // check_mouse_click(buttons, &cursor, &mut grid.data);
        // println!("{:?}", grid.data);
        // render_grid(&grid.data, &mut image_buffer.data); //makes vec instead of image
        // image.data = image_buffer.data.clone();
    // }
}

pub fn does_gravity_apply_to_entity(entity_x: usize, entity_y: usize, entity_width: usize, entity_height: usize, grid: &Vec<Pixel>) -> bool {
    for x in entity_x..entity_x + entity_width {
        let index = flatten_index(x, entity_y + entity_height, WINDOW_WIDTH);
        match grid[index]{
            Pixel::Sky => continue,
            _ => return true
        }
    }
    false
}

pub fn flatten_index(x: usize, y: usize, width: usize) -> usize {
    4 * (y * width + x)
}

// pub fn move_rect(start_x: &usize, start_y: &usize, width: usize, height: usize, end_x: usize, end_y: usize, grid: &mut Vec<Pixel>, window_width: usize) {
//     let rect = remove_rect(start_x, start_y, width, height, grid, window_width);
//     draw_rect(end_x, end_y, width, height, rect, grid, window_width);
// }

// pub fn remove_rect(start_x: &usize, start_y: &usize, width: usize, height: usize, grid: &mut Vec<Pixel>, window_width: usize) -> Vec<u8> {
//     let mut rect: Vec<u8> = Vec::with_capacity(width * height * 4);
//     for y in *start_y..start_y + height {
//         for x in *start_x..start_x + width {
//             let i = flatten_index(x, y, window_width);
//             rect.push(grid[i].pop().unwrap());
//             rect.push(grid[i + 1].pop().unwrap());
//             rect.push(grid[i + 2].pop().unwrap());
//             rect.push(grid[i + 3].pop().unwrap());
//         }
//     }
//     rect
// }

pub fn draw_rect(x_pos: usize, y_pos: usize, width: usize, height: usize, rect: Vec<u8>, grid: &mut Vec<Vec<u8>>, window_width: usize) {
    let mut rect_index = 0;
    for y in 0..height {
        for x in 0..width {
            grid[flatten_index(x_pos + x, y_pos + y, window_width)][0] = rect[rect_index];
            grid[flatten_index(x_pos + x, y_pos + y, window_width) + 1][0] = rect[rect_index + 1];
            grid[flatten_index(x_pos + x, y_pos + y, window_width) + 2][0] = rect[rect_index + 2];
            grid[flatten_index(x_pos + x, y_pos + y, window_width) + 3][0] = rect[rect_index + 3];
            rect_index += 4;
        }
    }
}

// pub fn unflatten_grid(grid: &Vec<u8>, width: usize, height: usize) -> Vec<Vec<Vec<u8>>> {
//     let mut result: Vec<Vec<Vec<u8>>> = Vec::with_capacity(height);
//     for y in 0..height {
//         let mut row: Vec<u8> = Vec::with_capacity(width);
//         for x in 0..width {
//             let i = flatten_index(x, y, width);
//             row.push(grid[i]);
//             row.push(grid[i + 1]);
//             row.push(grid[i + 2]);
//             row.push(grid[i + 3]);
//         }
//         result.push(row);
//     }
//     result
// }

pub fn display_2d_grid(grid: &Vec<Vec<Vec<u8>>>) {
    for row in grid {
        for pixel in row {
            for layer_value in pixel{
                if *layer_value as u32 == 0 {
                    print!("000 ")
                } else {
                    print!("{:?} ", pixel);
                }
            }
        }
        println!();
    }
}