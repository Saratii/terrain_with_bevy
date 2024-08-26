use std::cmp::min;

use bevy::{math::Vec3, prelude::{Image, Mut, Query, Transform, With, Without}, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}}, window::{PrimaryWindow, Window}};

use crate::{components::{CurrentTool, PickaxeTag, Pixel, PlayerTag, ShovelTag, Tool, Velocity}, constants::{CURSOR_BORDER_WIDTH, CURSOR_ORBITAL_RADIUS, CURSOR_RADIUS, PLAYER_HEIGHT, PLAYER_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH}, util::{c_to_tl, distance, flatten_index_standard_grid, grid_to_image}};

pub fn generate_player_image() -> Image{
    let mut data_buffer: Vec<Pixel> = Vec::new();
    for y in 0..PLAYER_HEIGHT {
        for x in 0..PLAYER_WIDTH {
            if y < 15 {
                data_buffer.push(Pixel::PlayerSkin);
            } else {
                data_buffer.push(Pixel::Black);
            }
        }
    }
    for i in 0..2{
        data_buffer[flatten_index_standard_grid(&(2 + i), &5, PLAYER_WIDTH)] = Pixel::White;
        data_buffer[flatten_index_standard_grid(&(PLAYER_WIDTH - 2 - i), &5, PLAYER_WIDTH)] = Pixel::White;
    }
    for i in 0..PLAYER_WIDTH - 4{
        data_buffer[flatten_index_standard_grid(&(2 + i), &10, PLAYER_WIDTH)] = Pixel::Red;
    }
    grid_to_image(&mut data_buffer, PLAYER_WIDTH as u32, PLAYER_HEIGHT as u32)
}

pub fn generate_shovel_grid() -> Vec<Pixel>{
    let mut data_buffer: Vec<Pixel> = Vec::with_capacity(CURSOR_RADIUS * 2 * CURSOR_RADIUS * 2);
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

pub fn generate_pickaxe_grid() -> Vec<Pixel> {
    let mut data_buffer: Vec<Pixel> = Vec::with_capacity(CURSOR_RADIUS * 2 * CURSOR_RADIUS * 2);
    for y in 0..CURSOR_RADIUS * 2 {
        for x in 0..CURSOR_RADIUS * 2 {
            let distance = distance(x as i32, y as i32, CURSOR_RADIUS as i32, CURSOR_RADIUS as i32);
            if distance > CURSOR_RADIUS as f32 {
                data_buffer.push(Pixel::Clear);
            } else if distance < CURSOR_RADIUS as f32 - CURSOR_BORDER_WIDTH {
                data_buffer.push(Pixel::TranslucentGrey);
            } else {
                data_buffer.push(Pixel::Red);
            }
        }
    }
    data_buffer
}

pub fn apply_velocity(entity_position_c: &mut Vec3, velocity: &mut Mut<Velocity>, grid: &Vec<Pixel>) {
    let min_x_c = -1. * WINDOW_WIDTH as f32 / 2. + PLAYER_WIDTH as f32 / 2.;
    let max_x_c = WINDOW_WIDTH as f32 / 2. - PLAYER_WIDTH as f32 / 2.;
    if entity_position_c.x < min_x_c {
        entity_position_c.x = min_x_c;
        velocity.vx = 0.;
    } else if entity_position_c.x > max_x_c {
        entity_position_c.x = max_x_c;
        velocity.vx = 0.;
    } else {
        let entity_position_tl = c_to_tl(entity_position_c, PLAYER_WIDTH as f32, PLAYER_HEIGHT as f32);
        if horizontal_collision(&velocity.vx, grid, &entity_position_tl) {
            velocity.vx = 0.;
        }
    }
    entity_position_c.y += velocity.vy;
    entity_position_c.x += velocity.vx;
}

fn horizontal_collision(velocity: &f32, grid: &Vec<Pixel>, entity_position_tl: &(f32, f32)) -> bool{
    if velocity < &0.{
        for y in 0..PLAYER_HEIGHT {
            let index = flatten_index_standard_grid(&(entity_position_tl.0 as usize - 1), &(y as usize + entity_position_tl.1 as usize), WINDOW_WIDTH);
            if grid[index] != Pixel::Sky && grid[index] != Pixel::SellBox{
                return true
            }
        }
    } else if velocity > &0.{
        for y in 0..PLAYER_HEIGHT {
            let index = flatten_index_standard_grid(&(entity_position_tl.0 as usize + PLAYER_WIDTH + 1), &(y as usize + entity_position_tl.1 as usize), WINDOW_WIDTH);
            if grid[index] != Pixel::Sky && grid[index] != Pixel::SellBox{
                return true
            }
        }
    }
    false
}

pub fn update_tool(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<(&mut Transform, &mut Velocity), (With<PlayerTag>, Without<ShovelTag>)>,
    mut shovel_query: Query<&mut Transform, (With<ShovelTag>, (Without<PlayerTag>, Without<PickaxeTag>))>,
    mut pickaxe_query: Query<&mut Transform, (With<PickaxeTag>, (Without<PlayerTag>, Without<ShovelTag>))>,
    current_tool_query: Query<&CurrentTool>,
){
    let player = player_query.get_single_mut().unwrap();
    let current_tool = current_tool_query.get_single().unwrap();
    let mut tool_position;
    if current_tool.tool == Tool::Pickaxe{
        tool_position = pickaxe_query.get_single_mut().unwrap();
    } else {
        tool_position = shovel_query.get_single_mut().unwrap();
    }
    if let Some(position) = q_windows.single().cursor_position() {
        let converted_position_x = position.x - WINDOW_WIDTH as f32 / 2.;
        let converted_position_y = (position.y - WINDOW_HEIGHT as f32 / 2.) * -1.;
        let angle = (converted_position_y - player.0.translation.y).atan2(converted_position_x - player.0.translation.x);
        let distance_from_player = distance(player.0.translation.x as i32, player.0.translation.y as i32, converted_position_x as i32, converted_position_y as i32);
        let min_distance = min(CURSOR_ORBITAL_RADIUS as usize, distance_from_player as usize) as f32;
        tool_position.translation.x = player.0.translation.x + min_distance * angle.cos();
        tool_position.translation.y = player.0.translation.y + min_distance * angle.sin();
    }
}

pub fn update_shovel_content_visual(shovel_image_grid: &mut Vec<Pixel>, shovel_contents: &Vec<Pixel>){
    for color in shovel_image_grid.iter_mut(){
        if matches!(*color, Pixel::Ground(_) | Pixel::Gravel){
            *color = Pixel::TranslucentGrey;
        }
    }
    let mut drawn_content = 0;
    for color in shovel_image_grid.iter_mut().rev(){
        if drawn_content == shovel_contents.len(){
            return
        }
        if *color == Pixel::TranslucentGrey {
            let pixel = &shovel_contents[drawn_content];
            *color = pixel.clone();
            drawn_content += 1;
        }
    }
}