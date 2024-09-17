use std::cmp::min;

use bevy::{asset::AssetServer, math::Vec3, prelude::{default, Commands, Component, Query, Res, Transform, Visibility, With, Without}, sprite::SpriteBundle, window::{PrimaryWindow, Window}};
use rand::Rng;

use crate::{components::{Bool, ContentList, GravityCoords, Grid, ImageBuffer, Pixel, PixelType, PlayerTag, TerrainGridTag, Velocity}, constants::{CURSOR_BORDER_WIDTH, CURSOR_ORBITAL_RADIUS, CURSOR_RADIUS, HOE_HEIGHT, HOE_WIDTH, MAX_SHOVEL_CAPACITY, PLAYER_SPAWN_X, PLAYER_SPAWN_Y, WINDOW_HEIGHT, WINDOW_WIDTH}, util::{distance, flatten_index, flatten_index_standard_grid, grid_to_image}};

#[derive(Component)]
pub struct HoeTag;

#[derive(PartialEq)]
pub enum Tool{
    Shovel,
    Pickaxe,
    Hoe,
}

#[derive(Component)]
pub struct PickaxeTag;

#[derive(Component)]
pub struct ShovelTag;

#[derive(Component, PartialEq)]
pub struct CurrentTool{
    pub tool: Tool
}

pub fn spawn_tools(mut commands: Commands, assets: Res<AssetServer>) {
    let shovel_grid = generate_shovel_grid();
    let pickaxe_grid = generate_pickaxe_grid();
    let hoe_grid = generate_hoe_grid();
    let shovel_image = grid_to_image(&shovel_grid, CURSOR_RADIUS as u32 * 2, CURSOR_RADIUS as u32 * 2, None);
    let pickaxe_image = grid_to_image(&pickaxe_grid, CURSOR_RADIUS as u32 * 2, CURSOR_RADIUS as u32 * 2, None);
    let hoe_image = grid_to_image(&hoe_grid, HOE_WIDTH as u32, HOE_HEIGHT as u32, None);
    commands.spawn(HoeTag)
            .insert(SpriteBundle {
                texture: assets.add(grid_to_image(&hoe_grid.clone(), HOE_WIDTH as u32, HOE_HEIGHT as u32, None)),
                transform: Transform { translation: Vec3 { x: PLAYER_SPAWN_X as f32, y: PLAYER_SPAWN_Y as f32, z: 1. }, ..default()},
                visibility: Visibility::Hidden,
                ..default()})
            .insert(Grid { data: hoe_grid })
            .insert(ImageBuffer { data: hoe_image.data })
            .insert(Bool { bool: false });
    commands.spawn(ShovelTag)
        .insert(SpriteBundle {
            texture: assets.add(grid_to_image(&shovel_grid.clone(), CURSOR_RADIUS as u32 * 2, CURSOR_RADIUS as u32 * 2, None)),
            transform: Transform { translation: Vec3 { x: PLAYER_SPAWN_X as f32, y: PLAYER_SPAWN_Y as f32, z: 1. }, ..default()},
            ..default()})
        .insert(ContentList{ contents: Vec::new() })
        .insert(Grid { data: shovel_grid })
        .insert(ImageBuffer{ data: shovel_image.data });
    commands.spawn(PickaxeTag)
            .insert(SpriteBundle {
                texture: assets.add(grid_to_image(&pickaxe_grid.clone(), CURSOR_RADIUS as u32 * 2, CURSOR_RADIUS as u32 * 2, None)),
                transform: Transform { translation: Vec3 { x: PLAYER_SPAWN_X as f32, y: PLAYER_SPAWN_Y as f32, z: 1. }, ..default()},
                visibility: Visibility::Hidden,
                ..default()})
            .insert(Grid { data: pickaxe_grid })
            .insert(ImageBuffer { data: pickaxe_image.data });
}

fn generate_shovel_grid() -> Vec<Pixel>{
    let mut data_buffer: Vec<Pixel> = Vec::with_capacity(CURSOR_RADIUS * 2 * CURSOR_RADIUS * 2);
    for y in 0..CURSOR_RADIUS * 2 {
        for x in 0..CURSOR_RADIUS * 2 {
            let distance = distance(x as i32, y as i32, CURSOR_RADIUS as i32, CURSOR_RADIUS as i32);
            if distance > CURSOR_RADIUS as f32 {
                data_buffer.push(Pixel { pixel_type: PixelType::Clear, gamma: 1. });
            } else if distance < CURSOR_RADIUS as f32 - CURSOR_BORDER_WIDTH {
                data_buffer.push(Pixel { pixel_type: PixelType::TranslucentGrey, gamma: 1. });
            } else {
                data_buffer.push(Pixel { pixel_type: PixelType::White, gamma: 1. });
            }
        }
    }
    data_buffer
}

