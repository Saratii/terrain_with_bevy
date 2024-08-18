use bevy::{input::ButtonInput, math::Vec3, prelude::{Image, KeyCode, MouseButton, Mut, Query, Res, Transform, With}, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}}, time::Time, window::{PrimaryWindow, Window}};

use crate::{components::Velocity, constants::{CURSOR_ORBITAL_RADIUS, CURSOR_RADIUS, MAX_PLAYER_SPEED, PLAYER_COLOR, PLAYER_HEIGHT, PLAYER_SPAWN_X, PLAYER_SPAWN_Y, PLAYER_WIDTH, SKY_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH}, world_generation::{does_gravity_apply_to_entity, flatten_index, Pixel}};

pub fn generate_player_image() -> Image{
    let mut data_buffer: Vec<u8> = Vec::with_capacity(4 * PLAYER_WIDTH * PLAYER_HEIGHT);
    for _ in 0..PLAYER_WIDTH * PLAYER_HEIGHT{
        data_buffer.push(PLAYER_COLOR[0]);
        data_buffer.push(PLAYER_COLOR[1]);
        data_buffer.push(PLAYER_COLOR[2]);
        data_buffer.push(PLAYER_COLOR[3]);

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

pub fn generate_cursor_image() -> Image{
    let mut data_buffer: Vec<u8> = Vec::with_capacity(4 * CURSOR_RADIUS * 2 * CURSOR_RADIUS * 2);
    for y in 0..CURSOR_RADIUS * 2 {
        for x in 0..CURSOR_RADIUS * 2 {
            let dx = x as isize - CURSOR_RADIUS as isize;
            let dy = y as isize - CURSOR_RADIUS as isize;
            let distance_squared = dx * dx + dy * dy;
            if distance_squared >= (CURSOR_RADIUS as isize - 1).pow(2) 
                && distance_squared <= (CURSOR_RADIUS as isize).pow(2) {
                data_buffer.push(255);
                data_buffer.push(255);
                data_buffer.push(255);
                data_buffer.push(255);
            } else {
                data_buffer.push(0);
                data_buffer.push(0);
                data_buffer.push(0);
                data_buffer.push(0);
            }
        }
    }
    Image::new(
        Extent3d {
            width: CURSOR_RADIUS as u32*2,
            height: CURSOR_RADIUS as u32*2,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data_buffer,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD
    )
}

pub fn move_player(grid: &Vec<Pixel>, keys: Res<ButtonInput<KeyCode>>, transform: &mut Mut<Transform>, velocity: &mut Mut<Velocity>, time: Res<Time>){
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
        cursor_position.translation.x = player.translation.x + CURSOR_ORBITAL_RADIUS * angle.cos();
        cursor_position.translation.y = player.translation.y + CURSOR_ORBITAL_RADIUS * angle.sin();
    }
}

pub fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f32 {
    ((x1 as f32 - x2 as f32).powi(2) + (y1 as f32 - y2 as f32).powi(2)).sqrt()
}

pub fn check_mouse_click(buttons: Res<ButtonInput<MouseButton>>, cursor_position: &Mut<Transform>, grid: &mut Vec<Pixel>){
    let mut count = 0;
    if buttons.just_pressed(MouseButton::Left){
        let left = cursor_position.translation.x as i32 - CURSOR_RADIUS as i32;
        let right = cursor_position.translation.x as i32 + CURSOR_RADIUS as i32;
        let top = cursor_position.translation.y as i32 + CURSOR_RADIUS as i32; 
        let bottom = cursor_position.translation.y as i32 - CURSOR_RADIUS as i32;
        for y in bottom..top{
            for x in left..right{
                if distance(x, y, cursor_position.translation.x as i32, cursor_position.translation.y as i32) < CURSOR_RADIUS as f32 {
                    let index = flatten_index(x as i32, y as i32);
                    match grid[index] {
                        Pixel::Ground => {
                            count += 1;
                            grid[index] = Pixel::Sky;
                        },
                        _ => {

                        }
                    }
                }
            }
        }
        println!("count: {}", count);
    }
}