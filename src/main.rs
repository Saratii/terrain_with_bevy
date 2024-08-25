mod world_generation;
mod layer_map;
pub mod constants;
pub mod player;
pub mod components;
pub mod render;
pub mod util;
pub mod mouse_controller;
pub mod keyboard_controller;

use bevy::app::*;
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::SystemInformationDiagnosticsPlugin;
use bevy::ecs::schedule::ScheduleBuildSettings;
use bevy::prelude::*;
use bevy::window::PresentMode;
use constants::WINDOW_HEIGHT;
use iyes_perf_ui::PerfUiPlugin;
use keyboard_controller::process_key_event;
use mouse_controller::check_mouse_click;
use player::update_tool;
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
        .add_systems(Update, (grid_tick, process_key_event, check_mouse_click, update_tool, render_scene))
        .run();
}