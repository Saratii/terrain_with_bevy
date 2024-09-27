mod world_generation;
pub mod constants;
pub mod player;
pub mod components;
pub mod render;
pub mod util;
pub mod mouse_controller;
pub mod keyboard_controller;
pub mod fog;
pub mod sun;
pub mod tools;

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
use sun::render_rays;
use render::render_scene;
use sun::lighting_update;
use sun::move_sun;
use sun::spawn_sun;
use tools::spawn_tools;
use tools::update_tool;
use world_generation::setup_camera;
use world_generation::setup_world;
use world_generation::grid_tick;
use world_generation::update_money_text;
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
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, (setup_camera, setup_world, apply_deferred, spawn_sun).chain())
        .add_systems(Update, (grid_tick, render_scene, lighting_update, render_rays, move_sun))
        .run();
}