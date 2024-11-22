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
use crate::color_map::{dirt_variant_pmf, COPPER, DIRT1, DIRT2, DIRT3, GRAVEL1, GRAVEL2, GRAVEL3, GRAVITY_AFFECTED, GROUND, LIGHT, REFINED_COPPER, ROCK, SELL_BOX, SILVER, SKY};
use crate::components::{ChunkMap, Count, GravityCoords, MoneyTextTag, RelativePosition, SunTick, TerrainImageTag, TimerComponent};
use crate::constants::{CHUNKS_HORIZONTAL, CHUNKS_VERTICAL, CHUNK_SIZE, COPPER_SPAWN_RADIUS, GLOBAL_MAX_X, GLOBAL_MAX_Y, GLOBAL_MIN_X, GLOBAL_MIN_Y, MAX_COPPER_ORE_SPAWNS, MAX_DIRT_HEIGHT_G, MAX_ROCK_HEIGHT_G, RENDER_SIZE, SELL_BOX_HEIGHT, SELL_BOX_SPAWN_X, SELL_BOX_SPAWN_Y, SELL_BOX_WIDTH};
use crate::drill::DrillTag;
use crate::util::{chunk_index_x_to_world_grid_index_shift, chunk_index_y_to_world_grid_index_shift, distance, flatten_index_standard_grid, get_chunk_x_g, get_chunk_x_v, get_chunk_y_g, get_chunk_y_v, get_global_y_coordinate, get_local_x, get_local_y, global_to_chunk_index_and_local_index, grid_to_image};

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
    let mut dirt_perlin_values = [0.; CHUNKS_HORIZONTAL as usize * CHUNK_SIZE as usize];
    let mut rock_perlin_values = [0.; CHUNKS_HORIZONTAL as usize * CHUNK_SIZE as usize];
    let dirt_noise_smoothness = 0.003;
    let rock_noise_smoothness = 0.004;
    let dirt_variation = 15.;
    let rock_variation = 80.;
    for x in 0..dirt_perlin_values.len() {
        dirt_perlin_values[x] = MAX_DIRT_HEIGHT_G + perlin.get([x as f64 * dirt_noise_smoothness, 0.0]) * dirt_variation;
        rock_perlin_values[x] = MAX_ROCK_HEIGHT_G + perlin.get([x as f64 * rock_noise_smoothness, 0.0]) * rock_variation;
    }
    for y in 0..CHUNKS_VERTICAL as usize {
        let world_chunk_y = chunk_index_y_to_world_grid_index_shift(y);
        for x in 0..CHUNKS_HORIZONTAL as usize {
            let world_chunk_x = chunk_index_x_to_world_grid_index_shift(x);
            chunk_map.push(generate_chunk(world_chunk_x, world_chunk_y, &dirt_perlin_values, &rock_perlin_values));
        }
    }
    commands.spawn(GravityCoords { coords: HashSet::new() });
    let mut pos = Vec3 { x: SELL_BOX_SPAWN_X as f32, y: SELL_BOX_SPAWN_Y as f32, z: 1. } ;
    while does_gravity_apply_to_entity(pos, SELL_BOX_WIDTH as i32, SELL_BOX_HEIGHT as i32, &chunk_map) {
        pos.y -= 1.;
    }
    add_sell_box_to_grid(&mut chunk_map, &pos);
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
                        .insert(RelativePosition { pos: (x, y) });
        }
    }
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

fn generate_chunk(chunk_x_g: i32, chunk_y_g: i32, dirt_perlin_values: &[f64], rock_perlin_values: &[f64]) -> Vec<u8> {
    let mut grid = vec![0; (CHUNK_SIZE * CHUNK_SIZE) as usize];
    let x_tl = (chunk_x_g + CHUNKS_HORIZONTAL as i32/2) as usize * CHUNK_SIZE as usize;
    for x in 0..CHUNK_SIZE as usize {
        let x_g_tl = x_tl + x;
        for y in 0..CHUNK_SIZE as usize {
            let global_y = get_global_y_coordinate(chunk_y_g, y);
            let index = y * CHUNK_SIZE as usize + x;
            if global_y > dirt_perlin_values[x_g_tl] as i32 {
                grid[index] = SKY;
                continue;
            }
            if global_y > rock_perlin_values[x_g_tl] as i32 {
                grid[index] = dirt_variant_pmf();
            } else {
                grid[index] = ROCK;
            }
        }
    }
    grid
}


