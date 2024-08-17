
use bevy::prelude::Component;

use crate::world_generation::Pixel;

#[derive(Component, Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

#[derive(Component)]
pub struct PlayerTag;

#[derive(Component)]
pub struct CursorTag;

#[derive(Component, Debug)]
pub struct Grid{
    pub data: Vec<Pixel>
}

#[derive(Component)]
pub struct ImageBuffer{
    pub data: Vec<u8>
}

#[derive(Component)]
pub struct GridImageTag;