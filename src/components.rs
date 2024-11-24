use std::collections::HashSet;
use bevy::{prelude::Component, time::Timer};
use rand::{distributions::Standard, prelude::Distribution, Rng};

#[derive(Component, Debug)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

#[derive(Component)]
pub struct PlayerTag;

#[derive(Component)]
pub struct Bool{
    pub bool: bool
}

#[derive(Component)]
pub struct FogImageTag;

#[derive(Component)]
pub struct SellBoxTag;

#[derive(Component, Debug)]
pub struct Grid<T> {
    pub data: Vec<T>
}

#[derive(Component)]
pub struct ImageBuffer{
    pub data: Vec<u8>
}

#[derive(Component)]
pub struct F32 {
    pub f32: f32
}

#[derive(Component)]
pub struct SunTag;

#[derive(Component)]
pub struct TerrainImageTag;

#[derive(Component)]
pub struct ChunkMapTag;

#[derive(Component, Debug)]
pub struct ContentList {
    pub contents: Vec<u8>
}

#[derive(Component, Debug)]
pub struct GravityCoords {
    pub coords: HashSet<(i32, i32)>
}

#[derive(Component)]
pub struct FogIndicesToUncover {
    pub indices: HashSet<usize>
}

#[derive(Component)]
pub struct TimerComponent {
    pub timer: Timer,
}

#[derive(Component)]
pub struct SunTick {
    pub timer: Timer,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DirtVariant {
    Dirt1,
    Dirt2,
    Dirt3,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GravelVariant {
    Gravel1,
    Gravel2,
    Gravel3,
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

impl Distribution<GravelVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GravelVariant {
        match rng.gen_range(0..6) {
            0 => GravelVariant::Gravel1,
            4 => GravelVariant::Gravel2,
            5 => GravelVariant::Gravel3,
            1 => GravelVariant::Gravel1,
            2 => GravelVariant::Gravel1,
            _ => GravelVariant::Gravel1,
        }
    }
}

#[derive(Component)]
pub struct Count{
    pub count: f32,
}

#[derive(Component)]
pub struct MoneyTextTag;

#[derive(Clone, Debug, PartialEq)]
pub struct Rock {
    pub vertical_force: usize
}

#[derive(Component)]
pub struct ErosionCoords {
    pub coords: HashSet<(usize, usize)>
}

#[derive(Component)]
pub struct USize {
    pub usize: usize
}

#[derive(Component)]
pub struct ChunkMap {
    pub map: Vec<Vec<u8>>,
}