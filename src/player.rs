use std::cmp::min;

use bevy::{input::ButtonInput, math::Vec3, prelude::{Image, KeyCode, MouseButton, Mut, Query, Res, Transform, With, Without}, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}}, time::Time, window::{PrimaryWindow, Window}};

use crate::{components::{Count, CursorTag, Grid, Pixel, PlayerTag, TerrainGridTag, TerrainPositionsAffectedByGravity, Velocity}, constants::{CURSOR_BORDER_WIDTH, CURSOR_ORBITAL_RADIUS, CURSOR_RADIUS, MAX_PLAYER_SPEED, MAX_SHOVEL_CAPACITY, PLAYER_COLOR, PLAYER_HEIGHT, PLAYER_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH}, util::{c_to_tl, distance, flatten_index, flatten_index_standard_grid}, world_generation::does_gravity_apply_to_entity};

pub fn generate_player_image() -> Image{
    let mut data_buffer: Vec<u8> = vec![255; 4 * PLAYER_WIDTH * PLAYER_HEIGHT];
    for y in 0..PLAYER_HEIGHT {
        for x in 0..PLAYER_WIDTH {
            if y < 15 {
                data_buffer[flatten_index_standard_grid(&x, &y, PLAYER_WIDTH) * 4] = PLAYER_COLOR[0];
                data_buffer[flatten_index_standard_grid(&x, &y, PLAYER_WIDTH) * 4 + 1] = PLAYER_COLOR[1];
                data_buffer[flatten_index_standard_grid(&x, &y, PLAYER_WIDTH) * 4 + 2] = PLAYER_COLOR[2];
            } else {
                data_buffer[flatten_index_standard_grid(&x, &y, PLAYER_WIDTH) * 4] = 0;
                data_buffer[flatten_index_standard_grid(&x, &y, PLAYER_WIDTH) * 4 + 1] = 0;
                data_buffer[flatten_index_standard_grid(&x, &y, PLAYER_WIDTH) * 4 + 2] = 0;
            }
        }
    }
    for i in 0..2{
        data_buffer[flatten_index_standard_grid(&(2 + i), &5, PLAYER_WIDTH) * 4] = 255;
        data_buffer[flatten_index_standard_grid(&(2 + i), &5, PLAYER_WIDTH) * 4 + 1] = 255;
        data_buffer[flatten_index_standard_grid(&(2 + i), &5, PLAYER_WIDTH) * 4 + 2] = 255;
        data_buffer[flatten_index_standard_grid(&(PLAYER_WIDTH - 2 - i), &5, PLAYER_WIDTH) * 4] = 255;
        data_buffer[flatten_index_standard_grid(&(PLAYER_WIDTH - 2 - i), &5, PLAYER_WIDTH) * 4 + 1] = 255;
        data_buffer[flatten_index_standard_grid(&(PLAYER_WIDTH - 2 - i), &5, PLAYER_WIDTH) * 4 + 2] = 255;
    }
    for i in 0..PLAYER_WIDTH - 4{
        data_buffer[flatten_index_standard_grid(&(2 + i), &10, PLAYER_WIDTH) * 4] = 255;
        data_buffer[flatten_index_standard_grid(&(2 + i), &10, PLAYER_WIDTH) * 4 + 1] = 0;
        data_buffer[flatten_index_standard_grid(&(2 + i), &10, PLAYER_WIDTH) * 4 + 2] = 0;
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

pub fn move_player(
    mut grid_query: Query<&mut Grid, (With<TerrainGridTag>, Without<CursorTag>)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Velocity), (With<PlayerTag>, Without<CursorTag>)>,
    time: Res<Time>
){
    let grid = grid_query.get_single_mut().unwrap();
    let mut player = player_query.get_single_mut().unwrap();
    let does_gravity_apply = does_gravity_apply_to_entity(player.0.translation.x as i32 - PLAYER_WIDTH as i32/2,  player.0.translation.y as i32, PLAYER_WIDTH as i32, PLAYER_HEIGHT as i32, &grid.data);
    if does_gravity_apply {
        player.1.vy -= 1. * time.delta_seconds();
    } else {
        player.1.vy = 0.;
    }
    if keys.pressed(KeyCode::KeyA) {
        if player.1.vx * -1. < MAX_PLAYER_SPEED {
            player.1.vx -= 1. * time.delta_seconds();
        }
    } else if keys.pressed(KeyCode::KeyD) {
         if player.1.vx < MAX_PLAYER_SPEED {
            player.1.vx += 1. * time.delta_seconds();
        }
    }
    if keys.pressed(KeyCode::Space){
        if !does_gravity_apply{
            player.1.vy += 150. * time.delta_seconds();
        }
    }
    apply_velocity(&mut player.0.translation, &mut player.1, &grid.data);
}

fn apply_velocity(entity_position_c: &mut Vec3, velocity: &mut Mut<Velocity>, grid: &Vec<Pixel>) {
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
        for y in 0..PLAYER_HEIGHT{
            let index = flatten_index_standard_grid(&(entity_position_tl.0 as usize - 1), &(y as usize + entity_position_tl.1 as usize), WINDOW_WIDTH);
            if grid[index] == Pixel::Ground{
                return true
            }
        }
    } else if velocity > &0.{
        for y in 0..PLAYER_HEIGHT{
            let index = flatten_index_standard_grid(&(entity_position_tl.0 as usize + PLAYER_WIDTH + 1), &(y as usize + entity_position_tl.1 as usize), WINDOW_WIDTH);
            if grid[index] == Pixel::Ground{
                return true
            }
        }
    }
    false
}

