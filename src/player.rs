use bevy::{math::Vec3, prelude::{Image, Res}, time::Time};

use crate::{components::{Pixel, PixelType, Velocity}, constants::{PLAYER_HEIGHT, PLAYER_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH}, util::{c_to_tl, flatten_index_standard_grid, grid_to_image}};

pub fn generate_player_image() -> Image{
    let mut data_buffer: Vec<Pixel> = Vec::new();
    for y in 0..PLAYER_HEIGHT {
        for _ in 0..PLAYER_WIDTH {
            if y < 15 {
                data_buffer.push(Pixel { pixel_type: PixelType::PlayerSkin, gamma: 0. });
            } else {
                data_buffer.push(Pixel { pixel_type: PixelType::Black, gamma: 0. });
            }
        }
    }
    for i in 0..2 {
        data_buffer[flatten_index_standard_grid(&(2 + i), &5, PLAYER_WIDTH)] = Pixel { pixel_type: PixelType::White, gamma: 0. };
        data_buffer[flatten_index_standard_grid(&(PLAYER_WIDTH - 2 - i), &5, PLAYER_WIDTH)] = Pixel { pixel_type: PixelType::White, gamma: 0. };
    }
    for i in 0..PLAYER_WIDTH - 4 {
        data_buffer[flatten_index_standard_grid(&(2 + i), &10, PLAYER_WIDTH)] = Pixel { pixel_type: PixelType::Red, gamma: 0. };
    }
    grid_to_image(&mut data_buffer, PLAYER_WIDTH as u32, PLAYER_HEIGHT as u32, None)
}

pub fn apply_velocity(
    entity_position_c: &mut Vec3,
    velocity: &mut Velocity,
    grid: &Vec<Pixel>,
    time: &Res<Time>,
) {
    let min_x_c = -1. * WINDOW_WIDTH as f32 / 2. + PLAYER_WIDTH as f32 / 2.;
    let max_x_c = WINDOW_WIDTH as f32 / 2. - PLAYER_WIDTH as f32 / 2.;
    let entity_position_tl = c_to_tl(entity_position_c, PLAYER_WIDTH as f32, PLAYER_HEIGHT as f32);
    if velocity.vx != 0. && horizontal_collision(&velocity.vx, grid, &entity_position_tl) {
        velocity.vx = 0.;
    }
    if entity_position_c.x < min_x_c {
        entity_position_c.x = min_x_c;
        velocity.vx = 0.;
    } else if entity_position_c.x > max_x_c {
        entity_position_c.x = max_x_c;
        velocity.vx = 0.;
    }
    if velocity.vy > 0. && ((entity_position_c.y as i32 + PLAYER_HEIGHT as i32 / 2) >= (WINDOW_HEIGHT as i32 / 2) - 1 
        || vertical_collision(grid, &entity_position_tl)) {
        velocity.vy = 0.;
    }
    entity_position_c.x += velocity.vx * time.delta_seconds();
    entity_position_c.y += velocity.vy * time.delta_seconds();
}

fn horizontal_collision(velocity: &f32, grid: &Vec<Pixel>, entity_position_tl: &(f32, f32)) -> bool {
    if velocity < &0. {
        for y in 0..PLAYER_HEIGHT {
            let index = flatten_index_standard_grid(&(entity_position_tl.0 as usize - 1), &(y as usize + entity_position_tl.1 as usize), WINDOW_WIDTH);
            if !matches!(grid[index].pixel_type, PixelType::Sky | PixelType::SellBox | PixelType::Light) {
                return true
            }
        }
    } else if velocity > &0.{
        for y in 0..PLAYER_HEIGHT {
            let index = flatten_index_standard_grid(&(entity_position_tl.0 as usize + PLAYER_WIDTH + 1), &(y as usize + entity_position_tl.1 as usize), WINDOW_WIDTH);
            if !matches!(grid[index].pixel_type, PixelType::Sky | PixelType::SellBox | PixelType::Light) {
                return true
            }
        }
    }
    false
}

fn vertical_collision(grid: &Vec<Pixel>, entity_position_tl: &(f32, f32)) -> bool {
    for x in entity_position_tl.0 as usize..entity_position_tl.0 as usize + PLAYER_WIDTH as usize {
        if !matches!(grid[flatten_index_standard_grid(&x, &(entity_position_tl.1 as usize - 1), WINDOW_WIDTH)].pixel_type, PixelType::Sky | PixelType::SellBox | PixelType::Light) {
            return true
        }
    }
    false
}