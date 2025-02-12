use std::collections::{HashMap, HashSet};
use std::time::Duration;

use bevy::asset::Assets;
use bevy::color::palettes::css::GOLD;
use bevy::ecs::event::EventWriter;
use bevy::math::Vec2;
use bevy::prelude::{Mesh, Query, Rectangle, ResMut, TextBundle, With};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::text::{TextSection, TextStyle};
use bevy::time::{Time, Timer, TimerMode};
use iyes_perf_ui::entries::PerfUiBundle;
use bevy::utils::default;

use bevy::{asset::AssetServer, core_pipeline::core_2d::Camera2dBundle, ecs::system::{Commands, Res}, math::Vec3, transform::components::Transform};
use noise::Perlin;
use rand::rngs::ThreadRng;
use rand::Rng;
use crate::chunk_generator::NewChunkEvent;
use crate::color_map::{apply_gamma_correction, COPPER, DIRT1, DIRT2, DIRT3, GRAVEL1, GRAVEL2, GRAVEL3, GRAVITY_AFFECTED, LIGHT, RAW_DECODER_DATA, REFINED_COPPER, ROCK, SELL_BOX, SILVER, SKY};
use crate::compute_shader::ShadowsImages;
use crate::components::{CameraTag, ChunkMap, Count, GravityCoords, MoneyTextTag, PerlinHandle, SunTick, TerrainImageTag, TimerComponent};
use crate::constants::{CHUNK_SIZE, SELL_BOX_HEIGHT, SELL_BOX_SPAWN_X, SELL_BOX_SPAWN_Y, SELL_BOX_WIDTH, SPAWN_SELL_BOX};
use crate::sun::{ChunkImageHandle, GridMaterial, Othereers};
// use crate::drill::DrillTag;
use crate::util::{flatten_index_standard_grid, get_chunk_x_g, get_chunk_y_g, get_global_y_coordinate, get_local_x, get_local_y, local_to_global_x};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(PerfUiBundle::default());
    commands.spawn(Camera2dBundle::default()).insert(CameraTag);
}