pub fn update_cursor(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<(&mut Transform, &mut Velocity), (With<PlayerTag>, Without<CursorTag>)>,
    mut cursor_query: Query<&mut Transform, (With<CursorTag>, Without<PlayerTag>)>,
){
    let player = player_query.get_single_mut().unwrap();
    let mut cursor_position = cursor_query.get_single_mut().unwrap();
    if let Some(position) = q_windows.single().cursor_position() {
        let converted_position_x = position.x - WINDOW_WIDTH as f32 / 2.;
        let converted_position_y = (position.y - WINDOW_HEIGHT as f32 / 2.) * -1.;
        let angle = (converted_position_y - player.0.translation.y).atan2(converted_position_x - player.0.translation.x);
        let distance_from_player = distance(player.0.translation.x as i32, player.0.translation.y as i32, converted_position_x as i32, converted_position_y as i32);
        let min_distance = min(CURSOR_ORBITAL_RADIUS as usize, distance_from_player as usize) as f32;
        cursor_position.translation.x = player.0.translation.x + min_distance * angle.cos();
        cursor_position.translation.y = player.0.translation.y + min_distance * angle.sin();
    }
}

pub fn check_mouse_click(
    buttons: Res<ButtonInput<MouseButton>>,
    mut cursor_query: Query<&mut Transform, (With<CursorTag>, Without<PlayerTag>)>,
    mut grid_query: Query<&mut Grid, (With<TerrainGridTag>, Without<CursorTag>)>,
    mut cursor_content_count_query: Query<&mut Count, With<CursorTag>>,
    mut shovel_grid_query: Query<&mut Grid, (With<CursorTag>, Without<TerrainGridTag>)>,
    mut gravity_columns_query: Query<&mut TerrainPositionsAffectedByGravity>,
){
    let mut cursor_content_count = cursor_content_count_query.get_single_mut().unwrap();
    let mut grid = grid_query.get_single_mut().unwrap();
    let cursor_position = cursor_query.get_single_mut().unwrap();
    let mut shovel_grid = shovel_grid_query.get_single_mut().unwrap();
    let mut gravity_columns = gravity_columns_query.get_single_mut().unwrap();
    if buttons.just_pressed(MouseButton::Middle) {
    }
    if buttons.just_pressed(MouseButton::Right) {
        for y in 0..CURSOR_RADIUS * 2 {
            for x in 0..CURSOR_RADIUS * 2 {
                let shovel_grid_index = flatten_index_standard_grid(&x, &y, CURSOR_RADIUS * 2);
                if shovel_grid.data[shovel_grid_index] == Pixel::Ground {
                    let main_grid_index = flatten_index(cursor_position.translation.x as i32 - CURSOR_RADIUS as i32 + x as i32, cursor_position.translation.y as i32 - CURSOR_RADIUS as i32 + (CURSOR_RADIUS * 2 - y - 1) as i32);
                    if grid.data[main_grid_index] == Pixel::Sky {
                        grid.data[main_grid_index] = Pixel::Ground;
                        cursor_content_count.count -= 1;
                        gravity_columns.positions.insert(main_grid_index % WINDOW_WIDTH);
                    }
                }
            }
        }
        update_shovel_content_visual(&mut shovel_grid.data, cursor_content_count.count);
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
                    match grid.data[index] {
                        Pixel::Ground => {
                            cursor_content_count.count += 1;
                            grid.data[index] = Pixel::Sky;
                            gravity_columns.positions.insert(index % WINDOW_WIDTH);
                            if cursor_content_count.count == MAX_SHOVEL_CAPACITY {
                                update_shovel_content_visual(&mut shovel_grid.data, cursor_content_count.count);
                                return
                            }
                        },
                        _ => {

                        }
                    }
                }
            }
        }
        update_shovel_content_visual(&mut shovel_grid.data, cursor_content_count.count);
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