fn generate_pickaxe_grid() -> Vec<Pixel> {
    let mut data_buffer: Vec<Pixel> = Vec::with_capacity(CURSOR_RADIUS * 2 * CURSOR_RADIUS * 2);
    for y in 0..CURSOR_RADIUS * 2 {
        for x in 0..CURSOR_RADIUS * 2 {
            let distance = distance(x as i32, y as i32, CURSOR_RADIUS as i32, CURSOR_RADIUS as i32);
            if distance > CURSOR_RADIUS as f32 {
                data_buffer.push(Pixel { pixel_type: PixelType::Clear, gamma: 1. });
            } else if distance < CURSOR_RADIUS as f32 - CURSOR_BORDER_WIDTH {
                data_buffer.push(Pixel { pixel_type: PixelType::TranslucentGrey, gamma: 1. });
            } else {
                data_buffer.push(Pixel { pixel_type: PixelType::Red, gamma: 1. });
            }
        }
    }
    data_buffer
}

fn generate_hoe_grid() -> Vec<Pixel> {
    let mut data_buffer: Vec<Pixel> = Vec::with_capacity(HOE_WIDTH * HOE_HEIGHT);
    for _ in 0..HOE_HEIGHT {
        for _ in 0..HOE_WIDTH {
            data_buffer.push(Pixel { pixel_type: PixelType::Steel, gamma: 1. });
        }
    }
    data_buffer
}

pub fn update_tool(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<(&mut Transform, &mut Velocity), (With<PlayerTag>, Without<ShovelTag>)>,
    mut shovel_query: Query<&mut Transform, (With<ShovelTag>, (Without<PlayerTag>, Without<PickaxeTag>))>,
    mut pickaxe_query: Query<&mut Transform, (With<PickaxeTag>, (Without<PlayerTag>, Without<ShovelTag>))>,
    mut hoe_query: Query<&mut Transform, (With<HoeTag>, (Without<PlayerTag>, Without<ShovelTag>, Without<PickaxeTag>))>,
    current_tool_query: Query<&CurrentTool>,
    grid_query: Query<&Grid<Pixel>, With<TerrainGridTag>>,
    is_hoe_locked_query: Query<&Bool, With<HoeTag>>,
) {
    let player = player_query.get_single_mut().unwrap();
    let current_tool = current_tool_query.get_single().unwrap();
    let mut tool_position;
    let grid = grid_query.get_single().unwrap();
    let hoe_is_locked = is_hoe_locked_query.get_single().unwrap();
    match current_tool.tool {
        Tool::Shovel => {
            tool_position = shovel_query.get_single_mut().unwrap();
        },
        Tool::Pickaxe => {
            tool_position = pickaxe_query.get_single_mut().unwrap();
        },
        Tool::Hoe => {
            tool_position = hoe_query.get_single_mut().unwrap();
        }
    }
    if let Some(position) = q_windows.single().cursor_position() {
        let converted_position_x = position.x - WINDOW_WIDTH as f32 / 2.;
        let converted_position_y = (position.y - WINDOW_HEIGHT as f32 / 2.) * -1.;
        let angle = (converted_position_y - player.0.translation.y).atan2(converted_position_x - player.0.translation.x);
        let distance_from_player = distance(player.0.translation.x as i32, player.0.translation.y as i32, converted_position_x as i32, converted_position_y as i32);
        let min_distance = min(CURSOR_ORBITAL_RADIUS as usize, distance_from_player as usize) as f32;
        let mut potential_x = player.0.translation.x + min_distance * angle.cos();
        let mut potential_y = player.0.translation.y + min_distance * angle.sin();
        let mut dy = potential_y - player.0.translation.y;
        let mut dx = potential_x - player.0.translation.x;
        if dy.abs() < dx.abs() {
            dy /= dx;
            dx = 1.;
        } else {
            dx /= dy;
            dy = 1.;
        }
        dx = -dx.abs() * (potential_x - player.0.translation.x).signum();
        dy = -dy.abs() * (potential_y - player.0.translation.y).signum();
        while grid.data[flatten_index(potential_x as i32, potential_y as i32)].pixel_type != PixelType::Sky && grid.data[flatten_index(potential_x as i32, potential_y as i32)].pixel_type != PixelType::Light {
            potential_x += dx as f32;
            potential_y += dy as f32;
        }
        if !hoe_is_locked.bool {
            tool_position.translation.y = potential_y;
            tool_position.translation.x = potential_x;
        } else {
            if tool_position.translation.x < potential_x {
                for y in (tool_position.translation.y as i32 - HOE_HEIGHT as i32/2..tool_position.translation.y as i32 + HOE_HEIGHT as i32/2).rev() {
                    let index = flatten_index(tool_position.translation.x as i32 + HOE_WIDTH as i32/2 + 1, y);
                    println!("{:?}", grid.data[index].pixel_type);
                } 
            }
        }
        
    }
}

