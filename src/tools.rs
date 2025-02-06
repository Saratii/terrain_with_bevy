use std::collections::HashMap;

use bevy::{asset::Assets, ecs::event::EventWriter, math::Vec2, prelude::{Camera, Commands, Component, GlobalTransform, Image, Mesh, Query, Rectangle, ResMut, Transform, Visibility, With, Without}, sprite::MaterialMesh2dBundle, window::{PrimaryWindow, Window}};

use crate::{chunk_generator::NewChunkEvent, color_map::{apply_gamma_correction, gravel_variant_pmf, CLEAR, LIGHT, RAW_DECODER_DATA, RED, ROCK, SHOVEL_ABLE, SKY, STEEL, TRANSLUCENT_GREY, WHITE}, components::{Bool, ChunkMap, ContentList, GravityCoords, PlayerTag, Velocity}, constants::{CHUNK_SIZE, CURSOR_BORDER_WIDTH, CURSOR_ORBITAL_RADIUS, CURSOR_RADIUS, HOE_HEIGHT, HOE_WIDTH, MAX_SHOVEL_CAPACITY}, util::{distance, flatten_index, flatten_index_standard_grid, get_chunk_x_g, get_chunk_y_g, get_local_x, get_local_y, grid_to_image}, world_generation::CameraTag, GridMaterial};

#[derive(Component)]
pub struct HoeTag;

