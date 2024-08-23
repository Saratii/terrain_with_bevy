mod world_generation;
mod layer_map;
pub mod constants;
pub mod player;
pub mod components;
pub mod render;
pub mod util;

use bevy::app::*;
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::SystemInformationDiagnosticsPlugin;
use bevy::ecs::schedule::ScheduleBuildSettings;
use bevy::prelude::*;
use bevy::window::PresentMode;
use constants::WINDOW_HEIGHT;
use iyes_perf_ui::PerfUiPlugin;
use player::check_mouse_click;
use player::move_player;
use player::update_cursor;
use render::render_scene;
use world_generation::setup_world;
use world_generation::grid_tick;
use crate::constants::WINDOW_WIDTH;

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
          PerfUiPlugin,
        ))
        .edit_schedule(Startup, |schedule| {
          schedule.set_build_settings(ScheduleBuildSettings {
              auto_insert_apply_deferred: false,
              ..default()
          });
        })
        .add_systems(Startup, (apply_deferred, setup_world).chain())
        .add_systems(Update, (grid_tick, move_player, check_mouse_click, update_cursor, render_scene))
        .run();
}