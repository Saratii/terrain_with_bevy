use std::cmp::min;

use bevy::{asset::{Assets, Handle}, math::{Vec2, Vec3}, prelude::{Commands, Component, Image, Mesh, Query, Rectangle, ResMut, Transform, Visibility, With, Without}, sprite::MaterialMesh2dBundle, window::{PrimaryWindow, Window}};

use crate::{color_map::{gravel_variant_pmf, CLEAR, COPPER, DIRT1, DIRT2, DIRT3, GRAVEL1, GRAVEL2, GRAVEL3, LIGHT, RED, ROCK, SKY, STEEL, TRANSLUCENT_GREY, WHITE}, components::{Bool, ContentList, GravityCoords, PlayerTag, TerrainGridTag, Velocity}, constants::{CURSOR_BORDER_WIDTH, CURSOR_ORBITAL_RADIUS, CURSOR_RADIUS, HOE_HEIGHT, HOE_WIDTH, MAX_SHOVEL_CAPACITY, WINDOW_HEIGHT, WINDOW_WIDTH}, util::{c_to_tl, distance, flatten_index, flatten_index_standard_grid, grid_to_image}, world_generation::GridMaterial};

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

pub fn spawn_tools(
    mut commands: Commands,
    mut materials: ResMut<Assets<GridMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let shovel_grid = generate_shovel_grid();
    let pickaxe_grid = generate_pickaxe_grid();
    let hoe_grid = generate_hoe_grid();
    let shovel_image = grid_to_image(&shovel_grid, CURSOR_RADIUS as u32 * 2, CURSOR_RADIUS as u32 * 2, None);
    let pickaxe_image = grid_to_image(&pickaxe_grid, CURSOR_RADIUS as u32 * 2, CURSOR_RADIUS as u32 * 2, None);
    let hoe_image = grid_to_image(&hoe_grid, HOE_WIDTH as u32, HOE_HEIGHT as u32, None);
    commands.spawn(HoeTag)
            .insert(MaterialMesh2dBundle {
                material: materials.add(GridMaterial {
                    color_map: images.add(hoe_image),
                    size: Vec2::new(HOE_WIDTH as f32, HOE_HEIGHT as f32),
                }),
                mesh: meshes
                .add(Rectangle {
                    half_size: Vec2::new((HOE_WIDTH/2) as f32, (HOE_HEIGHT/2) as f32),
                })
                .into(),
                visibility: Visibility::Hidden,
                ..Default::default()
            })
            .insert(Bool { bool: false });
    commands.spawn(ShovelTag)
            .insert(MaterialMesh2dBundle {
                material: materials.add(GridMaterial {
                    color_map: images.add(shovel_image),
                    size: Vec2::new((CURSOR_RADIUS * 2) as f32, (CURSOR_RADIUS * 2) as f32),
                }),
                mesh: meshes
                .add(Rectangle {
                    half_size: Vec2::new((CURSOR_RADIUS) as f32, (CURSOR_RADIUS) as f32),
                })
                .into(),
                ..Default::default()
            })
            .insert(ContentList { contents: Vec::new() });
    commands.spawn(PickaxeTag)
            .insert(MaterialMesh2dBundle {
                material: materials.add(GridMaterial {
                    color_map: images.add(pickaxe_image),
                    size: Vec2::new((CURSOR_RADIUS * 2) as f32, (CURSOR_RADIUS * 2) as f32),
                }),
                mesh: meshes
                .add(Rectangle {
                    half_size: Vec2::new((CURSOR_RADIUS) as f32, (CURSOR_RADIUS) as f32),
                })
                .into(),
                visibility: Visibility::Hidden,
                ..Default::default()
            });
}

fn generate_shovel_grid() -> Vec<u8>{
    let mut data_buffer: Vec<u8> = Vec::with_capacity(CURSOR_RADIUS * 2 * CURSOR_RADIUS * 2);
    for y in 0..CURSOR_RADIUS * 2 {
        for x in 0..CURSOR_RADIUS * 2 {
            let distance = distance(x as i32, y as i32, CURSOR_RADIUS as i32, CURSOR_RADIUS as i32);
            if distance > CURSOR_RADIUS as f32 {
                data_buffer.push(CLEAR);
            } else if distance < CURSOR_RADIUS as f32 - CURSOR_BORDER_WIDTH {
                data_buffer.push(TRANSLUCENT_GREY);
            } else {
                data_buffer.push(WHITE);
            }
        }
    }
    data_buffer
}