#[derive(PartialEq)]
pub enum Tool{
    Shovel,
    Pickaxe,
    Hoe,
    SpawnDrill,
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
                    decoder: apply_gamma_correction(RAW_DECODER_DATA),
                    color_map_of_above: images.add(grid_to_image(&vec![0 as u8; HOE_HEIGHT * HOE_WIDTH], HOE_WIDTH as u32, HOE_HEIGHT as u32, None)),

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
                    decoder: apply_gamma_correction(RAW_DECODER_DATA),
                    color_map_of_above: images.add(grid_to_image(&vec![0 as u8; CURSOR_RADIUS as usize * 2 * CURSOR_RADIUS as usize * 2], CURSOR_RADIUS as u32 * 2, CURSOR_RADIUS as u32 * 2, None)),

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
                    decoder: apply_gamma_correction(RAW_DECODER_DATA),
                    color_map_of_above: images.add(grid_to_image(&vec![0 as u8; CURSOR_RADIUS as usize * 2 * CURSOR_RADIUS as usize * 2], CURSOR_RADIUS as u32 * 2, CURSOR_RADIUS as u32 * 2, None)),
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
    mut chunk_map_query: Query<&mut ChunkMap>,
    q_camera: Query<(&Camera, &GlobalTransform), With<CameraTag>>,
) {
    let player = player_query.get_single_mut().unwrap();
    let current_tool = current_tool_query.get_single().unwrap();
    let mut tool_position;
    let chunk_map = chunk_map_query.get_single_mut().unwrap();
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
        },
        Tool::SpawnDrill => {
            return
        }
    }
    let (camera, camera_transform) = q_camera.single();
    if let Ok(window) = q_windows.get_single() {
        if let Some(cursor) = window.cursor_position() {
            if let Some(ray) = camera.viewport_to_world(camera_transform, cursor) {
                let position_c = ray.origin.truncate();
                let angle = (position_c.y - player.0.translation.y).atan2(position_c.x - player.0.translation.x);
                let mut potential_x = player.0.translation.x;
                let mut potential_y = player.0.translation.y;
                let dy = angle.sin();
                let dx = angle.cos();
                let mut local_x = get_local_x(potential_x as i32);
                let mut local_y = get_local_y(potential_y as i32);
                let mut local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
                let (mut chunk_x_g, mut chunk_y_g) = (get_chunk_x_g(potential_x as i32), get_chunk_y_g(potential_y as i32));
                while chunk_map.map.get(&(chunk_x_g, chunk_y_g)).unwrap()[local_index] == SKY || chunk_map.map.get(&(chunk_x_g, chunk_y_g)).unwrap()[local_index] == LIGHT {
                    potential_x += dx as f32;
                    potential_y += dy as f32;
                    let distance_from_player_to_potential = distance(player.0.translation.x as i32, player.0.translation.y as i32, potential_x as i32, potential_y as i32);
                    if distance_from_player_to_potential > CURSOR_ORBITAL_RADIUS {
                        break
                    }
                    if distance(potential_x as i32, potential_y as i32, position_c.x as i32, position_c.y as i32) < 2. {
                        break
                    }
                    local_x = get_local_x(potential_x as i32);
                    local_y = get_local_y(potential_y as i32);
                    local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
                    (chunk_x_g, chunk_y_g) = (get_chunk_x_g(potential_x as i32), get_chunk_y_g(potential_y as i32));
                    
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
    }
}

pub fn update_shovel_content_visual(shovel_image_grid: &mut Vec<u8>, shovel_contents: &Vec<u8>) {
    for pixel in shovel_image_grid.iter_mut() {
        if SHOVEL_ABLE.contains(pixel) {
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

pub fn right_click_shovel(shovel_grid: &mut Vec<u8>, chunk_map: &mut HashMap<(i32, i32), Vec<u8>>, cursor_position: &Transform, cursor_contents: &mut Vec<u8>, gravity_coords: &mut GravityCoords) {
    for y in 0..CURSOR_RADIUS * 2 {
        for x in 0..CURSOR_RADIUS * 2 {
            if cursor_contents.len() == 0 {
                update_shovel_content_visual(shovel_grid, cursor_contents);
                return
            }
            let shovel_grid_index = flatten_index_standard_grid(&x, &y, CURSOR_RADIUS * 2);
            if SHOVEL_ABLE.contains(&shovel_grid[shovel_grid_index]) {
                let (local_x, local_y) = (get_local_x(cursor_position.translation.x as i32 - CURSOR_RADIUS as i32 + x as i32), get_local_y(cursor_position.translation.y as i32 - CURSOR_RADIUS as i32 + (CURSOR_RADIUS * 2 - y - 1) as i32));
                let local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
                let (chunk_x_g, chunk_y_g) = (get_chunk_x_g(cursor_position.translation.x as i32 - CURSOR_RADIUS as i32 + x as i32), get_chunk_y_g(cursor_position.translation.y as i32 - CURSOR_RADIUS as i32 + (CURSOR_RADIUS * 2 - y - 1) as i32));
                if chunk_map.get(&(chunk_x_g, chunk_y_g)).unwrap()[local_index] == SKY || chunk_map.get(&(chunk_x_g, chunk_y_g)).unwrap()[local_index] == LIGHT {
                    let pixel = cursor_contents.pop().unwrap();
                    chunk_map.get_mut(&(chunk_x_g, chunk_y_g)).unwrap()[local_index] = pixel;
                    gravity_coords.coords.insert((cursor_position.translation.x as i32 - CURSOR_RADIUS as i32 + x as i32, cursor_position.translation.y as i32 - CURSOR_RADIUS as i32 + (CURSOR_RADIUS * 2 - y - 1) as i32));
                }
            }
        }
    }
    update_shovel_content_visual(shovel_grid, cursor_contents);
}

pub fn left_click_shovel(shovel_position: &Transform, shovel_contents: &mut Vec<u8>, chunk_map: &mut HashMap<(i32, i32), Vec<u8>>, shovel_grid: &mut Vec<u8>, gravity_coords: &mut GravityCoords, chunk_writer: &mut EventWriter<NewChunkEvent>) {
    let left = shovel_position.translation.x as i32 - CURSOR_RADIUS as i32;
    let right = shovel_position.translation.x as i32 + CURSOR_RADIUS as i32;
    let top = shovel_position.translation.y as i32 + CURSOR_RADIUS as i32; 
    let bottom = shovel_position.translation.y as i32 - CURSOR_RADIUS as i32;
    let starting_count = shovel_contents.len();
    for y in bottom..top {
        for x in left..right {
            if distance(x, y, shovel_position.translation.x as i32, shovel_position.translation.y as i32) < CURSOR_RADIUS as f32 - CURSOR_BORDER_WIDTH {
                let (local_x, local_y) = (get_local_x(x), get_local_y(y));
                let local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
                let (chunk_x_g, chunk_y_g) = (get_chunk_x_g(x), get_chunk_y_g(y));
                let comparing_pixel = chunk_map.get(&(chunk_x_g, chunk_y_g)).unwrap()[local_index];
                if SHOVEL_ABLE.contains(&comparing_pixel) {
                    shovel_contents.push(comparing_pixel);
                    chunk_map.get_mut(&(chunk_x_g, chunk_y_g)).unwrap()[local_index] = SKY;
                    if let Some(y) = search_upward_for_non_sky_pixel(chunk_map, x, y, chunk_writer) {
                        gravity_coords.coords.insert((x, y));
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

pub fn left_click_pickaxe(pickaxe_position: &Transform, chunk_map: &mut HashMap<(i32, i32), Vec<u8>>, gravity_coords: &mut GravityCoords) {
    let left = pickaxe_position.translation.x as i32 - CURSOR_RADIUS as i32;
    let right = pickaxe_position.translation.x as i32 + CURSOR_RADIUS as i32;
    let top = pickaxe_position.translation.y as i32 + CURSOR_RADIUS as i32; 
    let bottom = pickaxe_position.translation.y as i32 - CURSOR_RADIUS as i32;
    let mut gravel_variant_pmf = gravel_variant_pmf();
    for y_g in bottom..top{
        for x_g in left..right{
            if distance(x_g, y_g, pickaxe_position.translation.x as i32, pickaxe_position.translation.y as i32) < CURSOR_RADIUS as f32 - CURSOR_BORDER_WIDTH {
                let (local_x, local_y) = (get_local_x(x_g), get_local_y(y_g));
                let local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
                let (chunk_x_g, chunk_y_g) = (get_chunk_x_g(x_g), get_chunk_y_g(y_g));
                if chunk_map.get(&(chunk_x_g, chunk_y_g)).unwrap()[local_index] == ROCK {
                    chunk_map.get_mut(&(chunk_x_g, chunk_y_g)).unwrap()[local_index] = gravel_variant_pmf.next().unwrap();
                    gravity_coords.coords.insert((x_g, y_g));
                }
            }
        }
    }
}

fn search_upward_for_non_sky_pixel(chunk_map: &mut HashMap<(i32, i32), Vec<u8>>, x_g: i32, y_g: i32, chunk_event_writer: &mut EventWriter<NewChunkEvent>) -> Option<i32> {
    let mut y_level = 1;
    while y_g + y_level < y_g + CURSOR_ORBITAL_RADIUS as i32 * 2 {
        let local_x = get_local_x(x_g);
        let local_y = get_local_y(y_g + y_level);
        let local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
        let (chunk_x_g, chunk_y_g) = (x_g / CHUNK_SIZE as i32, (y_g + y_level) / CHUNK_SIZE as i32);
        if let Some(chunk) = chunk_map.get(&(chunk_x_g, chunk_y_g)) {
            if chunk[local_index] != SKY {
                return Some(y_g + y_level)
            }
        } else {
            match chunk_map.get_mut(&(chunk_x_g, chunk_y_g)) {
                Some(_) => {},
                None => {
                    chunk_event_writer.send(NewChunkEvent { chunk_x_g, chunk_y_g });
                }
            }
            if chunk_map.get(&(chunk_x_g, chunk_y_g)).unwrap()[local_index] != SKY {
                return Some(y_g + y_level)
            }
        }
        y_level += 1;
    }
    None
}

pub fn left_click_hoe(_hoe_position_c: &mut Transform, _grid: &mut HashMap<(i32, i32), Vec<u8>>, is_locked: &mut bool) {
    // for x in (hoe_position_c.translation.x - HOE_WIDTH as f32 /2.) as i32 .. (hoe_position_c.translation.x + HOE_WIDTH as f32 / 2.) as i32 {
    //     for y in (hoe_position_c.translation.y - HOE_HEIGHT as f32 / 2.) as i32 .. (hoe_position_c.translation.y + HOE_HEIGHT as f32 / 2.) as i32{
    //         let index = flatten_index(x as i32, y as i32);
    //         if grid[index] != SKY {
    //             return;
    //         }
    //     }
    // }
    // for _ in 0..10 {
    //     for x in (hoe_position_c.translation.x - HOE_WIDTH as f32 /2.) as i32 .. (hoe_position_c.translation.x + HOE_WIDTH as f32 / 2.) as i32 {
    //         let index = flatten_index(x as i32, (hoe_position_c.translation.y - HOE_HEIGHT as f32 / 2.) as i32 - 1);
    //         if grid[index] == SKY {
    //             *is_locked = true;
    //             return;
    //         }
    //         hoe_position_c.translation.y -= 1.;
    //     }
    // }
    *is_locked = true;
}

pub fn right_click_hoe(is_locked: &mut bool) {
    *is_locked = false;
}