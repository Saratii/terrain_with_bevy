use std::collections::HashSet;
use std::time::Duration;

use bevy::prelude::{Query, With, Without};
use bevy::time::{Time, Timer, TimerMode};
use iyes_perf_ui::entries::PerfUiBundle;
use bevy::utils::default;

use bevy::{asset::AssetServer, core_pipeline::core_2d::Camera2dBundle, ecs::system::{Commands, Res}, math::Vec3, sprite::SpriteBundle, transform::components::Transform};
use crate::components::{Count, CursorTag, ErosionColumns, GravityTick, Grid, ImageBuffer, Pixel, PlayerTag, Position, TerrainGridTag, TerrainPositionsAffectedByGravity, Velocity};
use crate::constants::{CURSOR_RADIUS, GROUND_HEIGHT, PLAYER_SPAWN_X, PLAYER_SPAWN_Y, SKY_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::player::{generate_cursor_grid, generate_player_image};
use crate::util::{flatten_index, flatten_index_standard_grid, grid_to_image};

pub fn setup_world(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(PerfUiBundle::default());
    commands.spawn(Camera2dBundle::default());
    let terrain_grid = generate_terrain_grid();
    let shovel_grid = generate_cursor_grid();
    let terrain_image = grid_to_image(&terrain_grid, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);
    let shovel_image = grid_to_image(&shovel_grid, CURSOR_RADIUS as u32 * 2, CURSOR_RADIUS as u32 * 2);
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
            });
    commands.spawn(CursorTag)
            .insert(SpriteBundle{
                texture: assets.add(grid_to_image(&shovel_grid.clone(), CURSOR_RADIUS as u32 * 2, CURSOR_RADIUS as u32 * 2)),
                transform: Transform { translation: Vec3 { x: PLAYER_SPAWN_X as f32, y: PLAYER_SPAWN_Y as f32, z: 1. }, ..default()},
                ..default()})
            .insert(Count{count: 0})
            .insert(Grid{data: shovel_grid})
            .insert(ImageBuffer{data: shovel_image.data});
    commands.spawn(GravityTick{timer: Timer::new(Duration::from_millis(20), TimerMode::Repeating)});
}

fn generate_terrain_grid() -> Vec<Pixel> {
    let mut grid: Vec<Pixel> = Vec::with_capacity(4 * WINDOW_WIDTH * WINDOW_HEIGHT);
    for _ in 0..SKY_HEIGHT * WINDOW_WIDTH{
        grid.push(Pixel::Sky);
    }
    for _ in 0..GROUND_HEIGHT * WINDOW_WIDTH{
        grid.push(Pixel::Ground);
    }
    grid
}

pub fn grid_tick(
    mut grid_query: Query<&mut Grid, (With<TerrainGridTag>, Without<CursorTag>)>,
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
        gravity_tick(&mut gravity_columns.positions, &mut grid.data);
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

fn gravity_tick(columns: &mut HashSet<usize>, grid: &mut Vec<Pixel>){
    columns.retain(|column| {
        let mut have_any_moved = false;
        for y in (0..WINDOW_HEIGHT-1).rev() {
            let index = flatten_index_standard_grid(column, &y, WINDOW_WIDTH);
            if grid[index] == Pixel::Ground{
                let below_index = flatten_index_standard_grid(column, &(y + 1), WINDOW_WIDTH);
                if grid[below_index] == Pixel::Sky{
                    have_any_moved = true;
                    grid[below_index] = Pixel::Ground;
                    grid[index] = Pixel::Sky;
                }
            }
        }
        have_any_moved
    })
    
}