fn generate_pickaxe_grid() -> Vec<u8> {
    let mut data_buffer: Vec<u8> = Vec::with_capacity(CURSOR_RADIUS * 2 * CURSOR_RADIUS * 2);
    for y in 0..CURSOR_RADIUS * 2 {
        for x in 0..CURSOR_RADIUS * 2 {
            let distance = distance(x as i32, y as i32, CURSOR_RADIUS as i32, CURSOR_RADIUS as i32);
            if distance > CURSOR_RADIUS as f32 {
                data_buffer.push(CLEAR);
            } else if distance < CURSOR_RADIUS as f32 - CURSOR_BORDER_WIDTH {
                data_buffer.push(TRANSLUCENT_GREY);
            } else {
                data_buffer.push(RED);
            }
        }
    }
    data_buffer
}

fn generate_hoe_grid() -> Vec<u8> {
    let mut data_buffer: Vec<u8> = Vec::with_capacity(HOE_WIDTH * HOE_HEIGHT);
    for _ in 0..HOE_HEIGHT {
        for _ in 0..HOE_WIDTH {
            data_buffer.push(STEEL);
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
    is_hoe_locked_query: Query<&Bool, With<HoeTag>>,
    mut materials: ResMut<Assets<GridMaterial>>,
    terrain_material_handle: Query<&Handle<GridMaterial>, With<TerrainGridTag>>,
    mut images: ResMut<Assets<Image>>,
) {
    let player = player_query.get_single_mut().unwrap();
    let current_tool = current_tool_query.get_single().unwrap();
    let mut tool_position;
    let terrain_grid = &mut images.get_mut(&materials.get_mut(terrain_material_handle.get_single().unwrap()).unwrap().color_map).unwrap().data;
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
        while terrain_grid[flatten_index(potential_x as i32, potential_y as i32)] != SKY && terrain_grid[flatten_index(potential_x as i32, potential_y as i32)] != LIGHT {
            potential_x += dx as f32;
            potential_y += dy as f32;
        }
        if !hoe_is_locked.bool {
            tool_position.translation.y = potential_y;
            tool_position.translation.x = potential_x;
        } else {
            if tool_position.translation.x < potential_x {
                for y in (tool_position.translation.y as i32 - HOE_HEIGHT as i32/2..tool_position.translation.y as i32 + HOE_HEIGHT as i32/2).rev() {
                    let _index = flatten_index(tool_position.translation.x as i32 + HOE_WIDTH as i32/2 + 1, y);
                } 
            }
        }
    }
}

pub fn update_shovel_content_visual(shovel_image_grid: &mut Vec<u8>, shovel_contents: &Vec<u8>) {
    for pixel in shovel_image_grid.iter_mut() {
        if *pixel == DIRT1 || *pixel == DIRT2 || *pixel == DIRT3 || *pixel == GRAVEL1 || *pixel == GRAVEL2 || *pixel == GRAVEL3 || *pixel == COPPER {
            *pixel = TRANSLUCENT_GREY;
        }
    }
    let mut drawn_content = 0;
    for pixel in shovel_image_grid.iter_mut().rev() {
        if drawn_content == shovel_contents.len() {
            return
        }
        if *pixel == TRANSLUCENT_GREY {
            *pixel = shovel_contents[drawn_content].clone();
            drawn_content += 1;
        }
    }
}

pub fn right_click_shovel(shovel_grid: &mut Vec<u8>, terrain_grid: &mut Vec<u8>, cursor_position: &Transform, cursor_contents: &mut Vec<u8>, gravity_coords: &mut GravityCoords) {
    for y in 0..CURSOR_RADIUS * 2 {
        for x in 0..CURSOR_RADIUS * 2 {
            if cursor_contents.len() == 0{
                update_shovel_content_visual(shovel_grid, cursor_contents);
                return
            }
            let shovel_grid_index = flatten_index_standard_grid(&x, &y, CURSOR_RADIUS * 2);
            if shovel_grid[shovel_grid_index] == DIRT1 || shovel_grid[shovel_grid_index] == DIRT2 || shovel_grid[shovel_grid_index] == DIRT3 || shovel_grid[shovel_grid_index] == COPPER || shovel_grid[shovel_grid_index] == GRAVEL1 ||shovel_grid[shovel_grid_index] == GRAVEL2 || shovel_grid[shovel_grid_index] == GRAVEL3 {
                let main_grid_index = flatten_index(cursor_position.translation.x as i32 - CURSOR_RADIUS as i32 + x as i32, cursor_position.translation.y as i32 - CURSOR_RADIUS as i32 + (CURSOR_RADIUS * 2 - y - 1) as i32);
                if terrain_grid[main_grid_index] == SKY || terrain_grid[main_grid_index] == LIGHT {
                    let pixel = cursor_contents.pop().unwrap();
                    terrain_grid[main_grid_index] = pixel;
                    gravity_coords.coords.insert((main_grid_index % WINDOW_WIDTH, main_grid_index / WINDOW_WIDTH));
                }
            }
        }
    }
    update_shovel_content_visual(shovel_grid, cursor_contents);
}

pub fn left_click_shovel(shovel_position: &Transform, shovel_contents: &mut Vec<u8>, terrain_grid: &mut Vec<u8>, gravity_coords: &mut GravityCoords, shovel_grid: &mut Vec<u8>) {
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
                let pos_tl = c_to_tl(&Vec3::new(x as f32, y as f32, 0.), 1., 1.);
                let index = flatten_index_standard_grid(&(pos_tl.0 as usize), &(pos_tl.1 as usize), WINDOW_WIDTH);
                if terrain_grid[index] == DIRT1 || terrain_grid[index] == DIRT2 || terrain_grid[index] == DIRT3 || terrain_grid[index] == GRAVEL1 || terrain_grid[index] == GRAVEL2 || terrain_grid[index] == GRAVEL3 || terrain_grid[index] == COPPER{
                    shovel_contents.push(terrain_grid[index]);
                    terrain_grid[index] = SKY;
                    if let Some(y) = search_upward_for_non_sky_pixel(terrain_grid, pos_tl.0 as usize, pos_tl.1 as usize) {
                        gravity_coords.coords.insert((pos_tl.0 as usize, y));
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
                }
            }
        }
    }
    if starting_count != shovel_contents.len() {
        update_shovel_content_visual(shovel_grid, shovel_contents);
    }
}

