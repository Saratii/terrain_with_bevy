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
use crate::constants::{GROUND_HEIGHT, PLAYER_SPAWN_X, PLAYER_SPAWN_Y, SKY_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::player::{check_mouse_click, generate_cursor_image, generate_player_image, move_player, update_cursor};
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
    commands.spawn((
        SpriteBundle{
            texture: assets.add(image_buffer.clone()),
            transform: Transform { translation: Vec3 { x: 0., y: 0., z: 0. }, ..default()},
            ..default()
        },
    )).insert(GridImageTag);
    commands.spawn(ImageBuffer{data: image_buffer.data});
    commands.spawn(Grid{data: grid_data});
    commands.spawn(PlayerTag)
            .insert(Position{x: PLAYER_SPAWN_X as f32, y: PLAYER_SPAWN_Y as f32})
            .insert(Velocity{vx: 0.0, vy: 0.0})
            .insert(SpriteBundle{
                texture: assets.add(generate_player_image()),
                transform: Transform { translation: Vec3 { x: PLAYER_SPAWN_X as f32, y: PLAYER_SPAWN_Y as f32, z: 1. }, ..default()},
                ..default()
            });
    commands.spawn(CursorTag)
            .insert(SpriteBundle{
                texture: assets.add(generate_cursor_image()),
                transform: Transform { translation: Vec3 { x: PLAYER_SPAWN_X as f32, y: PLAYER_SPAWN_Y as f32, z: 1. }, ..default()},
                ..default()
            });
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
    mut cursor_query: Query<&mut Transform, (With<CursorTag>, Without<PlayerTag>)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut image_buffer_query: Query<&mut ImageBuffer>,
) {
    let mut grid = grid_query.get_single_mut().unwrap();
    let mut image_buffer = image_buffer_query.get_single_mut().unwrap();
    if let Some(image) = images.get_mut(image_query.single_mut()) {
        let mut player = player_query.get_single_mut().unwrap();
        let mut cursor_position = cursor_query.get_single_mut().unwrap();
        move_player(&grid.data, keys, &mut player.0, &mut player.1, time);
        update_cursor(q_windows, &mut player.0, &mut cursor_position);
        check_mouse_click(buttons, &cursor_position, &mut grid.data);
        render_grid(&grid.data, &mut image_buffer.data); //makes vec instead of image
        image.data = image_buffer.data.clone();
    }
}

pub fn does_gravity_apply_to_entity(entity_x: i32, entity_y: i32, entity_width: i32, entity_height: i32, grid: &Vec<Pixel>) -> bool {
    for x in entity_x..entity_x + entity_width {
        let index = flatten_index(x, entity_y - entity_height/2);
        match &grid[index]{
            Pixel::Sky => continue,
            _ => return false
        }
    }
    true
}

pub fn flatten_index(x: i32, y: i32) -> usize {
    let index = ((WINDOW_HEIGHT as i32 / 2) - y) * WINDOW_WIDTH as i32 + (x + WINDOW_WIDTH as i32 / 2);
    return index as usize;
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

// pub fn draw_rect(x_pos: i32, y_pos: i32, width: i32, height: i32, rect: Vec<u8>, grid: &mut Vec<Vec<u8>>) {
//     let mut rect_index = 0;
//     for y in 0..height {
//         for x in 0..width {
//             grid[flatten_index(x_pos + x, y_pos + y)][0] = rect[rect_index];
//             grid[flatten_index(x_pos + x, y_pos + y) + 1][0] = rect[rect_index + 1];
//             grid[flatten_index(x_pos + x, y_pos + y) + 2][0] = rect[rect_index + 2];
//             grid[flatten_index(x_pos + x, y_pos + y) + 3][0] = rect[rect_index + 3];
//             rect_index += 4;
//         }
//     }
// }

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

// pub fn display_2d_grid(grid: &Vec<Vec<Vec<u8>>>) {
//     for row in grid {
//         for pixel in row {
//             for layer_value in pixel{
//                 if *layer_value as u32 == 0 {
//                     print!("000 ")
//                 } else {
//                     print!("{:?} ", pixel);
//                 }
//             }
//         }
//         println!();
//     }
// }