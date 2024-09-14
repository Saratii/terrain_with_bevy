use std::cmp::{max, min};
use std::collections::HashSet;
use std::time::Duration;

use bevy::color::palettes::css::GOLD;
use bevy::prelude::{Query, TextBundle, Visibility, With, Without};
use bevy::text::{Text, TextSection, TextStyle};
use bevy::time::{Time, Timer, TimerMode};
use iyes_perf_ui::entries::PerfUiBundle;
use bevy::utils::default;

use bevy::{asset::AssetServer, core_pipeline::core_2d::Camera2dBundle, ecs::system::{Commands, Res}, math::Vec3, sprite::SpriteBundle, transform::components::Transform};
use rand::Rng;
use crate::components::{ContentList, Count, CurrentTool, ErosionCoords, GravityCoords, GravityTick, Grid, ImageBuffer, MoneyTextTag, PickaxeTag, Pixel, PlayerTag, Position, Rock, ShovelTag, TerrainGridTag, Tool, Velocity};
use crate::constants::{CALCOPIRITE_RADIUS, CHALCOPIRITE_SPAWN_COUNT, CURSOR_RADIUS, GROUND_HEIGHT, PLAYER_SPAWN_X, PLAYER_SPAWN_Y, ROCK_HEIGHT, SELL_BOX_HEIGHT, SELL_BOX_WIDTH, SKY_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::player::{generate_pickaxe_grid, generate_player_image, generate_shovel_grid};
use crate::util::{distance, flatten_index, flatten_index_standard_grid, grid_to_image};

pub fn setup_world(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(PerfUiBundle::default());
    commands.spawn(Camera2dBundle::default());
    let mut terrain_grid = generate_terrain_grid();
    add_sell_box_to_grid(&mut terrain_grid);
    let shovel_grid = generate_shovel_grid();
    let pickaxe_grid = generate_pickaxe_grid();
    let terrain_image = grid_to_image(&terrain_grid, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);
    let shovel_image = grid_to_image(&shovel_grid, CURSOR_RADIUS as u32 * 2, CURSOR_RADIUS as u32 * 2);
    let pickaxe_image = grid_to_image(&pickaxe_grid, CURSOR_RADIUS as u32 * 2, CURSOR_RADIUS as u32 * 2);
    commands.spawn(TerrainGridTag)
            .insert(Grid{data: terrain_grid})
            .insert(
        SpriteBundle{
                    texture: assets.add(terrain_image.clone()),
                    transform: Transform { translation: Vec3 { x: 0., y: 0., z: -1. }, ..default()},
                    ..default()
                }
            )
            .insert(ImageBuffer{data: terrain_image.data})
            .insert(GravityCoords{coords: HashSet::new()})
            .insert(ErosionCoords{coords: HashSet::new()});
    commands.spawn(PlayerTag)
            .insert(Position{x: PLAYER_SPAWN_X as f32, y: PLAYER_SPAWN_Y as f32})
            .insert(Velocity{vx: 0.0, vy: 0.0})
            .insert(SpriteBundle{
                texture: assets.add(generate_player_image()),
                transform: Transform { translation: Vec3 { x: PLAYER_SPAWN_X as f32, y: PLAYER_SPAWN_Y as f32, z: 1. }, ..default()},
                ..default()
            })
            .insert(CurrentTool{tool: Tool::Shovel});
    commands.spawn(ShovelTag)
            .insert(SpriteBundle{
                texture: assets.add(grid_to_image(&shovel_grid.clone(), CURSOR_RADIUS as u32 * 2, CURSOR_RADIUS as u32 * 2)),
                transform: Transform { translation: Vec3 { x: PLAYER_SPAWN_X as f32, y: PLAYER_SPAWN_Y as f32, z: 1. }, ..default()},
                ..default()})
            .insert(ContentList{contents: Vec::new()})
            .insert(Grid{data: shovel_grid})
            .insert(ImageBuffer{data: shovel_image.data});
    commands.spawn(GravityTick{timer: Timer::new(Duration::from_millis(20), TimerMode::Repeating)});
    commands.spawn(PickaxeTag)
            .insert(SpriteBundle{
                texture: assets.add(grid_to_image(&pickaxe_grid.clone(), CURSOR_RADIUS as u32 * 2, CURSOR_RADIUS as u32 * 2)),
                transform: Transform { translation: Vec3 { x: PLAYER_SPAWN_X as f32, y: PLAYER_SPAWN_Y as f32, z: 1. }, ..default()},
                visibility: Visibility::Hidden,
                ..default()})
            .insert(Grid{data: pickaxe_grid})
            .insert(ImageBuffer{data: pickaxe_image.data});
    commands.spawn(Count{count: 0.});

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

fn generate_terrain_grid() -> Vec<Pixel> {
    let mut rng = rand::thread_rng();
    let mut grid: Vec<Pixel> = Vec::with_capacity(4 * WINDOW_WIDTH * WINDOW_HEIGHT);
    for _ in 0..SKY_HEIGHT * WINDOW_WIDTH{
        grid.push(Pixel::Sky);
    }
    for _ in 0..GROUND_HEIGHT * WINDOW_WIDTH{
        grid.push(Pixel::Ground(rng.gen()));
    }
    for y in GROUND_HEIGHT + SKY_HEIGHT .. WINDOW_HEIGHT {
        for _ in 0..WINDOW_WIDTH {
            grid.push(Pixel::Rock(Rock{vertical_force: y - GROUND_HEIGHT - SKY_HEIGHT}));
        }
    }
    for _ in 0..CHALCOPIRITE_SPAWN_COUNT{
        let x = rng.gen_range(0..WINDOW_WIDTH);
        let y = rng.gen_range(SKY_HEIGHT + GROUND_HEIGHT..SKY_HEIGHT + GROUND_HEIGHT + ROCK_HEIGHT);
        let index = flatten_index_standard_grid(&x, &y, WINDOW_WIDTH);
        grid[index] = Pixel::Chalcopyrite;
        for xx in max(0, x - CALCOPIRITE_RADIUS)..min(WINDOW_WIDTH, x + CALCOPIRITE_RADIUS){
            for yy in max(0, y - CALCOPIRITE_RADIUS)..min(WINDOW_HEIGHT, y + CALCOPIRITE_RADIUS){
                let distance = distance(xx as i32, yy as i32, x as i32, y as i32);
                if distance < CALCOPIRITE_RADIUS as f32{
                    if distance != 0. && rng.gen_range(0..distance as usize * 2) == 0{
                        let index = flatten_index_standard_grid(&xx, &yy, WINDOW_WIDTH);
                        grid[index] = Pixel::Chalcopyrite;
                    }
                }
            }
        }
    }
    grid
}

pub fn grid_tick(
    mut grid_query: Query<&mut Grid, (With<TerrainGridTag>, Without<ShovelTag>)>,
    time: Res<Time>,
    mut gravity_tick_timer_quiery: Query<&mut GravityTick>,
    mut gravity_coords_query: Query<&mut GravityCoords>,
    mut money_count_query: Query<&mut Count>,
) {
    let mut gravity_tick_timer = gravity_tick_timer_quiery.get_single_mut().unwrap();
    gravity_tick_timer.timer.tick(time.delta());
    if gravity_tick_timer.timer.finished(){
        let mut grid = grid_query.get_single_mut().unwrap();
        let mut money_count = money_count_query.get_single_mut().unwrap();
        let mut gravity_coords = gravity_coords_query.get_single_mut().unwrap();
        gravity_tick(&mut gravity_coords.coords, &mut grid.data, &mut money_count.count);
    }
}

pub fn does_gravity_apply_to_entity(entity_x: i32, entity_y: i32, entity_width: i32, entity_height: i32, grid: &Vec<Pixel>) -> bool {
    for x in entity_x..entity_x + entity_width {
        let index = flatten_index(x, entity_y - entity_height/2);
        match &grid[index]{
            Pixel::Sky => continue,
            Pixel::SellBox => continue,
            _ => return false
        }
    }
    true
}

fn gravity_tick(gravity_coords: &mut HashSet<(usize, usize)>, grid: &mut Vec<Pixel>, money_count: &mut f32){
    let mut new_coords = HashSet::new();
    for coord in gravity_coords.iter(){
        let index = flatten_index_standard_grid(&coord.0, &coord.1, WINDOW_WIDTH);
        if matches!(grid[index], Pixel::Ground(_) | Pixel::Gravel(_) | Pixel::Chalcopyrite){
            let mut below_index = flatten_index_standard_grid(&coord.0, &(coord.1 + 1), WINDOW_WIDTH);
            if grid[below_index] == Pixel::Sky{ 
                let mut looking_at_y = coord.1 + 1;
                new_coords.insert((coord.0, looking_at_y));
                loop {
                    below_index = flatten_index_standard_grid(&coord.0, &looking_at_y, WINDOW_WIDTH);
                    let above_index = flatten_index_standard_grid(&coord.0, &(looking_at_y - 1), WINDOW_WIDTH);
                    if matches!(grid[above_index], Pixel::Sky | Pixel::RefinedCopper | Pixel::Rock(_)) {
                        break;
                    }
                    grid[below_index] = grid[above_index].clone();
                    grid[above_index] = Pixel::Sky;
                    looking_at_y -= 1;
                }
            } else if grid[below_index] == Pixel::SellBox{
                let mut looking_at_y = coord.1 + 1;
                new_coords.insert((coord.0, looking_at_y));
                loop {
                    let above_index = flatten_index_standard_grid(&coord.0, &(looking_at_y - 1), WINDOW_WIDTH);
                    if grid[above_index] == Pixel::Sky || grid[above_index] == Pixel::RefinedCopper{
                        break
                    }
                    match grid[above_index]{
                        Pixel::Chalcopyrite => *money_count += 0.5,
                        Pixel::Ground(_) => *money_count += 0.01,
                        Pixel::Gravel(_) => *money_count += 0.01,
                        _ => {}
                    }
                    grid[above_index] = Pixel::Sky;
                    looking_at_y -= 1;
                }
            }
        }
    };
    *gravity_coords = new_coords;
}

fn add_sell_box_to_grid(grid: &mut Vec<Pixel>) {
    for y in SKY_HEIGHT - SELL_BOX_HEIGHT..SKY_HEIGHT{
        for x in 800..800+SELL_BOX_WIDTH{
            let index = flatten_index_standard_grid(&x, &y, WINDOW_WIDTH);
            if x < 800 + SELL_BOX_WIDTH - 1 - 2 && y < SKY_HEIGHT - 1 - 2 && x > 800 + 2{
                grid[index] = Pixel::SellBox;
            } else {
                grid[index] = Pixel::RefinedCopper
            }
        }
    }
}

pub fn update_money_text(
    mut money_text_query: Query<&mut Text, With<MoneyTextTag>>,
    mut money_count_query: Query<&Count>,
){
    let money_count = money_count_query.get_single_mut().unwrap();
    let mut money_text = money_text_query.get_single_mut().unwrap();
    money_text.sections[0].value = format!("${:.2}", money_count.count);
}