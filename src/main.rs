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

use std::time::Duration;

use bevy::app::*;
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::SystemInformationDiagnosticsPlugin;
use bevy::ecs::schedule::ScheduleBuildSettings;
use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
use bevy::utils::HashMap;
use bevy::window::PresentMode;
use components::ChunkMap;
use constants::CHUNK_SIZE;
use constants::LIGHTING_DEMO;
use constants::WINDOW_HEIGHT;
use drill::drill_tick;
use iyes_perf_ui::PerfUiPlugin;
use keyboard_controller::process_key_event;
use mouse_controller::check_mouse_click;
use player::spawn_player;
use rand::Rng;
use tools::spawn_tools;
use tools::update_tool;
use util::flatten_index_standard_grid;
use world_generation::setup_camera;
use world_generation::setup_world;
use world_generation::grid_tick;
use world_generation::update_money_text;
use world_generation::GridMaterial;
use crate::constants::WINDOW_WIDTH;
use crate::render::render;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
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
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)));
    if LIGHTING_DEMO {
      app.add_systems(Startup, (setup_camera, setup_world, setup_timer).chain());
      app.add_systems(Update, (spawn_random_squares, render));
    } else {
      app.add_systems(Startup, (setup_camera, setup_world, spawn_player, apply_deferred, spawn_tools).chain());
      app.add_systems(Update, (process_key_event, update_tool, check_mouse_click, grid_tick, render));
    }
    app.run();
}

#[derive(Resource)]
struct SpawnTimer(Timer);

fn spawn_random_squares(
//     mut chunk_map_query: Query<&mut ChunkMap>,
//     time: Res<Time>,
//     mut timer: ResMut<SpawnTimer>,
) {
//     timer.0.tick(time.delta());
//     if timer.0.finished() {
//         if let Ok(mut chunk_map) = chunk_map_query.get_single_mut() {
//             let mut rng = rand::thread_rng();
//             chunk_map.map = HashMap::new();
//             chunk_map.map = vec![vec![0; CHUNK_SIZE as usize * CHUNK_SIZE as usize]; 9];
//             for _ in 0..15 {
//                 let x = rng.gen_range(0..1000);
//                 let y = rng.gen_range(0..1000);
//                 let size = rng.gen_range(10..200);
//                 for x in x..x + size {
//                     for y in y..y + size {
//                         chunk_map.map[4][flatten_index_standard_grid(
//                             &(x as usize),
//                             &(y as usize),
//                             CHUNK_SIZE as usize,
//                         )] = color_map::ROCK;
//                     }
//                 }
//             }
//         }
//     }
}


fn setup_timer(mut commands: Commands) {
  commands.insert_resource(SpawnTimer(Timer::new(Duration::from_millis(1000), TimerMode::Repeating)));
}