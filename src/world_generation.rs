use std::cmp::{max, min};
use std::collections::HashSet;
use std::time::Duration;

use bevy::asset::{Asset, Assets, Handle};
use bevy::color::palettes::css::GOLD;
use bevy::math::Vec2;
use bevy::prelude::{Component, Image, Mesh, Query, Rectangle, ResMut, TextBundle, With};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, MaterialMesh2dBundle};
use bevy::text::{Text, TextSection, TextStyle};
use bevy::time::{Time, Timer, TimerMode};
use bevy_reflect::TypePath;
use iyes_perf_ui::entries::PerfUiBundle;
use bevy::utils::default;

use bevy::{asset::AssetServer, core_pipeline::core_2d::Camera2dBundle, ecs::system::{Commands, Res}, math::Vec3, transform::components::Transform};
use noise::{NoiseFn, Perlin};
use rand::rngs::ThreadRng;
use rand::Rng;
use crate::color_map::{dirt_variant_pmf, COPPER, DIRT1, DIRT2, DIRT3, GRAVEL1, GRAVEL2, GRAVEL3, LIGHT, REFINED_COPPER, ROCK, SELL_BOX, SKY};
use crate::components::{ChunkMap, Count, GravityCoords, MoneyTextTag, RelativePosition, SunTick, TerrainImageTag, TimerComponent};
use crate::constants::{CHUNKS_HORIZONTAL, CHUNKS_VERTICAL, CHUNK_SIZE, COPPER_SPAWN_RADIUS, GLOBAL_MAX_X, GLOBAL_MAX_Y, GLOBAL_MIN_X, GLOBAL_MIN_Y, GROUND_HEIGHT, MAX_COPPER_ORE_SPAWNS, MAX_DIRT_HEIGHT_G, MAX_ROCK_HEIGHT_G, RENDER_SIZE, ROCK_HEIGHT, SELL_BOX_HEIGHT, SELL_BOX_WIDTH, SKY_HEIGHT};
use crate::drill::DrillTag;
use crate::util::{chunk_index_x_y_to_world_grid_index_shift, distance, flatten_index_standard_grid, get_chunk_x_g, get_chunk_x_v, get_global_x_coordinate, get_global_y_coordinate, get_local_x, get_local_y, grid_to_image};

#[derive(Component)]
pub struct CameraTag;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(PerfUiBundle::default());
    commands.spawn(Camera2dBundle::default()).insert(CameraTag);
}

pub fn setup_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<GridMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();
    let perlin = Perlin::new(rng.gen());
    let mut chunk_map: Vec<Vec<u8>> = Vec::with_capacity(CHUNKS_VERTICAL as usize * CHUNKS_HORIZONTAL as usize);
    for y in 0..CHUNKS_VERTICAL as usize {
        for x in 0..CHUNKS_HORIZONTAL as usize {
            let (world_chunk_x, world_chunk_y) = chunk_index_x_y_to_world_grid_index_shift(x, y);
            chunk_map.push(generate_chunk(&perlin, world_chunk_x, world_chunk_y, &mut rng));
        }
    }
    commands.spawn(ChunkMap { map: chunk_map });
    for x in -1 * RENDER_SIZE / 2..=RENDER_SIZE / 2 {
        for y in -1 * RENDER_SIZE / 2..=RENDER_SIZE / 2 {
            commands.spawn(TerrainImageTag)
                        .insert(MaterialMesh2dBundle {
                            material: materials.add(GridMaterial {
                                color_map: images.add(grid_to_image(&vec![9; (CHUNK_SIZE * CHUNK_SIZE) as usize], CHUNK_SIZE as u32, CHUNK_SIZE as u32, None)),
                                size: Vec2::new(CHUNK_SIZE as f32, CHUNK_SIZE as f32),
                            }),
                            mesh: meshes
                            .add(Rectangle {
                                half_size: Vec2::new(CHUNK_SIZE/2., CHUNK_SIZE/2.),
                            })
                            .into(),
                            transform: Transform {
                                translation: Vec3::new(x as f32 * CHUNK_SIZE, y as f32 * CHUNK_SIZE, -5.),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(GravityCoords { coords: HashSet::new() })
                        .insert(RelativePosition { pos: (x, y) });
        }
    }
    
    // add_sell_box_to_grid(&mut color_map.data);
    commands.spawn(TimerComponent { timer: Timer::new(Duration::from_millis(7), TimerMode::Repeating) }).insert(TerrainImageTag);
    commands.spawn(TimerComponent { timer: Timer::new(Duration::from_millis(20), TimerMode::Repeating) }).insert(DrillTag);
    commands.spawn(SunTick { timer: Timer::new(Duration::from_millis(1000), TimerMode::Repeating) });
    commands.spawn(Count { count: 0. });
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "$0.00 ",
                TextStyle {
                    font: assets.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: GOLD.into(),
                    ..default()
                },
            ),
        ]),
    )).insert(MoneyTextTag);
}