pub fn update_shovel_content_visual(shovel_image_grid: &mut Vec<Pixel>, shovel_contents: &Vec<Pixel>) {
    for pixel in shovel_image_grid.iter_mut() {
        if matches!(pixel.pixel_type, PixelType::Ground(_) | PixelType::Gravel(_) | PixelType::Chalcopyrite) {
            pixel.pixel_type = PixelType::TranslucentGrey;
        }
    }
    let mut drawn_content = 0;
    for pixel in shovel_image_grid.iter_mut().rev() {
        if drawn_content == shovel_contents.len() {
            return
        }
        if matches!(pixel.pixel_type, PixelType::TranslucentGrey) {
            let pixel_type = &shovel_contents[drawn_content].pixel_type;
            pixel.pixel_type = pixel_type.clone();
            drawn_content += 1;
        }
    }
}

pub fn right_click_shovel(shovel_grid: &mut Vec<Pixel>, terrain_grid: &mut Vec<Pixel>, cursor_position: &Transform, cursor_contents: &mut Vec<Pixel>, gravity_coords: &mut GravityCoords){
    for y in 0..CURSOR_RADIUS * 2 {
        for x in 0..CURSOR_RADIUS * 2 {
            if cursor_contents.len() == 0{
                update_shovel_content_visual(shovel_grid, cursor_contents);
                return
            }
            let shovel_grid_index = flatten_index_standard_grid(&x, &y, CURSOR_RADIUS * 2);
            if matches!(shovel_grid[shovel_grid_index].pixel_type, PixelType::Ground(_) | PixelType::Gravel(_) | PixelType::Chalcopyrite) {
                let main_grid_index = flatten_index(cursor_position.translation.x as i32 - CURSOR_RADIUS as i32 + x as i32, cursor_position.translation.y as i32 - CURSOR_RADIUS as i32 + (CURSOR_RADIUS * 2 - y - 1) as i32);
                if matches!(terrain_grid[main_grid_index].pixel_type, PixelType::Sky | PixelType::Light) {
                    let pixel = cursor_contents.pop().unwrap();
                    terrain_grid[main_grid_index] = pixel;
                    gravity_coords.coords.insert((main_grid_index % WINDOW_WIDTH, main_grid_index / WINDOW_WIDTH));
                }
            }
        }
    }
    update_shovel_content_visual(shovel_grid, cursor_contents);
}

pub fn left_click_shovel(shovel_position: &Transform, shovel_contents: &mut Vec<Pixel>, grid: &mut Vec<Pixel>, gravity_coords: &mut GravityCoords, shovel_grid: &mut Vec<Pixel>) {
    let left = shovel_position.translation.x as i32 - CURSOR_RADIUS as i32;
    let right = shovel_position.translation.x as i32 + CURSOR_RADIUS as i32;
    let top = shovel_position.translation.y as i32 + CURSOR_RADIUS as i32; 
    let bottom = shovel_position.translation.y as i32 - CURSOR_RADIUS as i32;
    let mut min_x = WINDOW_WIDTH + 1;
    let mut max_x = 0;
    let starting_count = shovel_contents.len();
    for y in bottom..top {
        for x in left..right {
            if distance(x, y, shovel_position.translation.x as i32, shovel_position.translation.y as i32) < CURSOR_RADIUS as f32 - CURSOR_BORDER_WIDTH {
                let index = flatten_index(x as i32, y as i32);
                if let PixelType::Ground(dirt_variant) = grid[index].pixel_type.clone() {
                    shovel_contents.push(Pixel { pixel_type: PixelType::Ground(dirt_variant), gamma: 1.});
                    grid[index] = Pixel { pixel_type: PixelType::Sky, gamma: 1.};
                    if let Some(y) = search_upward_for_non_sky_pixel(grid, index % WINDOW_WIDTH, index / WINDOW_WIDTH){
                        gravity_coords.coords.insert((index % WINDOW_WIDTH, y));
                    }
                    if index % WINDOW_WIDTH < min_x {
                        min_x = index % WINDOW_WIDTH;
                    } else if index % WINDOW_WIDTH > max_x {
                        max_x = index % WINDOW_WIDTH;
                    }
                    if shovel_contents.len() == MAX_SHOVEL_CAPACITY {
                        update_shovel_content_visual(shovel_grid, shovel_contents);
                        return
                    }
                } else if let PixelType::Gravel(gravel_variant) = grid[index].pixel_type.clone() {
                    shovel_contents.push(Pixel { pixel_type: PixelType::Gravel(gravel_variant), gamma: 1.});
                    grid[index] = Pixel { pixel_type: PixelType::Sky, gamma: 1.};
                    if let Some(y) = search_upward_for_non_sky_pixel(grid, index % WINDOW_WIDTH, index / WINDOW_WIDTH){
                        gravity_coords.coords.insert((index % WINDOW_WIDTH, y));
                    }
                    if index % WINDOW_WIDTH < min_x {
                        min_x = index % WINDOW_WIDTH;
                    } else if index % WINDOW_WIDTH > max_x {
                        max_x = index % WINDOW_WIDTH;
                    }
                    if shovel_contents.len() == MAX_SHOVEL_CAPACITY {
                        update_shovel_content_visual(shovel_grid, shovel_contents);
                        return
                    }
                } else if let PixelType::Chalcopyrite = grid[index].pixel_type {
                    shovel_contents.push(Pixel { pixel_type: PixelType::Chalcopyrite, gamma: 1.});
                    grid[index] = Pixel { pixel_type: PixelType::Sky, gamma: 1.};
                    gravity_coords.coords.insert((index % WINDOW_WIDTH, index / WINDOW_WIDTH));
                    if index % WINDOW_WIDTH < min_x {
                        min_x = index % WINDOW_WIDTH;
                    } else if index % WINDOW_WIDTH > max_x {
                        max_x = index % WINDOW_WIDTH;
                    }
                    if shovel_contents.len() == MAX_SHOVEL_CAPACITY {
                        update_shovel_content_visual(shovel_grid, shovel_contents);
                        return
                    }
                }
            }
        }
    }
    if starting_count != shovel_contents.len() {
        update_shovel_content_visual(shovel_grid, shovel_contents);
    }
}