pub fn setup_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<GridMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<AssetServer>,
    mut chunk_event_writer: EventWriter<NewChunkEvent>,
    center_image_handle: Res<ChunkImageHandle>,
    other_fuckers: Res<Othereers>,
    shadows_fucker: Res<ShadowsImages>,
) {
    let mut rng = rand::rng();
    let perlin = Perlin::new(rng.random());
    let mut chunk_map = HashMap::new();
    commands.spawn(PerlinHandle { handle: perlin.clone() });
    if SPAWN_SELL_BOX {
        commands.spawn(GravityCoords { coords: HashSet::new() });
        let mut pos = Vec3 { x: SELL_BOX_SPAWN_X as f32, y: SELL_BOX_SPAWN_Y as f32, z: 1. } ;
        while does_gravity_apply_to_entity(pos, SELL_BOX_WIDTH as i32, SELL_BOX_HEIGHT as i32, &mut chunk_map, &mut chunk_event_writer) {
            pos.y -= 1.;
        }
        add_sell_box_to_grid(&mut chunk_map, &pos);
    }
    commands.spawn(GravityCoords { coords: HashSet::new() });
    commands.spawn(ChunkMap { map: chunk_map });
    for i in 0..9 {
        let image_handle;
        if i == 4 {
            image_handle = center_image_handle.handle.clone();
        } else {
            image_handle = other_fuckers.handles.get(&(i as u32)).unwrap().clone();
        }
        commands.spawn(TerrainImageTag)
                    .insert(MaterialMesh2dBundle {
                        material: materials.add(GridMaterial {
                            color_map: image_handle,
                            size: Vec2::new(CHUNK_SIZE as f32, CHUNK_SIZE as f32),
                            decoder: apply_gamma_correction(RAW_DECODER_DATA),
                            shadow_map: Some(shadows_fucker.texture_b.clone()),
                            chunk_position: Vec2::new(0., 0.),
                        }),
                        mesh: meshes.add(Rectangle { half_size: Vec2::new(CHUNK_SIZE/2., CHUNK_SIZE/2.) }).into(),
                        transform: Transform { translation: Vec3::new(Default::default(), Default::default(), -5.), ..Default::default() },
                        ..Default::default()
                    });
        
    }
    commands.spawn(TimerComponent { timer: Timer::new(Duration::from_millis(7), TimerMode::Repeating) }).insert(TerrainImageTag);
    // commands.spawn(TimerComponent { timer: Timer::new(Duration::from_millis(20), TimerMode::Repeating) }).insert(DrillTag);
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

pub fn does_gravity_apply_to_entity(entity_pos_g: Vec3, entity_width: i32, entity_height: i32, chunk_map: &mut HashMap<(i32, i32), Vec<u8>>, chunk_event_writer: &mut EventWriter<NewChunkEvent>) -> bool {
    let mut sent_chunks = Vec::new();
    for x in (entity_pos_g.x - entity_width as f32/2.) as i32..(entity_pos_g.x + entity_width as f32/2.) as i32 {
        let local_x = get_local_x(x);
        let local_y = get_local_y(entity_pos_g.y as i32 - entity_height/2);
        let local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
        let (chunk_x_g, chunk_y_g) = (get_chunk_x_g(x), get_chunk_y_g(entity_pos_g.y as i32 - entity_height as i32 / 2));
        if let Some(_) = chunk_map.get(&(chunk_x_g, chunk_y_g)) {
            match &chunk_map.get(&(chunk_x_g, chunk_y_g)).unwrap()[local_index]{
                &SKY => continue,
                &SELL_BOX => continue,
                &LIGHT => continue,
                _ => {
                    return false
                }
            }
        } else {
            if !sent_chunks.contains(&(chunk_x_g, chunk_y_g)) {
                chunk_event_writer.send(NewChunkEvent { chunk_x_g, chunk_y_g } );
                sent_chunks.push((chunk_x_g, chunk_y_g));
            }
            // seed_chunk_with_ore((chunk_x_g, chunk_y_g), chunk_map);
            // match &chunk_map.get(&(chunk_x_g, chunk_y_g)).unwrap()[local_index]{
            //     &SKY => continue,
            //     &SELL_BOX => continue,
            //     &LIGHT => continue,
            //     _ => {
            //         return false
            //     }
            // }
        }
    }
    true
}

fn gravity_tick(
    gravity_coords: &mut HashSet<(i32, i32)>,
    chunk_map: &mut HashMap<(i32, i32), Vec<u8>>,
    money_count: &mut f32,
) {
    let mut new_coords = HashSet::new();
    for (x, y) in gravity_coords.iter() {
        let (local_x, local_y) = (get_local_x(*x), get_local_y(*y));
        let local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
        let (chunk_x, chunk_y) = (get_chunk_x_g(*x), get_chunk_y_g(*y));
        if GRAVITY_AFFECTED.contains(&chunk_map.get(&(chunk_x, chunk_y)).unwrap()[local_index]) {
            // Compute the initial below positions
            let (below_local_x, below_local_y) = (get_local_x(*x), get_local_y(*y - 1));
            let below_local_index = flatten_index_standard_grid(&below_local_x, &below_local_y, CHUNK_SIZE as usize);
            let (below_chunk_x, below_chunk_y) = (get_chunk_x_g(*x), get_chunk_y_g(*y - 1));
            if chunk_map.get(&(below_chunk_x, below_chunk_y)).unwrap()[below_local_index] == SKY {
                let mut looking_at_y = y - 1;
                new_coords.insert((*x, looking_at_y));
                loop {
                    // **Recompute below_chunk_x and below_chunk_y inside the loop**
                    let (below_chunk_x, below_chunk_y) = (get_chunk_x_g(*x), get_chunk_y_g(looking_at_y));
                    let (below_local_x, below_local_y) = (get_local_x(*x), get_local_y(looking_at_y));
                    let below_local_index = flatten_index_standard_grid(&below_local_x, &below_local_y, CHUNK_SIZE as usize);

                    let (above_chunk_x, above_chunk_y) = (get_chunk_x_g(*x), get_chunk_y_g(looking_at_y + 1));
                    let (above_local_x, above_local_y) = (get_local_x(*x), get_local_y(looking_at_y + 1));
                    let above_local_index = flatten_index_standard_grid(&above_local_x, &above_local_y, CHUNK_SIZE as usize);

                    if chunk_map.get(&(above_chunk_x, above_chunk_y)).unwrap()[above_local_index] == SKY
                        || chunk_map.get(&(above_chunk_x, above_chunk_y)).unwrap()[above_local_index] == REFINED_COPPER
                        || chunk_map.get(&(above_chunk_x, above_chunk_y)).unwrap()[above_local_index] == ROCK
                        || chunk_map.get(&(above_chunk_x, above_chunk_y)).unwrap()[above_local_index] == LIGHT
                    {
                        break;
                    }
                    chunk_map.get_mut(&(below_chunk_x, below_chunk_y)).unwrap()[below_local_index] =
                        chunk_map.get(&(above_chunk_x, above_chunk_y)).unwrap()[above_local_index].clone();
                    chunk_map.get_mut(&(above_chunk_x, above_chunk_y)).unwrap()[above_local_index] = SKY;
                    looking_at_y += 1;
                }
            } else if chunk_map.get(&(below_chunk_x, below_chunk_y)).unwrap()[below_local_index] == SELL_BOX {
                let mut looking_at_y = y - 1;
                new_coords.insert((*x, looking_at_y));
                loop {
                    let (above_chunk_x, above_chunk_y) = (get_chunk_x_g(*x), get_chunk_y_g(looking_at_y + 1));
                    let (above_local_x, above_local_y) = (get_local_x(*x), get_local_y(looking_at_y + 1));
                    let above_local_index = flatten_index_standard_grid(&above_local_x, &above_local_y, CHUNK_SIZE as usize);

                    if chunk_map.get(&(above_chunk_x, above_chunk_y)).unwrap()[above_local_index] == SKY
                        || chunk_map.get(&(above_chunk_x, above_chunk_y)).unwrap()[above_local_index] == REFINED_COPPER
                    {
                        break;
                    }
                    match chunk_map.get(&(above_chunk_x, above_chunk_y)).unwrap()[above_local_index] {
                        COPPER => *money_count += 0.5,
                        DIRT1 | DIRT2 | DIRT3 | GRAVEL1 | GRAVEL2 | GRAVEL3 => *money_count += 0.01,
                        SILVER => *money_count += 1.0,
                        _ => {}
                    }
                    chunk_map.get_mut(&(above_chunk_x, above_chunk_y)).unwrap()[above_local_index] = SKY;
                    looking_at_y += 1;
                }
            }
        }
    }
    *gravity_coords = new_coords;
}


fn add_sell_box_to_grid(chunk_map: &mut HashMap<(i32, i32), Vec<u8>>, pos: &Vec3) {
    for y in pos.y as i32 - SELL_BOX_HEIGHT as i32/2..pos.y as i32 + SELL_BOX_HEIGHT as i32/2 {
        for x in pos.x as i32 - SELL_BOX_WIDTH as i32/2..pos.x as i32 + SELL_BOX_WIDTH as i32/2 {
            let (chunk_x_g, chunk_y_g) = (get_chunk_x_g(x), get_chunk_y_g(y));
            if x < pos.x as i32 + SELL_BOX_WIDTH as i32/2 - 1 - 2 && y > pos.y as i32 - SELL_BOX_HEIGHT as i32/2 + 2 && x > pos.x as i32 - SELL_BOX_WIDTH as i32/2 + 2 {
                let local_x = get_local_x(x);
                let local_y = get_local_y(y);
                let local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
                chunk_map.get_mut(&(chunk_x_g, chunk_y_g)).unwrap()[local_index] = SELL_BOX;
            } else {
                let local_x = get_local_x(x);
                let local_y = get_local_y(y);
                let local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
                chunk_map.get_mut(&(chunk_x_g, chunk_y_g)).unwrap()[local_index] = REFINED_COPPER;
            }
        }
    }
}

// pub fn update_money_text(
//     mut money_text_query: Query<&mut Text, With<MoneyTextTag>>,
//     mut money_count_query: Query<&Count>,
// ) {
//     let money_count = money_count_query.get_single_mut().unwrap();
//     let mut money_text = money_text_query.get_single_mut().unwrap();
//     money_text.sections[0].value = format!("${:.2}", money_count.count);
// }


// impl Material2d for HeightMapTexture {
//     fn fragment_shader() -> ShaderRef {
//         "shaders/render_shader.wgsl".into()
//     }
// }

pub fn seed_chunk_with_ore(chunk_pos: (i32, i32), chunk_map: &mut HashMap<(i32, i32), Vec<u8>>) {
    let mut rng = rand::rng();
    let mut rng2 = rand::rng();
    // let mut chunk_map = chunk_map_query.get_single_mut().unwrap();
    // let copper_random_multiplier = rng.gen_range(0.8..1.2);
    // let num_copper_seeds = CHUNKS_HORIZONTAL * CHUNKS_VERTICAL * CHUNK_SIZE * CHUNK_SIZE * 0.000007 * copper_random_multiplier;
    // for _ in 0..num_copper_seeds as i32 {
    //     let mut x = rng.gen_range(GLOBAL_MIN_X..GLOBAL_MAX_X);
    //     let mut y = rng.gen_range(GLOBAL_MIN_Y..GLOBAL_MAX_Y);
    //     let (mut chunk_index, mut local_index) = global_to_chunk_index_and_local_index(x, y);
    //     let (chunk_x_g, chunk_y_g) = (get_chunk_x_g(x as f32), get_chunk_y_g(y as f32));
    //     while !GROUND.contains(&chunk_map.map.get(&(chunk_x_g, chunk_y_g)).unwrap()[local_index]) || rng.gen::<f32>() > (y.abs() as f32 / GLOBAL_MAX_Y as f32).min(0.11) {
    //         x = rng.gen_range(GLOBAL_MIN_X..GLOBAL_MAX_X);
    //         y = rng.gen_range(GLOBAL_MIN_Y..GLOBAL_MAX_Y);
    //         (chunk_index, local_index) = global_to_chunk_index_and_local_index(x, y);
    //     }
    //     grow_ore_seed(&mut rng, x, y, COPPER, &mut chunk_map.map, rng2.gen_range(30..80), rng2.gen_range(30..80), 0.1);
    // }
    // let silver_random_multiplier = rng.gen_range(0.8..1.2);
    // let num_silver_seeds = CHUNKS_HORIZONTAL * CHUNKS_VERTICAL * CHUNK_SIZE * CHUNK_SIZE * 0.000003 * silver_random_multiplier;
    // for _ in 0..num_silver_seeds as i32 {
    //     let mut x = rng.gen_range(GLOBAL_MIN_X..GLOBAL_MAX_X);
    //     let mut y = rng.gen_range(GLOBAL_MIN_Y..GLOBAL_MAX_Y);
    //     let (mut chunk_index, mut local_index) = global_to_chunk_index_and_local_index(x, y);
    //     let (chunk_x_g, chunk_y_g) = (get_chunk_x_g(x as f32), get_chunk_y_g(y as f32));
    //     while !GROUND.contains(&chunk_map.map.get(&(chunk_x_g, chunk_y_g)).unwrap()[local_index]) || rng.gen::<f32>() > (y.abs() as f32 / GLOBAL_MAX_Y as f32).min(0.1) {
    //         x = rng.gen_range(GLOBAL_MIN_X..GLOBAL_MAX_X);
    //         y = rng.gen_range(GLOBAL_MIN_Y..GLOBAL_MAX_Y);
    //         (chunk_index, local_index) = global_to_chunk_index_and_local_index(x, y);
    //     }
    //     grow_ore_seed(&mut rng, x, y, SILVER, &mut chunk_map.map, rng2.gen_range(10..40), rng2.gen_range(10..40), 0.2);
    // }
    let copper_range_end = (chunk_pos.1 * -1 + 1) / 2 as i32;
    if copper_range_end > 0 {
        for c in 0..rng.random_range(0..chunk_pos.1 * -1 + 1) {
            let x = rng.random_range(0..CHUNK_SIZE as i32);
            let y = rng.random_range(0..CHUNK_SIZE as i32);
            let x_g = local_to_global_x(chunk_pos.0, x as usize);
            let y_g = get_global_y_coordinate(chunk_pos.1, y as usize);
            grow_ore_seed(&mut rng2, x, y, COPPER, chunk_map, rng.random_range(30..80), rng.random_range(30..80), 0.1);
        }
    }
    let silver_range_end = (chunk_pos.1 * -1 + 1) / 2 as i32;
    if silver_range_end > 0 {
        for s in 0..rng.random_range(0..((chunk_pos.1 * -1 + 1) / 2 as i32)) {
            let x = rng.random_range(0..CHUNK_SIZE as i32);
            let y = rng.random_range(0..CHUNK_SIZE as i32);
            grow_ore_seed(&mut rng2, x, y, SILVER, chunk_map, rng.random_range(10..40), rng.random_range(10..40), 0.2);
        }
    }
}

fn grow_ore_seed(rng: &mut ThreadRng, seed_x_g: i32, seed_y_g: i32, seed_type: u8, chunk_map: &mut HashMap<(i32, i32), Vec<u8>>, radius_x: i32, radius_y: i32, density: f32) {
    // for x in seed_x_g - radius_x..seed_x_g + radius_x {
    //     for y in seed_y_g - radius_y..seed_y_g + radius_y {
    //         if (x - seed_x_g) as f32 * (x - seed_x_g) as f32 / (radius_x * radius_x) as f32 + (y - seed_y_g) as f32 * (y - seed_y_g) as f32 / (radius_y * radius_y) as f32 <= 1. {
    //             if rng.gen::<f32>() < density {
    //                 let local_x = get_local_x(local_to_global_x(seed_x_g, x as usize));
    //                 let local_y = get_local_y(get_global_y_coordinate(seed_y_g, y as usize));
    //                 let local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
    //                 let (chunk_x_g, chunk_y_g) = (get_chunk_x_g(x as f32), get_chunk_y_g(y as f32));
    //                 if let Some(chunk) = chunk_map.get_mut(&(chunk_x_g, chunk_y_g)) {
    //                     if GROUND.contains(&chunk[local_index]) {
    //                         chunk_map.get_mut(&(chunk_x_g, chunk_y_g)).unwrap()[local_index] = seed_type;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}

// fn generate_empty_chunk() -> Vec<u8> {
//     vec![0; (CHUNK_SIZE * CHUNK_SIZE) as usize]
// }