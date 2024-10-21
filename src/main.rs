mod world_generation;
pub mod constants;
pub mod player;
pub mod components;
pub mod util;
pub mod mouse_controller;
pub mod keyboard_controller;
pub mod fog;
pub mod sun;
pub mod tools;
pub mod color_map;
pub mod ui;
pub mod drill;
pub mod render;

use bevy::app::*;
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::SystemInformationDiagnosticsPlugin;
use bevy::ecs::schedule::ScheduleBuildSettings;
use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
use bevy::window::PresentMode;
use color_map::BLACK;
use color_map::RED;
use components::ChunkMap;
use constants::CHUNKS_HORIZONTAL;
use constants::CHUNKS_VERTICAL;
use constants::CHUNK_SIZE;
use constants::WINDOW_HEIGHT;
use drill::drill_tick;
use iyes_perf_ui::PerfUiPlugin;
use keyboard_controller::process_key_event;
use mouse_controller::check_mouse_click;
use player::spawn_player;
use tools::spawn_tools;
use tools::update_tool;
use util::flatten_index_standard_grid;
use util::get_chunk_x_g;
use util::get_chunk_x_v;
use util::get_chunk_y_g;
use util::get_chunk_y_v;
use util::get_local_x;
use util::get_local_y;
use world_generation::setup_camera;
use world_generation::setup_world;
use world_generation::grid_tick;
use world_generation::update_money_text;
use world_generation::GridMaterial;
use crate::constants::WINDOW_WIDTH;
use crate::render::render;

fn main() {
    App::new()
          .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                title: "UwU".into(),
                resolution: (WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
          FrameTimeDiagnosticsPlugin,
          EntityCountDiagnosticsPlugin,
          SystemInformationDiagnosticsPlugin,
          Material2dPlugin::<GridMaterial>::default(),
          PerfUiPlugin,
        ))
        .edit_schedule(Startup, |schedule| {
          schedule.set_build_settings(ScheduleBuildSettings {
              auto_insert_apply_deferred: false,
              ..default()
          });
        })
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, (setup_camera, setup_world, spawn_player, apply_deferred, spawn_tools).chain())
        .add_systems(Update, (render, process_key_event, update_tool, check_mouse_click))
        // .add_systems(Update, (grid_tick, process_key_event, update_tool, check_mouse_click, update_money_text, drill_tick))
        .run();
}

fn debugy(
  mut chunk_map_query: Query<&mut ChunkMap>,

) {
  let mut chunk_map = chunk_map_query.get_single_mut().unwrap();
  let global_y = -180;
  let chunk_y_g = get_chunk_y_g(global_y as f32);
  let chunk_y_v = get_chunk_y_v(chunk_y_g);
  let local_y = get_local_y(global_y);
  for x in 0..100 {
    let chunk_x_g = get_chunk_x_g(x as f32);
    let chunk_x_v = get_chunk_x_v(chunk_x_g);
    let local_x = get_local_x(x as i32);
    let chunk_index = flatten_index_standard_grid(&chunk_x_v, &chunk_y_v, CHUNKS_HORIZONTAL as usize);
    let local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
    chunk_map.map[chunk_index][local_index] = RED;
  }
}

fn debugx(
  mut chunk_map_query: Query<&mut ChunkMap>,

) {
  let global_x = 0;
  let mut chunk_map = chunk_map_query.get_single_mut().unwrap();
  let chunk_x_g = get_chunk_x_g(global_x as f32);
  let chunk_x_v = get_chunk_x_v(chunk_x_g);
  let local_x = get_local_x(global_x);
  for y in 0..100 {
    let chunk_y_g = get_chunk_y_g(y as f32);
    let chunk_y_v = get_chunk_y_v(chunk_y_g);
    let local_y = get_local_y(y as i32);
    let chunk_index = flatten_index_standard_grid(&chunk_x_v, &chunk_y_v, CHUNKS_VERTICAL as usize);
    let local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
    chunk_map.map[chunk_index][local_index] = BLACK;
  }
}