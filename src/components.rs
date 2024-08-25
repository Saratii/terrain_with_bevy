
use std::collections::HashSet;

use bevy::{prelude::Component, time::Timer};
use rand::{distributions::Standard, prelude::Distribution, Rng};

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
pub struct ShovelTag;

#[derive(Component)]
pub struct PickaxeTag;

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
pub struct ContentList{
    pub contents: Vec<Pixel>
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
pub enum DirtVariant{
    Dirt1,
    Dirt2,
    Dirt3,
}

impl Distribution<DirtVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DirtVariant {
        match rng.gen_range(0..6) {
            0 => DirtVariant::Dirt1,
            4 => DirtVariant::Dirt1,
            5 => DirtVariant::Dirt1,
            1 => DirtVariant::Dirt2,
            2 => DirtVariant::Dirt2,
            _ => DirtVariant::Dirt3,
        }
    }
}
  

#[derive(Clone, Debug, PartialEq)]
pub enum Pixel {
    Ground(DirtVariant), 
    Sky,
    White,
    TranslucentGrey,
    Clear,
    Rock,
    Gravel,
    Red,
}

#[derive(Component)]
pub struct ErosionColumns{
    pub columns: HashSet<usize>
}

#[derive(PartialEq)]
pub enum Tool{
    Shovel,
    Pickaxe
}

#[derive(Component, PartialEq)]
pub struct CurrentTool{
    pub tool: Tool
}