pub fn left_click_pickaxe(pickaxe_position: &Transform, grid: &mut Vec<Pixel>, gravity_coords: &mut GravityCoords) {
    let left = pickaxe_position.translation.x as i32 - CURSOR_RADIUS as i32;
    let right = pickaxe_position.translation.x as i32 + CURSOR_RADIUS as i32;
    let top = pickaxe_position.translation.y as i32 + CURSOR_RADIUS as i32; 
    let bottom = pickaxe_position.translation.y as i32 - CURSOR_RADIUS as i32;
    let rng = &mut rand::thread_rng();
    for y in bottom..top{
        for x in left..right{
            if distance(x, y, pickaxe_position.translation.x as i32, pickaxe_position.translation.y as i32) < CURSOR_RADIUS as f32 - CURSOR_BORDER_WIDTH {
                let index = flatten_index(x as i32, y as i32);
                if matches!(grid[index].pixel_type, PixelType::Rock) {
                    grid[index] = Pixel { pixel_type: PixelType::Gravel(rng.gen()), gamma: 1.};
                    gravity_coords.coords.insert((index % WINDOW_WIDTH, index / WINDOW_WIDTH));
                }
            }
        }
    }
}

fn search_upward_for_non_sky_pixel(grid: &Vec<Pixel>, x: usize, y: usize) -> Option<usize> {
    let mut y_level = 1;
    loop {
        if y - y_level == 0{
            return None
        }
        if !matches!(grid[flatten_index_standard_grid(&x, &(y - y_level), WINDOW_WIDTH)].pixel_type, PixelType::Sky) {
            return Some(y - y_level)
        }
        y_level += 1;
    }
}

pub fn left_click_hoe(hoe_position: &mut Transform, grid: &mut Vec<Pixel>, is_locked: &mut bool) {
    for x in (hoe_position.translation.x - HOE_WIDTH as f32 /2.) as i32 .. (hoe_position.translation.x + HOE_WIDTH as f32 / 2.) as i32 {
        for y in (hoe_position.translation.y - HOE_HEIGHT as f32 / 2.) as i32 .. (hoe_position.translation.y + HOE_HEIGHT as f32 / 2.) as i32{
            let index = flatten_index(x as i32, y as i32);
            if !matches!(grid[index].pixel_type, PixelType::Sky) {
                return;
            }
        }
    }
    for _ in 0..10 {
        for x in (hoe_position.translation.x - HOE_WIDTH as f32 /2.) as i32 .. (hoe_position.translation.x + HOE_WIDTH as f32 / 2.) as i32 {
            let index = flatten_index(x as i32, (hoe_position.translation.y - HOE_HEIGHT as f32 / 2.) as i32 - 1);
            if !matches!(grid[index].pixel_type, PixelType::Sky) {
                *is_locked = true;
                return;
            }
            hoe_position.translation.y -= 1.;
        }
    }
    *is_locked = true;
}

pub fn right_click_hoe(is_locked: &mut bool) {
    *is_locked = false;
}