pub fn grid_tick(
    time: Res<Time>,
    mut gravity_tick_timer_query: Query<&mut TimerComponent, With<TerrainImageTag>>,
    mut gravity_coords_query: Query<&mut GravityCoords>,
    mut money_count_query: Query<&mut Count>,
    mut chunk_map_query: Query<&mut ChunkMap>,
) {
    let mut gravity_tick_timer = gravity_tick_timer_query.get_single_mut().unwrap();
    gravity_tick_timer.timer.tick(time.delta());
    let mut chunk_map = chunk_map_query.get_single_mut().unwrap();
    if gravity_tick_timer.timer.finished() {
        let mut money_count = money_count_query.get_single_mut().unwrap();
        let mut gravity_coords = gravity_coords_query.get_single_mut().unwrap();
        gravity_tick(&mut gravity_coords.coords, &mut chunk_map.map, &mut money_count.count);
    }
}

pub fn does_gravity_apply_to_entity(entity_pos_g: Vec3, entity_width: i32, entity_height: i32, chunk_map: &Vec<Vec<u8>>) -> bool {
    for x in (entity_pos_g.x - entity_width as f32/2.) as i32..(entity_pos_g.x + entity_width as f32/2.) as i32 {
        let (chunk_index, local_index) = global_to_chunk_index_and_local_index(x, entity_pos_g.y as i32 - entity_height/2);
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

fn gravity_tick(gravity_coords: &mut HashSet<(i32, i32)>, chunk_map: &mut Vec<Vec<u8>>, money_count: &mut f32) {
    let mut new_coords = HashSet::new();
    for (x, y) in gravity_coords.iter() {
        let (chunk_index, local_index) = global_to_chunk_index_and_local_index(*x, *y);
        if GRAVITY_AFFECTED.contains(&chunk_map[chunk_index][local_index]) {
            let (below_chunk_index, below_local_index) = global_to_chunk_index_and_local_index(*x, *y-1);
            if chunk_map[below_chunk_index][below_local_index] == SKY || chunk_map[below_chunk_index][below_local_index] == LIGHT { 
                let mut looking_at_y = y - 1;
                new_coords.insert((*x, looking_at_y));
                loop {
                    let (below_chunk_index, below_local_index) = global_to_chunk_index_and_local_index(*x, looking_at_y);
                    let (above_chunk_index, above_local_index) = global_to_chunk_index_and_local_index(*x, looking_at_y + 1);
                    if chunk_map[above_chunk_index][above_local_index] == SKY || chunk_map[above_chunk_index][above_local_index] == REFINED_COPPER || chunk_map[above_chunk_index][above_local_index] == ROCK || chunk_map[above_chunk_index][above_local_index] == LIGHT {
                        break;
                    }
                    chunk_map[below_chunk_index][below_local_index] = chunk_map[above_chunk_index][above_local_index].clone();
                    chunk_map[above_chunk_index][above_local_index] = SKY;
                    looking_at_y += 1;
                }
            } else if chunk_map[below_chunk_index][below_local_index] == SELL_BOX {
                let mut looking_at_y = y - 1;
                new_coords.insert((*x, looking_at_y));
                loop {
                    let (above_chunk_index, above_local_index) = global_to_chunk_index_and_local_index(*x, looking_at_y + 1);
                    if chunk_map[above_chunk_index][above_local_index] == SKY || chunk_map[above_chunk_index][above_local_index] == REFINED_COPPER {
                        break;
                    }
                    match chunk_map[above_chunk_index][above_local_index] {
                        COPPER => *money_count += 0.5,
                        DIRT1 => *money_count += 0.01,
                        DIRT2 => *money_count += 0.01,
                        DIRT3 => *money_count += 0.01,
                        GRAVEL1 => *money_count += 0.01,
                        GRAVEL2 => *money_count += 0.01,
                        GRAVEL3 => *money_count += 0.01,
                        SILVER => *money_count += 1.0,
                        _ => {}
                    }
                    chunk_map[above_chunk_index][above_local_index] = SKY;
                    looking_at_y += 1;
                }
            }
        }
    };
    *gravity_coords = new_coords;
}

fn add_sell_box_to_grid(chunk_map: &mut Vec<Vec<u8>>, pos: &Vec3) {
    for y in pos.y as i32 - SELL_BOX_HEIGHT as i32/2..pos.y as i32 + SELL_BOX_HEIGHT as i32/2 {
        for x in pos.x as i32 - SELL_BOX_WIDTH as i32/2..pos.x as i32 + SELL_BOX_WIDTH as i32/2 {
            if x < pos.x as i32 + SELL_BOX_WIDTH as i32/2 - 1 - 2 && y > pos.y as i32 - SELL_BOX_HEIGHT as i32/2 + 2 && x > pos.x as i32 - SELL_BOX_WIDTH as i32/2 + 2 {
                let (chunk_index, local_index) = global_to_chunk_index_and_local_index(x, y);
                chunk_map[chunk_index][local_index] = SELL_BOX;
            } else {
                let (chunk_index, local_index) = global_to_chunk_index_and_local_index(x, y);
                chunk_map[chunk_index][local_index] = REFINED_COPPER;
            }
        }
    }
}

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

pub fn spawn_ore(mut chunk_map_query: Query<&mut ChunkMap>) {
    let mut rng = rand::thread_rng();
    let mut rng2 = rand::thread_rng();
    let mut chunk_map = chunk_map_query.get_single_mut().unwrap();
    let copper_random_multiplier = rng.gen_range(0.8..1.2);
    let num_copper_seeds = CHUNKS_HORIZONTAL * CHUNKS_VERTICAL * CHUNK_SIZE * CHUNK_SIZE * 0.000007 * copper_random_multiplier;
    for _ in 0..num_copper_seeds as i32 {
        let mut x = rng.gen_range(GLOBAL_MIN_X..GLOBAL_MAX_X);
        let mut y = rng.gen_range(GLOBAL_MIN_Y..GLOBAL_MAX_Y);
        let (mut chunk_index, mut local_index) = global_to_chunk_index_and_local_index(x, y);
        while !GROUND.contains(&chunk_map.map[chunk_index][local_index]) || rng.gen::<f32>() > (y.abs() as f32 / GLOBAL_MAX_Y as f32).min(0.11) {
            x = rng.gen_range(GLOBAL_MIN_X..GLOBAL_MAX_X);
            y = rng.gen_range(GLOBAL_MIN_Y..GLOBAL_MAX_Y);
            (chunk_index, local_index) = global_to_chunk_index_and_local_index(x, y);
        }
        grow_ore_seed(&mut rng, x, y, COPPER, &mut chunk_map.map, rng2.gen_range(30..80), rng2.gen_range(30..80), 0.1);
    }
    let silver_random_multiplier = rng.gen_range(0.8..1.2);
    let num_silver_seeds = CHUNKS_HORIZONTAL * CHUNKS_VERTICAL * CHUNK_SIZE * CHUNK_SIZE * 0.000003 * silver_random_multiplier;
    for _ in 0..num_silver_seeds as i32 {
        let mut x = rng.gen_range(GLOBAL_MIN_X..GLOBAL_MAX_X);
        let mut y = rng.gen_range(GLOBAL_MIN_Y..GLOBAL_MAX_Y);
        let (mut chunk_index, mut local_index) = global_to_chunk_index_and_local_index(x, y);
        while !GROUND.contains(&chunk_map.map[chunk_index][local_index]) || rng.gen::<f32>() > (y.abs() as f32 / GLOBAL_MAX_Y as f32).min(0.1) {
            x = rng.gen_range(GLOBAL_MIN_X..GLOBAL_MAX_X);
            y = rng.gen_range(GLOBAL_MIN_Y..GLOBAL_MAX_Y);
            (chunk_index, local_index) = global_to_chunk_index_and_local_index(x, y);
        }
        grow_ore_seed(&mut rng, x, y, SILVER, &mut chunk_map.map, rng2.gen_range(10..40), rng2.gen_range(10..40), 0.2);
    }
}

fn grow_ore_seed(rng: &mut ThreadRng, seed_x: i32, seed_y: i32, seed_type: u8, chunk_map: &mut Vec<Vec<u8>>, radius_x: i32, radius_y: i32, density: f32) {
    for x in seed_x - radius_x..seed_x + radius_x {
        if x < GLOBAL_MIN_X || x >= GLOBAL_MAX_X {
            continue;
        }
        for y in seed_y - radius_y..seed_y + radius_y {
            if y < GLOBAL_MIN_Y || y >= GLOBAL_MAX_Y {
                continue;
            }
            if (x - seed_x) as f32 * (x - seed_x) as f32 / (radius_x * radius_x) as f32 + (y - seed_y) as f32 * (y - seed_y) as f32 / (radius_y * radius_y) as f32 <= 1. {
                if rng.gen::<f32>() < density {
                    let (chunk_index, local_index) = global_to_chunk_index_and_local_index(x, y);
                    if GROUND.contains(&chunk_map[chunk_index][local_index]) {
                        chunk_map[chunk_index][local_index] = seed_type;
                    }
                }
            }
        }
    }
}