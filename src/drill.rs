use bevy::{asset::{AssetServer, Assets, Handle}, math::Vec3, prelude::{default, Commands, Component, Image, Query, Res, ResMut, Transform, With}, sprite::SpriteBundle, time::Time};

use crate::{color_map::{gravel_variant_pmf, COPPER, DRILL_BLACK, DRILL_GREY, GRAVITY_AFFECTED, ROCK, SKY}, components::{ContentList, GravityCoords, TerrainImageTag, TimerComponent, USize}, constants::CHUNK_SIZE, util::{flatten_index, flatten_index_standard_grid, get_chunk_x_g, get_local_x, get_local_y}, world_generation::GridMaterial};

pub const DRILL_SCALE: f32 = 2.;
pub const DRILL_WIDTH: f32 = 21. * DRILL_SCALE;
pub const DRILL_HEIGHT: f32 = 24. * DRILL_SCALE;
const DRILL_PIPE_OFFSET: i32 = 1;
const DRILL_BUFFER_SIZE: usize = 20;
const DRILL_OUTPUT_OFFSET_Y: i32 = 16;
const DRILL_OUTPUT_OFFSET_X: i32 = -21;

#[derive(Component)]
pub struct DrillTag;

pub fn spawn_drill(mut commands: Commands, asset_server: Res<AssetServer>, mut position_g: Vec3, chunk_map: &Vec<Vec<u8>>) {
    // 'outer: loop {
    //     for x_g in position_g.x as i32 - DRILL_WIDTH as i32 / 2 .. position_g.x as i32 + DRILL_WIDTH as i32 / 2 {
    //         let chunk_x_g = get_chunk_x_g(x_g as f32);
    //         let chunk_y_g = get_chunk_x_g(position_g.y - DRILL_HEIGHT / 2.);
    //         let chunk_x_v = get_chunk_x_v(chunk_x_g);
    //         let chunk_y_v = get_chunk_x_v(chunk_y_g);
    //         let chunk_index = flatten_index_standard_grid(&chunk_x_v, &chunk_y_v, CHUNKS_HORIZONTAL as usize);
    //         let local_x = get_local_x(x_g);
    //         let local_y = get_local_y((position_g.y - DRILL_HEIGHT / 2.) as i32);
    //         let local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
    //         if chunk_map[chunk_index][local_index] != SKY {
    //             break 'outer;
    //         }
    //     }
    //     position_g.y -= 1.;
    // }
    // commands.spawn(SpriteBundle { 
    //     texture: asset_server.load("sprites/drill_sprite.png"), 
    //     transform: Transform {
    //         translation: position_g, 
    //         scale: Vec3::new(DRILL_SCALE, DRILL_SCALE, 1.),
    //         ..Default::default()
    //     },
    //     ..default() 
    // }).insert(DrillTag)
    // .insert(USize { usize: 0 })
    // .insert(ContentList { contents: Vec::new() }); 
}   

pub fn drill_tick(
    mut drill_query: Query<(&Transform, &mut USize, &mut ContentList), With<DrillTag>>,
    mut drill_tick_query: Query<&mut TimerComponent, With<DrillTag>>,
    mut images: ResMut<Assets<Image>>,
    time: Res<Time>,
    mut materials: ResMut<Assets<GridMaterial>>,
    terrain_material_handle: Query<&Handle<GridMaterial>, With<TerrainImageTag>>,
    mut gravity_coords_query: Query<&mut GravityCoords>,
) {
    let mut drill_tick = drill_tick_query.get_single_mut().unwrap();
    drill_tick.timer.tick(time.delta());
    let handle = terrain_material_handle.get_single().unwrap();
    let material = materials.get_mut(handle).unwrap();
    let terrain_grid = &mut images.get_mut(&material.color_map).unwrap().data;
    if drill_tick.timer.finished() {
        let mut gravity_coords = gravity_coords_query.get_single_mut().unwrap();
        'outer: for (drill_transform, mut drill_depth, mut contents) in drill_query.iter_mut() {
            let y = drill_transform.translation.y as i32 - DRILL_HEIGHT as i32/2 - drill_depth.usize as i32;
            if y <= (CHUNK_SIZE as i32 / 2 - 1) * -1 {
                continue;
            }
            let mut dug_count = 0;
            for x in 0..3 * DRILL_SCALE as i32 {
                let index = flatten_index(drill_transform.translation.x as i32 + x + DRILL_PIPE_OFFSET, y);
                if DRILL_BUFFER_SIZE > contents.contents.len() {
                    if GRAVITY_AFFECTED.contains(&terrain_grid[index]) || terrain_grid[index] == COPPER {
                        contents.contents.push(terrain_grid[index]);
                        terrain_grid[index] = SKY;
                        dug_count += 1;
                    } else if terrain_grid[index] == ROCK {
                        terrain_grid[index] = gravel_variant_pmf();
                        dug_count += 1;
                    } else if terrain_grid[index] == SKY {
                    } else if terrain_grid[index] == 19 || terrain_grid[index] == 20 {
                    } else {
                        panic!("drill hit something unhandeled: {}", terrain_grid[index]);
                    }
                }
            }
            flush_buffer(&mut contents.contents, terrain_grid, drill_transform, &mut gravity_coords);
            if dug_count == 0 && contents.contents.len() == 0 {
                for x in 0..3 * DRILL_SCALE as i32 {
                    let index = flatten_index(drill_transform.translation.x as i32 + x + DRILL_PIPE_OFFSET, y - 1);
                    if terrain_grid[index] == !SKY {
                        continue 'outer;
                    }
                }
                if drill_transform.translation.y as i32 - DRILL_HEIGHT as i32/2 - drill_depth.usize as i32 - 2 > -1 * CHUNK_SIZE as i32 / 2 {
                    drill_depth.usize += 1;
                }
                for x in 0..DRILL_SCALE as i32 {
                    let index = flatten_index(drill_transform.translation.x as i32 + x + DRILL_PIPE_OFFSET, y);
                    terrain_grid[index] = DRILL_BLACK;
                }
                for x in DRILL_SCALE as i32 ..2 * DRILL_SCALE as i32 {
                    let index = flatten_index(drill_transform.translation.x as i32 + x + DRILL_PIPE_OFFSET, y);
                    terrain_grid[index] = DRILL_GREY;
                }
                for x in 2 * DRILL_SCALE as i32 ..3 * DRILL_SCALE as i32 {
                    let index = flatten_index(drill_transform.translation.x as i32 + x + DRILL_PIPE_OFFSET, y);
                    terrain_grid[index] = DRILL_BLACK;
                }
            }
        }
    }
}

fn flush_buffer(contents: &mut Vec<u8>, terrain_grid: &mut Vec<u8>, drill_transform_c: &Transform, gravity_coords: &mut GravityCoords) {
    for x in 0..3 * DRILL_SCALE as i32 {
        let index = flatten_index(drill_transform_c.translation.x as i32 + x + DRILL_OUTPUT_OFFSET_X, drill_transform_c.translation.y as i32 + DRILL_OUTPUT_OFFSET_Y);
        if terrain_grid[index] == SKY {
            if contents.len() != 0 {
                terrain_grid[index] = contents.remove(0);
                // gravity_coords.coords.insert((index % CHUNK_SIZE as usize, index / CHUNK_SIZE as usize));
            }
        }
    }
}