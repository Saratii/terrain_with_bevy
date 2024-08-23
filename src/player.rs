use std::{cmp::min, collections::HashSet};

use bevy::{input::ButtonInput, math::Vec3, prelude::{Image, KeyCode, MouseButton, Mut, Query, Res, Transform, With}, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}}, time::Time, window::{PrimaryWindow, Window}};

use crate::{components::{Count, Velocity}, constants::{CURSOR_BORDER_WIDTH, CURSOR_ORBITAL_RADIUS, CURSOR_RADIUS, MAX_PLAYER_SPEED, MAX_SHOVEL_CAPACITY, PLAYER_COLOR, PLAYER_HEIGHT, PLAYER_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH}, world_generation::{does_gravity_apply_to_entity, flatten_index, Pixel}};

pub fn generate_player_image() -> Image{
    let mut data_buffer: Vec<u8> = vec![255; 4 * PLAYER_WIDTH * PLAYER_HEIGHT];
    for y in 0..PLAYER_HEIGHT {
        for x in 0..PLAYER_WIDTH {
            if y < 15 {
                data_buffer[flatten_index_standard_grid(x, y, PLAYER_WIDTH) * 4] = PLAYER_COLOR[0];
                data_buffer[flatten_index_standard_grid(x, y, PLAYER_WIDTH) * 4 + 1] = PLAYER_COLOR[1];
                data_buffer[flatten_index_standard_grid(x, y, PLAYER_WIDTH) * 4 + 2] = PLAYER_COLOR[2];
            } else {
                data_buffer[flatten_index_standard_grid(x, y, PLAYER_WIDTH) * 4] = 0;
                data_buffer[flatten_index_standard_grid(x, y, PLAYER_WIDTH) * 4 + 1] = 0;
                data_buffer[flatten_index_standard_grid(x, y, PLAYER_WIDTH) * 4 + 2] = 0;
            }
        }
    }
    for i in 0..2{
        data_buffer[flatten_index_standard_grid(2 + i, 5, PLAYER_WIDTH) * 4] = 255;
        data_buffer[flatten_index_standard_grid(2 + i, 5, PLAYER_WIDTH) * 4 + 1] = 255;
        data_buffer[flatten_index_standard_grid(2 + i, 5, PLAYER_WIDTH) * 4 + 2] = 255;
        data_buffer[flatten_index_standard_grid(PLAYER_WIDTH - 2 - i, 5, PLAYER_WIDTH) * 4] = 255;
        data_buffer[flatten_index_standard_grid(PLAYER_WIDTH - 2 - i, 5, PLAYER_WIDTH) * 4 + 1] = 255;
        data_buffer[flatten_index_standard_grid(PLAYER_WIDTH - 2 - i, 5, PLAYER_WIDTH) * 4 + 2] = 255;
    }
    for i in 0..PLAYER_WIDTH - 4{
        data_buffer[flatten_index_standard_grid(2 + i, 10, PLAYER_WIDTH) * 4] = 255;
        data_buffer[flatten_index_standard_grid(2 + i, 10, PLAYER_WIDTH) * 4 + 1] = 0;
        data_buffer[flatten_index_standard_grid(2 + i, 10, PLAYER_WIDTH) * 4 + 2] = 0;
    }
    Image::new(
        Extent3d {
            width: PLAYER_WIDTH as u32,
            height: PLAYER_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data_buffer,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD
    )
}

pub fn generate_cursor_grid() -> Vec<Pixel>{
    let mut data_buffer: Vec<Pixel> = Vec::with_capacity(4 * CURSOR_RADIUS * 2 * CURSOR_RADIUS * 2);
    for y in 0..CURSOR_RADIUS * 2 {
        for x in 0..CURSOR_RADIUS * 2 {
            let distance = distance(x as i32, y as i32, CURSOR_RADIUS as i32, CURSOR_RADIUS as i32);
            if distance > CURSOR_RADIUS as f32 {
                data_buffer.push(Pixel::Clear);
            } else if distance < CURSOR_RADIUS as f32 - CURSOR_BORDER_WIDTH {
                data_buffer.push(Pixel::TranslucentGrey);
            } else {
                data_buffer.push(Pixel::White);
            }
        }
    }
    data_buffer
}

pub fn move_player(grid: &Vec<Pixel>, keys: Res<ButtonInput<KeyCode>>, transform: &mut Mut<Transform>, velocity: &mut Mut<Velocity>, time: &Res<Time>){
    let does_gravity_apply = does_gravity_apply_to_entity(transform.translation.x as i32,  transform.translation.y as i32, PLAYER_WIDTH as i32, PLAYER_HEIGHT as i32, grid);
    if does_gravity_apply {
        velocity.vy -= 1. * time.delta_seconds();
    } else {
        velocity.vy = 0.;
    }
    if keys.pressed(KeyCode::KeyA) {
        if velocity.vx * -1. < MAX_PLAYER_SPEED {
            velocity.vx -= 1. * time.delta_seconds();
        }
    } else if keys.pressed(KeyCode::KeyD) {
         if velocity.vx < MAX_PLAYER_SPEED {
            velocity.vx += 1. * time.delta_seconds();
        }
    }
    if keys.pressed(KeyCode::Space){
        if !does_gravity_apply{
            velocity.vy += 150. * time.delta_seconds();
        }
    }
    apply_velocity(&mut transform.translation, velocity);
}

fn apply_velocity(position: &mut Vec3, velocity: &mut Mut<Velocity>) {
    position.y += velocity.vy;
    position.x += velocity.vx;

    let min_x = -1. * WINDOW_WIDTH as f32 / 2. + PLAYER_WIDTH as f32 / 2.;
    let max_x = WINDOW_WIDTH as f32 / 2. - PLAYER_WIDTH as f32 / 2.;

    if position.x < min_x {
        position.x = min_x;
        velocity.vx = 0.;
    } else if position.x > max_x {
        position.x = max_x;
        velocity.vx = 0.;
    }
}

pub fn update_cursor(q_windows: Query<&Window, With<PrimaryWindow>>, player: &mut Mut<Transform>, cursor_position: &mut Mut<Transform>){
    if let Some(position) = q_windows.single().cursor_position() {
        let converted_position_x = position.x - WINDOW_WIDTH as f32 / 2.;
        let converted_position_y = (position.y - WINDOW_HEIGHT as f32 / 2.) * -1.;
        let angle = (converted_position_y - player.translation.y).atan2(converted_position_x - player.translation.x);
        let distance_from_player = distance(player.translation.x as i32, player.translation.y as i32, converted_position_x as i32, converted_position_y as i32);
        let min_distance = min(CURSOR_ORBITAL_RADIUS as usize, distance_from_player as usize) as f32;
        cursor_position.translation.x = player.translation.x + min_distance * angle.cos();
        cursor_position.translation.y = player.translation.y + min_distance * angle.sin();
    }
}

pub fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f32 {
    ((x1 as f32 - x2 as f32).powi(2) + (y1 as f32 - y2 as f32).powi(2)).sqrt()
}

pub fn flatten_index_standard_grid(x: usize, y: usize, grid_width: usize) -> usize {
    y * grid_width + x
}

pub fn check_mouse_click(buttons: Res<ButtonInput<MouseButton>>, cursor_position: &Mut<Transform>, grid: &mut Vec<Pixel>, cursor_content_count: &mut Count, shovel_grid: &mut Vec<Pixel>, gravity_affected_columns: &mut HashSet<usize>){
    if buttons.just_pressed(MouseButton::Middle) {
    }
    if buttons.just_pressed(MouseButton::Right) {
        for y in 0..CURSOR_RADIUS * 2 {
            for x in 0..CURSOR_RADIUS * 2 {
                let shovel_grid_index = flatten_index_standard_grid(x, y, CURSOR_RADIUS * 2);
                if shovel_grid[shovel_grid_index] == Pixel::Ground {
                    let main_grid_index = flatten_index(cursor_position.translation.x as i32 - CURSOR_RADIUS as i32 + x as i32, cursor_position.translation.y as i32 - CURSOR_RADIUS as i32 + (CURSOR_RADIUS * 2 - y - 1) as i32);
                    if grid[main_grid_index] == Pixel::Sky {
                        grid[main_grid_index] = Pixel::Ground;
                        cursor_content_count.count -= 1;
                        gravity_affected_columns.insert(main_grid_index % WINDOW_WIDTH);
                    }
                }
            }
        }
        update_shovel_content_visual(shovel_grid, cursor_content_count.count);
    }
    if buttons.just_pressed(MouseButton::Left) && cursor_content_count.count < MAX_SHOVEL_CAPACITY {
        let left = cursor_position.translation.x as i32 - CURSOR_RADIUS as i32;
        let right = cursor_position.translation.x as i32 + CURSOR_RADIUS as i32;
        let top = cursor_position.translation.y as i32 + CURSOR_RADIUS as i32; 
        let bottom = cursor_position.translation.y as i32 - CURSOR_RADIUS as i32;
        for y in bottom..top{
            for x in left..right{
                if distance(x, y, cursor_position.translation.x as i32, cursor_position.translation.y as i32) < CURSOR_RADIUS as f32 - CURSOR_BORDER_WIDTH {
                    let index = flatten_index(x as i32, y as i32);
                    match grid[index] {
                        Pixel::Ground => {
                            cursor_content_count.count += 1;
                            grid[index] = Pixel::Sky;
                            gravity_affected_columns.insert(index % WINDOW_WIDTH);
                            if cursor_content_count.count == MAX_SHOVEL_CAPACITY {
                                update_shovel_content_visual(shovel_grid, cursor_content_count.count);
                                return
                            }
                        },
                        _ => {

                        }
                    }
                }
            }
        }
        update_shovel_content_visual(shovel_grid, cursor_content_count.count);
    }
}

fn update_shovel_content_visual(shovel_image_grid: &mut Vec<Pixel>, ground_count: usize){
    for color in shovel_image_grid.iter_mut(){
        if *color == Pixel::Ground {
            *color = Pixel::TranslucentGrey;
        }
    }
    let mut drawn_content = 0;
    for color in shovel_image_grid.iter_mut().rev(){
        if drawn_content == ground_count{
            return
        }
        if *color == Pixel::TranslucentGrey {
            *color = Pixel::Ground;
            drawn_content += 1;
        }
    }
}