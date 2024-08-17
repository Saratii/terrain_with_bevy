use bevy::{input::ButtonInput, prelude::{Image, KeyCode, MouseButton, Mut, Query, Res, With}, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}}, time::Time, window::{PrimaryWindow, Window}};

use crate::{components::{Position, Velocity}, constants::{CURSOR_ORBITAL_RADIUS, CURSOR_RADIUS, MAX_LAYERS, MAX_PLAYER_SPEED, PLAYER_COLOR, PLAYER_HEIGHT, PLAYER_SPAWN_X, PLAYER_SPAWN_Y, PLAYER_WIDTH, SKY_HEIGHT, WINDOW_WIDTH}, world_generation::{does_gravity_apply_to_entity, flatten_index, Pixel}};

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

// pub fn move_player(grid: &mut Vec<Pixel>, keys: Res<ButtonInput<KeyCode>>, player: &mut (Mut<Position>, Mut<Velocity>), time: Res<Time>){
//     if player.0.x <= 0.0 || player.0.x >= (WINDOW_WIDTH - PLAYER_WIDTH) as f32 { //check if its trying to move off the screen
//         player.1.vx = 0.;
//     }
//     if does_gravity_apply_to_entity(player.0.x as usize,  player.0.y as usize, PLAYER_WIDTH, PLAYER_HEIGHT, grid) {
//         player.1.vy += 1.;
//     } else {
//         player.1.vy = 0.;
//     }
//     if keys.pressed(KeyCode::KeyA) {
//         if player.1.vx * -1. < MAX_PLAYER_SPEED {
//             player.1.vx -= 1.;
//         }
//     } else if keys.pressed(KeyCode::KeyD) {
//          if player.1.vx < MAX_PLAYER_SPEED {
//             player.1.vx += 1.;
//         }
//     }
//     if keys.pressed(KeyCode::Space){
//         match grid[flatten_index(player.0.x as usize, player.0.y as usize + PLAYER_HEIGHT, WINDOW_WIDTH)] {
//             Pixel::Ground => {
//                 player.1.vy = -300.;
//             },
//             _ => {}
//         }
//     }
//     let start_x = player.0.x as usize;
//     let start_y = player.0.y as usize;
//     apply_velocity(&mut player.0, &player.1, &time);
//     // move_rect(&start_x, &start_y, PLAYER_WIDTH, PLAYER_HEIGHT, player.0.x as usize, player.0.y as usize, grid, WINDOW_WIDTH);
// }

fn apply_velocity(position: &mut Mut<Position>, velocity: &Mut<Velocity>, time: &Res<Time>) {
    position.y += velocity.vy * time.delta_seconds();
    position.x += velocity.vx * time.delta_seconds();
    if position.x < 0.0 {
        position.x = 0.0;
    } else if position.x > (WINDOW_WIDTH - PLAYER_WIDTH) as f32 {
        position.x = (WINDOW_WIDTH - PLAYER_WIDTH) as f32;
    }
}

pub fn draw_cursor(grid: &mut Vec<Pixel>, q_windows: Query<&Window, With<PrimaryWindow>>, player: &mut (Mut<Position>, Mut<Velocity>), cursor_position: &mut Mut<Position>){
    if let Some(position) = q_windows.single().cursor_position() {
        // draw_circle(cursor_position.x as usize, cursor_position.y as usize, CURSOR_RADIUS, grid);
        let angle = (position.y - (player.0.y + PLAYER_HEIGHT as f32/2.)).atan2(position.x - (player.0.x + PLAYER_WIDTH as f32/2.));
        cursor_position.x = player.0.x + PLAYER_WIDTH as f32/2. + CURSOR_ORBITAL_RADIUS * angle.cos();
        cursor_position.y = player.0.y + PLAYER_HEIGHT as f32/2. + CURSOR_ORBITAL_RADIUS * angle.sin();
        // draw_circle(cursor_position.x as usize, cursor_position.y as usize, CURSOR_RADIUS, grid);
    }
}

pub fn distance(x1: usize, y1: usize, x2: usize, y2: usize) -> f32 {
    ((x1 as f32 - x2 as f32).powi(2) + (y1 as f32 - y2 as f32).powi(2)).sqrt()
}

// pub fn draw_circle(x0: usize, y0: usize, radius: usize, grid: &mut Vec<ArrayVec<Entity, MAX_LAYERS>>) {
//     let mut x = 0;
//     let mut y = radius;
//     let mut d: i32 = 3 - 2 * radius as i32;
//     while y >= x as usize {
//         let mut i = flatten_index(x0 + x, y0 + y, WINDOW_WIDTH);
//         grid[i].push(Entity::Shovel);
//         i = flatten_index(x0 - x, y0 + y, WINDOW_WIDTH);
//         grid[i].push(Entity::Shovel);
//         i = flatten_index(x0 + x, y0 - y, WINDOW_WIDTH);
//         grid[i].push(Entity::Shovel);
//         i = flatten_index(x0 - x, y0 - y, WINDOW_WIDTH);
//         grid[i].push(Entity::Shovel);
//         i = flatten_index(x0 + y, y0 + x, WINDOW_WIDTH);
//         grid[i].push(Entity::Shovel);
//         i = flatten_index(x0 - y, y0 + x, WINDOW_WIDTH);
//         grid[i].push(Entity::Shovel);
//         i = flatten_index(x0 + y, y0 - x, WINDOW_WIDTH);
//         grid[i].push(Entity::Shovel);
//         i = flatten_index(x0 - y, y0 - x, WINDOW_WIDTH);
//         grid[i].push(Entity::Shovel);
//         if d > 0{
//             y -= 1;
//             d = d + 4 * (x as i32 - y as i32) + 10;
//         } else {
//             d = d + 4 * x as i32 + 6;
//         }
//         x += 1;
//     }
// }

pub fn check_mouse_click(buttons: Res<ButtonInput<MouseButton>>, cursor_position: &Mut<Position>, grid: &mut Vec<Vec<u8>>){
    let mut count = 0;
    if buttons.just_pressed(MouseButton::Left){
        let left = cursor_position.x as usize - CURSOR_RADIUS;
        let right = cursor_position.x as usize + CURSOR_RADIUS;
        let top = cursor_position.y as usize - CURSOR_RADIUS;
        let bottom = cursor_position.y as usize + CURSOR_RADIUS;
        for y in top..bottom{
            for x in left..right{
                if distance(x, y, cursor_position.x as usize, cursor_position.y as usize) < CURSOR_RADIUS as f32 {
                    let index = flatten_index(x, y, WINDOW_WIDTH);
                    if grid[index][0] == 155{
                        grid[index][0] = 135;
                        grid[index + 1][0] = 206;
                        grid[index + 2][0] = 235;
                        count += 1;
                    }
                }
            }
        }
        println!("count: {}", count);
    }
}