fn generate_chunk(perlin: &Perlin, chunk_x_g: i32, chunk_y_g: i32, rng: &mut ThreadRng) -> Vec<u8> {
    let dirt_noise_smoothness = 0.003;
    let rock_noise_smoothness = 0.004;
    let dirt_variation = 15.;
    let rock_variation = 80.;
    let mut grid = vec![0; (CHUNK_SIZE * CHUNK_SIZE) as usize];
    for x in 0..CHUNK_SIZE as usize {
        let global_x = get_global_x_coordinate(chunk_x_g, x);
        let max_dirt_height_per_column_g = MAX_DIRT_HEIGHT_G + (perlin.get([global_x as f64 * dirt_noise_smoothness, 0.0]) * dirt_variation);
        let max_rock_height_per_column_c = MAX_ROCK_HEIGHT_G + perlin.get([global_x as f64 * rock_noise_smoothness, 0.0]) * rock_variation;
        for y in 0..CHUNK_SIZE as usize {
            let global_y = get_global_y_coordinate(chunk_y_g, y);
            let index = y * CHUNK_SIZE as usize + x;
            if global_y > max_dirt_height_per_column_g as i32 {
                grid[index] = SKY;
                continue;
            }
            if global_y > max_rock_height_per_column_c as i32 {
                grid[index] = dirt_variant_pmf(rng);
            } else {
                grid[index] = ROCK;
            }
        }
    }
    grid
}


pub fn grid_tick(
    mut materials: ResMut<Assets<GridMaterial>>,
    terrain_material_handle: Query<&Handle<GridMaterial>, With<TerrainImageTag>>,
    mut images: ResMut<Assets<Image>>,
    time: Res<Time>,
    mut gravity_tick_timer_query: Query<&mut TimerComponent, With<TerrainImageTag>>,
    mut gravity_coords_query: Query<&mut GravityCoords>,
    mut money_count_query: Query<&mut Count>,
) {
    let mut gravity_tick_timer = gravity_tick_timer_query.get_single_mut().unwrap();
    gravity_tick_timer.timer.tick(time.delta());
    let material_handle = terrain_material_handle.get_single().unwrap();
    let material = materials.get_mut(material_handle).unwrap();
    let terrain_grid = &mut images.get_mut(&material.color_map).unwrap().data;
    if gravity_tick_timer.timer.finished() {
        let mut money_count = money_count_query.get_single_mut().unwrap();
        let mut gravity_coords = gravity_coords_query.get_single_mut().unwrap();
        gravity_tick(&mut gravity_coords.coords, terrain_grid, &mut money_count.count);
    }
}

pub fn does_gravity_apply_to_entity(entity_pos_g: Vec3, entity_width: i32, entity_height: i32, chunk_map: &Vec<Vec<u8>>) -> bool {
    for x in (entity_pos_g.x - entity_width as f32/2.) as i32..(entity_pos_g.x + entity_width as f32/2.) as i32 {
        let chunk_x_g = get_chunk_x_g(entity_pos_g.x);
        let chunk_y_g = get_chunk_x_g(entity_pos_g.y);
        let chunk_x_v = get_chunk_x_v(chunk_x_g);
        let chunk_y_v = get_chunk_x_v(chunk_y_g);
        let chunk_index = flatten_index_standard_grid(&chunk_x_v, &chunk_y_v, CHUNKS_HORIZONTAL as usize);
        let local_x = get_local_x(x);
        let local_y = get_local_y(entity_pos_g.y as i32 - entity_height/2);
        let local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
        match &chunk_map[chunk_index][local_index]{
            &SKY => continue,
            &SELL_BOX => continue,
            &LIGHT => continue,
            _ => {
                return false
            }
        }
    }
    true
}

