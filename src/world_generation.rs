use std::cmp::{max, min};
use std::collections::HashSet;
use std::time::Duration;

use bevy::asset::{Asset, Assets, Handle};
use bevy::color::palettes::css::GOLD;
use bevy::math::Vec2;
use bevy::prelude::{Image, Mesh, Query, Rectangle, ResMut, TextBundle, With};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, MaterialMesh2dBundle};
use bevy::text::{Text, TextSection, TextStyle};
use bevy::time::{Time, Timer, TimerMode};
use bevy_reflect::TypePath;
use iyes_perf_ui::entries::PerfUiBundle;
use bevy::utils::default;

use bevy::{asset::AssetServer, core_pipeline::core_2d::Camera2dBundle, ecs::system::{Commands, Res}, math::Vec3, transform::components::Transform};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use crate::color_map::{dirt_variant_pmf, COPPER, DIRT1, DIRT2, DIRT3, GRAVEL1, GRAVEL2, GRAVEL3, LIGHT, REFINED_COPPER, ROCK, SELL_BOX, SKY};
use crate::components::{Count, GravityCoords, TimerComponent, MoneyTextTag, SunTick, TerrainGridTag};
use crate::constants::{CALCOPIRITE_RADIUS, CHALCOPIRITE_SPAWN_COUNT, GROUND_HEIGHT, ROCK_HEIGHT, SELL_BOX_HEIGHT, SELL_BOX_WIDTH, SKY_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::drill::DrillTag;
use crate::util::{c_to_tl, distance, flatten_index_standard_grid, grid_to_image};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(PerfUiBundle::default());
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<GridMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<AssetServer>,
) {
    let mut color_map = grid_to_image(&generate_terrain_grid(), WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, None);
    add_sell_box_to_grid(&mut color_map.data);
    commands.spawn(TerrainGridTag)
            .insert(MaterialMesh2dBundle {
                material: materials.add(GridMaterial {
                    color_map: images.add(color_map),
                    size: Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
                }),
                mesh: meshes
                .add(Rectangle {
                    half_size: Vec2::new((WINDOW_WIDTH/2) as f32, (WINDOW_HEIGHT/2) as f32),
                })
                .into(),
                transform: Transform {
                    translation: Vec3::new(0., 0., -5.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(GravityCoords { coords: HashSet::new() });
    commands.spawn(TimerComponent { timer: Timer::new(Duration::from_millis(7), TimerMode::Repeating) }).insert(TerrainGridTag);
    commands.spawn(TimerComponent { timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating) }).insert(DrillTag);
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

fn generate_terrain_grid() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut grid: Vec<u8> = vec!(SKY; WINDOW_WIDTH * WINDOW_HEIGHT);
    let perlin = Perlin::new(rng.gen());
    let dirt_noise_smoothness = 0.003;
    let rock_noise_smoothness = 0.004;
    let dirt_variation = 15.;
    let rock_variation = 80.;

    for x in 0..WINDOW_WIDTH {
        let noise_value = perlin.get([x as f64 * dirt_noise_smoothness, 0.0]);
        let dirt_height = (GROUND_HEIGHT as f64 + (noise_value * dirt_variation)) as usize;
        for y in dirt_height..WINDOW_HEIGHT {
            if y < WINDOW_HEIGHT - (ROCK_HEIGHT as f64 - perlin.get([x as f64 * rock_noise_smoothness, 0.0]) * rock_variation) as usize {
                grid[flatten_index_standard_grid(&x, &y, WINDOW_WIDTH)] = dirt_variant_pmf();
            } else {
                grid[flatten_index_standard_grid(&x, &y, WINDOW_WIDTH)] = ROCK;
            }
        }
    }
    if SKY_HEIGHT + GROUND_HEIGHT < SKY_HEIGHT + GROUND_HEIGHT + ROCK_HEIGHT {
        for _ in 0..CHALCOPIRITE_SPAWN_COUNT {
            let x = rng.gen_range(0..WINDOW_WIDTH);
            let y = rng.gen_range(SKY_HEIGHT + GROUND_HEIGHT..SKY_HEIGHT + GROUND_HEIGHT + ROCK_HEIGHT);
            let index = flatten_index_standard_grid(&x, &y, WINDOW_WIDTH);
            grid[index] = COPPER;
            for xx in max(0, x - CALCOPIRITE_RADIUS)..min(WINDOW_WIDTH, x + CALCOPIRITE_RADIUS){
                for yy in max(0, y - CALCOPIRITE_RADIUS)..min(WINDOW_HEIGHT, y + CALCOPIRITE_RADIUS){
                    let distance = distance(xx as i32, yy as i32, x as i32, y as i32);
                    if distance < CALCOPIRITE_RADIUS as f32{
                        if distance != 0. && rng.gen_range(0..distance as usize * 2) == 0 {
                            let index = flatten_index_standard_grid(&xx, &yy, WINDOW_WIDTH);
                            grid[index] = COPPER;
                        }
                    }
                }
            }
        }
    }
    // for _ in 0..10 {
    //     let random_x: usize = rng.gen_range(0..WINDOW_WIDTH - 50);
    //     let random_y: usize = rng.gen_range(0..WINDOW_HEIGHT - 40);
    //     for x in 50..100 {
    //         for i in 0..40 {
    //             grid[flatten_index_standard_grid(&(random_x + x), &(i as usize + random_y), WINDOW_WIDTH)] = ROCK;
    //         }
    //     }
    // }
    grid
}


pub fn grid_tick(
    mut materials: ResMut<Assets<GridMaterial>>,
    terrain_material_handle: Query<&Handle<GridMaterial>, With<TerrainGridTag>>,
    mut images: ResMut<Assets<Image>>,
    time: Res<Time>,
    mut gravity_tick_timer_query: Query<&mut TimerComponent, With<TerrainGridTag>>,
    mut gravity_coords_query: Query<&mut GravityCoords>,
    mut money_count_query: Query<&mut Count>,
) {
    let mut gravity_tick_timer = gravity_tick_timer_query.get_single_mut().unwrap();
    gravity_tick_timer.timer.tick(time.delta());
    let terrain_grid = &mut images.get_mut(&materials.get_mut(terrain_material_handle.get_single().unwrap()).unwrap().color_map).unwrap().data;
    if gravity_tick_timer.timer.finished() {
        let mut money_count = money_count_query.get_single_mut().unwrap();
        let mut gravity_coords = gravity_coords_query.get_single_mut().unwrap();
        gravity_tick(&mut gravity_coords.coords, terrain_grid, &mut money_count.count);
    }
}

pub fn does_gravity_apply_to_entity(entity_pos_c: Vec3, entity_width: i32, entity_height: i32, terrain_grid: &Vec<u8>) -> bool {
    let entity_pos_tl = c_to_tl(&entity_pos_c, entity_width as f32, entity_height as f32);
    for x in entity_pos_tl.0 as usize..entity_pos_tl.0 as usize + entity_width as usize {
        let index = flatten_index_standard_grid(&x, &(entity_pos_tl.1 as usize + entity_height as usize), WINDOW_WIDTH);
        match &terrain_grid[index] {
            &SKY => continue,
            &SELL_BOX => continue,
            &LIGHT => continue,
            _ => return false
        }
    }
    true
}

fn gravity_tick(gravity_coords: &mut HashSet<(usize, usize)>, grid: &mut Vec<u8>, money_count: &mut f32) {
    let mut new_coords = HashSet::new();
    for coord in gravity_coords.iter() {
        let index = flatten_index_standard_grid(&coord.0, &coord.1, WINDOW_WIDTH);
        if grid[index] == DIRT1 || grid[index] == DIRT2 || grid[index] == DIRT3 || grid[index] == GRAVEL1 || grid[index] == GRAVEL2 || grid[index] == GRAVEL3 || grid[index] == COPPER {
            let mut below_index = flatten_index_standard_grid(&coord.0, &(coord.1 + 1), WINDOW_WIDTH);
            if grid[below_index] == SKY || grid[below_index] == LIGHT { 
                let mut looking_at_y = coord.1 + 1;
                new_coords.insert((coord.0, looking_at_y));
                loop {
                    below_index = flatten_index_standard_grid(&coord.0, &looking_at_y, WINDOW_WIDTH);
                    let above_index = flatten_index_standard_grid(&coord.0, &(looking_at_y - 1), WINDOW_WIDTH);
                    if grid[above_index] == SKY || grid[above_index] == REFINED_COPPER ||  grid[above_index] == ROCK || grid[above_index] == LIGHT {
                        break;
                    }
                    grid[below_index] = grid[above_index].clone();
                    grid[above_index] = SKY;
                    looking_at_y -= 1;
                }
            } else if grid[below_index] == SELL_BOX {
                let mut looking_at_y = coord.1 + 1;
                new_coords.insert((coord.0, looking_at_y));
                loop {
                    let above_index = flatten_index_standard_grid(&coord.0, &(looking_at_y - 1), WINDOW_WIDTH);
                    if grid[above_index] == SKY || grid[above_index] == REFINED_COPPER {
                        break;
                    }
                    match grid[above_index] {
                        COPPER => *money_count += 0.5,
                        DIRT1 => *money_count += 0.01,
                        DIRT2 => *money_count += 0.01,
                        DIRT3 => *money_count += 0.01,
                        GRAVEL1 => *money_count += 0.01,
                        GRAVEL2 => *money_count += 0.01,
                        GRAVEL3 => *money_count += 0.01,
                        _ => {}
                    }
                    grid[above_index] = SKY;
                    looking_at_y -= 1;
                }
            }
        }
    };
    *gravity_coords = new_coords;
}

fn add_sell_box_to_grid(grid: &mut Vec<u8>) {
    for y in SKY_HEIGHT - SELL_BOX_HEIGHT..SKY_HEIGHT{
        for x in 800..800+SELL_BOX_WIDTH{
            let index = flatten_index_standard_grid(&x, &y, WINDOW_WIDTH);
            if x < 800 + SELL_BOX_WIDTH - 1 - 2 && y < SKY_HEIGHT - 1 - 2 && x > 800 + 2{
                grid[index] = SELL_BOX;
            } else {
                grid[index] = REFINED_COPPER;
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