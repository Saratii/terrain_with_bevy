use std::collections::HashSet;
use std::time::Duration;

use bevy::asset::{Assets, Handle};
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, MouseButton, Query, ResMut, With, Without};
use bevy::time::{Time, Timer, TimerMode};
use bevy::window::{PrimaryWindow, Window};
use iyes_perf_ui::entries::PerfUiBundle;
use bevy::utils::default;

use bevy::{asset::AssetServer, core_pipeline::core_2d::Camera2dBundle, ecs::system::{Commands, Res}, math::Vec3, render::texture::Image, sprite::SpriteBundle, transform::components::Transform};
use crate::components::{Count, CursorTag, GravityTick, Grid, ImageBuffer, Pixel, PlayerTag, Position, TerrainGridTag, TerrainPositionsAffectedByGravity, Velocity};
use crate::constants::{CURSOR_RADIUS, GROUND_HEIGHT, PLAYER_SPAWN_X, PLAYER_SPAWN_Y, SKY_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::player::{check_mouse_click, generate_cursor_grid, generate_player_image, move_player, update_cursor};
use crate::render::render_grid;
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
            .insert(TerrainPositionsAffectedByGravity{positions: HashSet::new()});
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

pub fn update(
    mut grid_query: Query<&mut Grid, (With<TerrainGridTag>, Without<CursorTag>)>,
    mut images: ResMut<Assets<Image>>,
    mut image_query: Query<&Handle<Image>, With<TerrainGridTag>>,
    mut cursor_image_query: Query<&Handle<Image>, With<CursorTag>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Velocity), (With<PlayerTag>, Without<CursorTag>)>,
    time: Res<Time>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut cursor_query: Query<&mut Transform, (With<CursorTag>, Without<PlayerTag>)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut image_buffer_query: Query<&mut ImageBuffer, Without<CursorTag>>,
    mut cursor_image_buffer_query: Query<&mut ImageBuffer, With<CursorTag>>,
    mut cursor_capacity_count_query: Query<&mut Count, With<CursorTag>>,
    mut shovel_grid_query: Query<&mut Grid, (With<CursorTag>, Without<TerrainGridTag>)>,
    mut gravity_tick_timer_quiery: Query<&mut GravityTick>,
    mut gravity_columns_query: Query<&mut TerrainPositionsAffectedByGravity>,
) {
    let mut grid = grid_query.get_single_mut().unwrap();
    let mut image_buffer = image_buffer_query.get_single_mut().unwrap();
    let mut cursor_image_buffer = cursor_image_buffer_query.get_single_mut().unwrap();
    let mut cursor_position = cursor_query.get_single_mut().unwrap();
    let mut cursor_capacity_count = cursor_capacity_count_query.get_single_mut().unwrap();
    let mut player = player_query.get_single_mut().unwrap();
    let mut gravity_tick_timer = gravity_tick_timer_quiery.get_single_mut().unwrap();
    let mut gravity_columns = gravity_columns_query.get_single_mut().unwrap();
    move_player(&grid.data, keys, &mut player.0, &mut player.1, &time);
    update_cursor(q_windows, &mut player.0, &mut cursor_position);
    let mut shovel_grid = shovel_grid_query.get_single_mut().unwrap();
    check_mouse_click(buttons, &cursor_position, &mut grid.data, &mut cursor_capacity_count, &mut shovel_grid.data, &mut gravity_columns.positions);
    tick_terrain_gravity(&mut gravity_columns.positions, &mut grid.data, &mut gravity_tick_timer.timer, &time);
    render_grid(&grid.data, &mut image_buffer.data);
    render_grid(&shovel_grid.data, &mut cursor_image_buffer.data);
    if let Some(image) = images.get_mut(image_query.single_mut()) {
        image.data = image_buffer.data.clone();
    }
    if let Some(image) = images.get_mut(cursor_image_query.single_mut()) {
        image.data = cursor_image_buffer.data.clone()
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

fn tick_terrain_gravity(columns: &mut HashSet<usize>, grid: &mut Vec<Pixel>, timer: &mut Timer, time: &Res<Time>){
    timer.tick(time.delta());
    if timer.finished(){
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
}