fn gravity_tick(gravity_coords: &mut HashSet<(i32, i32)>, grid: &mut Vec<u8>, money_count: &mut f32) {
    // let mut new_coords = HashSet::new();
    // for coord in gravity_coords.iter() {
    //     let index = flatten_index_standard_grid(&coord.0, &coord.1, CHUNK_SIZE as usize);
    //     if grid[index] == DIRT1 || grid[index] == DIRT2 || grid[index] == DIRT3 || grid[index] == GRAVEL1 || grid[index] == GRAVEL2 || grid[index] == GRAVEL3 || grid[index] == COPPER {
    //         let mut below_index = flatten_index_standard_grid(&coord.0, &(coord.1 + 1), CHUNK_SIZE as usize);
    //         if grid[below_index] == SKY || grid[below_index] == LIGHT { 
    //             let mut looking_at_y = coord.1 + 1;
    //             new_coords.insert((coord.0, looking_at_y));
    //             loop {
    //                 below_index = flatten_index_standard_grid(&coord.0, &looking_at_y, CHUNK_SIZE as usize);
    //                 let above_index = flatten_index_standard_grid(&coord.0, &(looking_at_y - 1), CHUNK_SIZE as usize);
    //                 if grid[above_index] == SKY || grid[above_index] == REFINED_COPPER ||  grid[above_index] == ROCK || grid[above_index] == LIGHT {
    //                     break;
    //                 }
    //                 grid[below_index] = grid[above_index].clone();
    //                 grid[above_index] = SKY;
    //                 looking_at_y -= 1;
    //             }
    //         } else if grid[below_index] == SELL_BOX {
    //             let mut looking_at_y = coord.1 + 1;
    //             new_coords.insert((coord.0, looking_at_y));
    //             loop {
    //                 let above_index = flatten_index_standard_grid(&coord.0, &(looking_at_y - 1), CHUNK_SIZE as usize);
    //                 if grid[above_index] == SKY || grid[above_index] == REFINED_COPPER {
    //                     break;
    //                 }
    //                 match grid[above_index] {
    //                     COPPER => *money_count += 0.5,
    //                     DIRT1 => *money_count += 0.01,
    //                     DIRT2 => *money_count += 0.01,
    //                     DIRT3 => *money_count += 0.01,
    //                     GRAVEL1 => *money_count += 0.01,
    //                     GRAVEL2 => *money_count += 0.01,
    //                     GRAVEL3 => *money_count += 0.01,
    //                     _ => {}
    //                 }
    //                 grid[above_index] = SKY;
    //                 looking_at_y -= 1;
    //             }
    //         }
    //     }
    // };
    // *gravity_coords = new_coords;
}

// fn add_sell_box_to_grid(grid: &mut Vec<u8>) {
//     for y in SKY_HEIGHT - SELL_BOX_HEIGHT..SKY_HEIGHT{
//         for x in 800..800+SELL_BOX_WIDTH{
//             let index = flatten_index_standard_grid(&x, &y, CHUNK_SIZE);
//             if x < 800 + SELL_BOX_WIDTH - 1 - 2 && y < SKY_HEIGHT - 1 - 2 && x > 800 + 2{
//                 grid[index] = SELL_BOX;
//             } else {
//                 grid[index] = REFINED_COPPER;
//             }
//         }
//     }
// }

pub fn update_money_text(
    mut money_text_query: Query<&mut Text, With<MoneyTextTag>>,
    mut money_count_query: Query<&Count>,
) {
    let money_count = money_count_query.get_single_mut().unwrap();
    let mut money_text = money_text_query.get_single_mut().unwrap();
    money_text.sections[0].value = format!("${:.2}", money_count.count);
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GridMaterial {
    #[texture(1)]
    pub color_map: Handle<Image>,
    #[uniform(0)]
    pub size: Vec2
}

impl Material2d for GridMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/render_shader.wgsl".into()
    }
}

