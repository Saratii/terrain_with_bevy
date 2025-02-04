//Cheats
pub const MAX_SHOVEL_CAPACITY: usize = 200;
pub const CURSOR_RADIUS: usize = 15 * 1;
pub const CURSOR_ORBITAL_RADIUS: f32 = 100.* 1.;

//Setup
pub const WINDOW_WIDTH: usize = 1200;
pub const WINDOW_HEIGHT: usize = 1200;
pub const CHUNK_SIZE: f32 = 600.;
pub const LIGHTING_DEMO: bool = false;

//Player
pub const PLAYER_WIDTH: usize = 20;
pub const PLAYER_HEIGHT: usize = 60;
pub const PLAYER_SPAWN_X: usize = 0;
pub const PLAYER_SPAWN_Y: usize = 500;
pub const SPAWN_PLAYER: bool = false;

//other
pub const MAX_PLAYER_SPEED: f32 = 80.;
pub const SHOW_COLLISION_BOX: bool = true;
pub const GROUND_HEIGHT: usize = (CHUNK_SIZE as f32 * 0.2) as usize;
pub const ROCK_HEIGHT: usize = (CHUNK_SIZE as f32 * 0.4) as usize;
pub const SKY_HEIGHT: usize = (CHUNK_SIZE as f32 * 0.4) as usize;
pub const MIN_CLOUD_HEIGHT: usize = 30;
pub const MAX_CLOUD_HEIGHT: usize = 80;
pub const MAX_CLOUD_SPEED: f32 = 20.;
pub const SUN_SIZE: usize = 100;
pub const MOON_SIZE: usize = 50;

//terrain gen
pub const DIRT_NOISE_SMOOTHNESS: f64 = 0.003;
pub const DIRT_VARIATION: f64 = 15.;
pub const ROCK_NOISE_SMOOTHNESS: f64 = 0.004;
pub const ROCK_VARIATION: f64 = 80.;


pub const GRAVITY: f32 = 10.;

pub const MAX_LAYERS: usize = 4;
pub const CURSOR_BORDER_WIDTH: f32 = 1.5;
pub const MIN_EROSION_HEIGHT: i32 = 3;
pub const FRICTION: f32 = 22.0;
pub const SELL_BOX_HEIGHT: usize = 16;
pub const SELL_BOX_WIDTH: usize = 40;
pub const MAX_COPPER_ORE_SPAWNS: usize = 12;
pub const COPPER_SPAWN_RADIUS: i32 = 40;
pub const ROCK_STRENGTH: usize = 500;
pub const STARTING_FOGLESS: usize = 30;
pub const SUN_SPAWN_X: usize = 0;
pub const SUN_SPAWN_Y: usize = 0;
pub const SUN_WIDTH: f32 = 63.;
pub const SUN_HEIGHT: f32 = 63.;
pub const FLASHLIGHT_RADIUS: usize = 0;
pub const SUN_ORBIT_RADIUS: f32 = WINDOW_WIDTH as f32/2. - 30.;
pub const DEFAULT_GAMMA: f32 = 0.8;
pub const PLAYER_ACCELERATION: f32 = 150.;
pub const HOE_WIDTH: usize = 3;
pub const HOE_HEIGHT: usize = 15;
pub const MAX_SUN_DECAY_DISTANCE: f32 = 1000.;
pub const SUN_SPEED: f32 = 0.09;
pub const SUN_RADIUS: f32 = 20.;
pub const SHOW_SUN: bool = false;
pub const RAY_COUNT: usize = 25;
pub const LIGHTS_PER_SUN: usize = 1;
pub const SHOW_RAYS: bool = true;
pub const NUM_BOXES_IN_TOOL_BAR: usize = 6;
pub const TOOL_BAR_BOX_SIZE: usize = 30; 
pub const RENDER_SIZE: i32 = 3;
pub const MAX_DIRT_HEIGHT_G: f64 = 0.;
pub const MAX_ROCK_HEIGHT_G: f64 = -100.; 
pub const SELL_BOX_SPAWN_X: i32 = 400;
pub const SELL_BOX_SPAWN_Y: i32 = 400;
pub const SPAWN_SELL_BOX: bool = false;