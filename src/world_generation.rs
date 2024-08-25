use std::collections::HashSet;
use std::time::Duration;

use bevy::prelude::{Query, Visibility, With, Without};
use bevy::time::{Time, Timer, TimerMode};
use iyes_perf_ui::entries::PerfUiBundle;
use bevy::utils::default;

use bevy::{asset::AssetServer, core_pipeline::core_2d::Camera2dBundle, ecs::system::{Commands, Res}, math::Vec3, sprite::SpriteBundle, transform::components::Transform};
use rand::{thread_rng, Rng};
use crate::components::{ContentList, CurrentTool, ErosionColumns, GravityTick, Grid, ImageBuffer, PickaxeTag, Pixel, PlayerTag, Position, ShovelTag, TerrainGridTag, TerrainPositionsAffectedByGravity, Tool, Velocity};
use crate::constants::{CURSOR_RADIUS, GROUND_HEIGHT, MIN_EROSION_HEIGHT, PLAYER_SPAWN_X, PLAYER_SPAWN_Y, ROCK_HEIGHT, SKY_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::player::{generate_pickaxe_grid, generate_player_image, generate_shovel_grid};
use crate::util::{flatten_index, flatten_index_standard_grid, grid_to_image};

pub fn setup_world(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(PerfUiBundle::default());
    commands.spawn(Camera2dBundle::default());
    let terrain_grid = generate_terrain_grid();
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
                    transform: Transform { translation: Vec3 { x: 0., y: 0., z: 0. }, ..default()},
                    ..default()
                }
            )
            .insert(ImageBuffer{data: terrain_image.data})
            .insert(TerrainPositionsAffectedByGravity{positions: HashSet::new()})
            .insert(ErosionColumns{columns: HashSet::new()});
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
    for _ in 0..ROCK_HEIGHT * WINDOW_WIDTH{
        grid.push(Pixel::Rock);
    }
    grid
}

pub fn grid_tick(
    mut grid_query: Query<&mut Grid, (With<TerrainGridTag>, Without<ShovelTag>)>,
    time: Res<Time>,
    mut gravity_tick_timer_quiery: Query<&mut GravityTick>,
    mut gravity_columns_query: Query<&mut TerrainPositionsAffectedByGravity>,
    mut erosion_columns_query: Query<&mut ErosionColumns>,
) {
    let mut grid = grid_query.get_single_mut().unwrap();
    let mut gravity_tick_timer = gravity_tick_timer_quiery.get_single_mut().unwrap();
    let mut gravity_columns = gravity_columns_query.get_single_mut().unwrap();
    let mut erosion_columns = erosion_columns_query.get_single_mut().unwrap();
    gravity_tick_timer.timer.tick(time.delta());
    if gravity_tick_timer.timer.finished(){
        gravity_tick(&mut gravity_columns.positions, &mut grid.data, &mut erosion_columns.columns);
        erosion_tick(&mut erosion_columns.columns, &mut grid.data, &mut gravity_columns.positions);
    }
}

pub fn does_gravity_apply_to_entity(entity_x: i32, entity_y: i32, entity_width: i32, entity_height: i32, grid: &Vec<Pixel>) -> bool {
    for x in entity_x..entity_x + entity_width {
        let index = flatten_index(x, entity_y - entity_height/2);
        match &grid[index]{
            Pixel::Sky => continue,
            _ => return false
        }
    }
    true
}

fn gravity_tick(columns: &mut HashSet<usize>, grid: &mut Vec<Pixel>, erosion_columns: &mut HashSet<usize>){
    columns.retain(|column| {
        let mut have_any_moved = false;
        for y in (0..WINDOW_HEIGHT-1).rev() {
            let index = flatten_index_standard_grid(column, &y, WINDOW_WIDTH);
            if let Pixel::Ground(variant) = grid[index].clone(){
                let below_index = flatten_index_standard_grid(column, &(y + 1), WINDOW_WIDTH);
                if grid[below_index] == Pixel::Sky{
                    have_any_moved = true;
                    grid[below_index] = Pixel::Ground(variant);
                    grid[index] = Pixel::Sky;
                }
            } else if grid[index] == Pixel::Gravel{
                let below_index = flatten_index_standard_grid(column, &(y + 1), WINDOW_WIDTH);
                if grid[below_index] == Pixel::Sky{
                    have_any_moved = true;
                    grid[below_index] = Pixel::Gravel;
                    grid[index] = Pixel::Sky;
                }
            }
        }
        if !have_any_moved{
            erosion_columns.insert(*column);
        }
        have_any_moved
    });
}

fn erosion_tick(erosion_columns: &mut HashSet<usize>, grid: &mut Vec<Pixel>, gravity_columns: &mut HashSet<usize>){
    let mut new_erosion_columns = HashSet::new();
    erosion_columns.retain(|column| {
        if gravity_columns.contains(&(column - 1)) || gravity_columns.contains(&(column + 1)){
            return true
        }
        let last_sky_index = find_last_sky_height(*column, grid);
        let last_sky_index_left = find_last_sky_height(*column - 1, grid);
        let last_sky_index_right = find_last_sky_height(*column + 1, grid);
        let left_to_center_distance =  last_sky_index_left as i32 - last_sky_index as i32;
        let right_to_center_distance = last_sky_index_right as i32 - last_sky_index as i32;
        if left_to_center_distance > MIN_EROSION_HEIGHT && left_to_center_distance > right_to_center_distance{
            let center_index = flatten_index_standard_grid(column, &(last_sky_index + 1), WINDOW_WIDTH);
            let moved_pixel = grid[center_index].clone();
            grid[center_index] = Pixel::Sky;
            let left_index = flatten_index_standard_grid(&(column - 1), &(last_sky_index + 1), WINDOW_WIDTH);
            grid[left_index] = moved_pixel;
            gravity_columns.insert(column - 1);
            new_erosion_columns.insert(column + 1);
            return true
        } else if right_to_center_distance > MIN_EROSION_HEIGHT{
            let center_index = flatten_index_standard_grid(column, &(last_sky_index + 1), WINDOW_WIDTH);
            let moved_pixel = grid[center_index].clone();
            grid[center_index] = Pixel::Sky;
            let right_index = flatten_index_standard_grid(&(column + 1), &(last_sky_index + 1), WINDOW_WIDTH);
            grid[right_index] = moved_pixel;
            gravity_columns.insert(column + 1);
            new_erosion_columns.insert(column - 1);
            return true
        }
        false
    });
    erosion_columns.extend(new_erosion_columns)
}

fn find_last_sky_height(column: usize, grid: &Vec<Pixel>) -> usize {
    for y in (0..WINDOW_HEIGHT).rev() {
        let index = flatten_index_standard_grid(&column, &y, WINDOW_WIDTH);
        if grid[index] == Pixel::Sky{
            return y
        }
    }
    0
}