pub fn left_click_pickaxe(pickaxe_position: &Transform, grid: &mut Vec<u8>, gravity_coords: &mut GravityCoords) {
    let left = pickaxe_position.translation.x as i32 - CURSOR_RADIUS as i32;
    let right = pickaxe_position.translation.x as i32 + CURSOR_RADIUS as i32;
    let top = pickaxe_position.translation.y as i32 + CURSOR_RADIUS as i32; 
    let bottom = pickaxe_position.translation.y as i32 - CURSOR_RADIUS as i32;
    for y in bottom..top{
        for x in left..right{
            if distance(x, y, pickaxe_position.translation.x as i32, pickaxe_position.translation.y as i32) < CURSOR_RADIUS as f32 - CURSOR_BORDER_WIDTH {
                let index = flatten_index(x as i32, y as i32);
                if grid[index] == ROCK {
                    grid[index] = gravel_variant_pmf();
                    gravity_coords.coords.insert((index % WINDOW_WIDTH, index / WINDOW_WIDTH));
                }
            }
        }
    }
}

fn search_upward_for_non_sky_pixel(terrain_grid: &Vec<u8>, x: usize, y: usize) -> Option<usize> {
    let mut y_level = 1;
    loop {
        if y - y_level == 0{
            return None
        }
        if terrain_grid[flatten_index_standard_grid(&x, &(y - y_level), WINDOW_WIDTH)] != SKY {
            return Some(y - y_level)
        }
        y_level += 1;
    }
}

pub fn left_click_hoe(hoe_position_c: &mut Transform, grid: &mut Vec<u8>, is_locked: &mut bool) {
    for x in (hoe_position_c.translation.x - HOE_WIDTH as f32 /2.) as i32 .. (hoe_position_c.translation.x + HOE_WIDTH as f32 / 2.) as i32 {
        for y in (hoe_position_c.translation.y - HOE_HEIGHT as f32 / 2.) as i32 .. (hoe_position_c.translation.y + HOE_HEIGHT as f32 / 2.) as i32{
            let index = flatten_index(x as i32, y as i32);
            if grid[index] != SKY {
                return;
            }
        }
    }
    for _ in 0..10 {
        for x in (hoe_position_c.translation.x - HOE_WIDTH as f32 /2.) as i32 .. (hoe_position_c.translation.x + HOE_WIDTH as f32 / 2.) as i32 {
            let index = flatten_index(x as i32, (hoe_position_c.translation.y - HOE_HEIGHT as f32 / 2.) as i32 - 1);
            if grid[index] == SKY {
                *is_locked = true;
                return;
            }
            hoe_position_c.translation.y -= 1.;
        }
    }
    *is_locked = true;
}

pub fn right_click_hoe(is_locked: &mut bool) {
    *is_locked = false;
}