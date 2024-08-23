
use std::collections::HashSet;

use bevy::{prelude::Component, time::Timer};

#[derive(Component, Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
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
pub struct TerrainGridTag;

#[derive(Component, Debug)]
pub struct Count{
    pub count: usize
}

#[derive(Component, Debug)]
pub struct TerrainPositionsAffectedByGravity{
    pub positions: HashSet<usize>
}

#[derive(Component)]
pub struct GravityTick{
    pub timer: Timer,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Pixel {
    Ground, 
    Sky,
    White,
    TranslucentGrey,
    Clear,
}

#[derive(Component)]
pub struct ErosionColumns{
    pub columns: HashSet<usize>
}