pub fn spawn_copper_ore(
    chunk_query: Query<&Handle<GridMaterial>, With<TerrainImageTag>>,
    chunk_material_handles: Query<&Handle<GridMaterial>, With<TerrainImageTag>>, 
    mut materials: ResMut<Assets<GridMaterial>>,
    mut images: ResMut<Assets<Image>>,

) {
    let mut rng = rand::thread_rng();
    // let material_handle = chunk_querry.get_single().unwrap();
    // let material = materials.get_mut(material_handle).unwrap();
    // let grid = &mut images.get_mut(&material.color_map).unwrap().data;
    for chunk_handle in chunk_material_handles.iter() {
        let terrain_grid_material = materials.get_mut(chunk_handle).unwrap();
        let terrain_grid_image = images.get_mut(&terrain_grid_material.color_map).unwrap();
        let terrain_grid_data = &mut terrain_grid_image.data;
        
        // Modify the terrain_grid_data as needed
        // Your logic here
    }


    
    for _ in 0..(MAX_COPPER_ORE_SPAWNS as f32 * rng.gen::<f32>()) as u32  {
        let seed_x = rng.gen_range(GLOBAL_MIN_X..GLOBAL_MAX_X);
        let seed_y = rng.gen_range(GLOBAL_MIN_Y..GLOBAL_MAX_Y);
        for x in seed_x - COPPER_SPAWN_RADIUS/2..seed_x + COPPER_SPAWN_RADIUS/2 {
            for y in seed_y - COPPER_SPAWN_RADIUS/2..seed_y + COPPER_SPAWN_RADIUS/2 {
                let distance = distance(seed_x, seed_y, x, y);
                if distance < COPPER_SPAWN_RADIUS as f32 {
                    
                    // grid[index] = COPPER;
                }
            }
        }
    }


        // let index = flatten_index_standard_grid(&x, &y, CHUNK_SIZE);
        // grid[index] = COPPER;
        // for xx in max(0, x - CALCOPIRITE_RADIUS)..min(CHUNK_SIZE, x + CALCOPIRITE_RADIUS){
        //     for yy in max(0, y - CALCOPIRITE_RADIUS)..min(CHUNK_SIZE, y + CALCOPIRITE_RADIUS){
        //         let distance = distance(xx as i32, yy as i32, x as i32, y as i32);
        //         if distance < CALCOPIRITE_RADIUS as f32{
        //             if distance != 0. && rng.gen_range(0..distance as usize * 2) == 0 {
        //                 let index = flatten_index_standard_grid(&xx, &yy, CHUNK_SIZE);
        //                 grid[index] = COPPER;
        //             }
        //         }
        //     }
        // }
    // }
    // if SKY_HEIGHT + GROUND_HEIGHT < SKY_HEIGHT + GROUND_HEIGHT + ROCK_HEIGHT {
    //     for _ in 0..CHALCOPIRITE_SPAWN_COUNT {
    //         let x = rng.gen_range(0..CHUNK_SIZE);
    //         let y = rng.gen_range(SKY_HEIGHT + GROUND_HEIGHT..SKY_HEIGHT + GROUND_HEIGHT + ROCK_HEIGHT);
    //         let index = flatten_index_standard_grid(&x, &y, CHUNK_SIZE);
    //         grid[index] = COPPER;
    //         for xx in max(0, x - CALCOPIRITE_RADIUS)..min(CHUNK_SIZE, x + CALCOPIRITE_RADIUS){
    //             for yy in max(0, y - CALCOPIRITE_RADIUS)..min(CHUNK_SIZE, y + CALCOPIRITE_RADIUS){
    //                 let distance = distance(xx as i32, yy as i32, x as i32, y as i32);
    //                 if distance < CALCOPIRITE_RADIUS as f32{
    //                     if distance != 0. && rng.gen_range(0..distance as usize * 2) == 0 {
    //                         let index = flatten_index_standard_grid(&xx, &yy, CHUNK_SIZE);
    //                         grid[index] = COPPER;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}