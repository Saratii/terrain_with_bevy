use std::cmp::max;
use std::cmp::min;

use bevy::app::*;
use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::window::PresentMode;
use bevy_fps_counter::FpsCounterPlugin;
use rand::Rng;


const WINDOW_WIDTH: usize = 2000;
const WINDOW_HEIGHT: usize = 1300;
const TOTAL_PIXELS: usize = WINDOW_WIDTH * WINDOW_HEIGHT;
const HEIGHT_OF_SKY_IN_PIXELS: usize = 600;
const CLOUD_HEIGHT: i32 = 50;
const CLOUD_WIDTH: i32 = 100;
struct Position{
    x: i32,
    y: i32
}
#[derive(Component)]
struct Clouds{
    positions: Vec<Position>,
}
#[derive(Component)]
struct Scene{
    clouds: Clouds
}

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
          .add_plugins(FpsCounterPlugin)
          .add_systems(Startup, setup)
          .add_systems(Update, update)
          .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let mut raw_data: Vec<u8> = Vec::with_capacity(4 * TOTAL_PIXELS as usize);
    generate_sky(&mut raw_data);
    generate_ground(&mut raw_data);
    commands.spawn(Scene{
        clouds: Clouds{positions: generate_clouds(&mut raw_data)}
    });
    let grid_data = Image::new(
        Extent3d {
            width: WINDOW_WIDTH as u32,
            height: WINDOW_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        raw_data,
        TextureFormat::Rgba8UnormSrgb 
    );
    let handle = assets.add(grid_data);
    commands.spawn(SpriteBundle {
        texture: handle,
        transform: Transform::from_xyz(0., 0., 0.1),
        ..default()
    });
}

fn update(mut q: Query<&Handle<Image>>, mut thingy: ResMut<Assets<Image>>, mut c: Query<&mut Scene>
) {
    let handle = q.single_mut();
    let image = thingy.get_mut(handle).unwrap();
    let mut scene = c.single_mut();
    move_clouds( &mut scene.clouds, &mut image.data);
}

fn move_clouds(clouds: &mut Clouds, raw_data: &mut Vec<u8>){
    let mut white_to_blue = Vec::new();
    for i in 0..clouds.positions.len(){
        for y in clouds.positions[i].y as i32..clouds.positions[i].y as i32 + CLOUD_HEIGHT{
            white_to_blue.push((clouds.positions[i].x as i32 - CLOUD_WIDTH, y));
        }
    }
    for i in 0..clouds.positions.len(){
        let cloud = &clouds.positions[i];
        for y in cloud.y as i32..cloud.y as i32 + CLOUD_HEIGHT{
            set(raw_data, index(cloud.x as i32 + CLOUD_WIDTH, y), 255, 255, 255);
        }
        let mut j = 0;
        while j < white_to_blue.len(){
            let (x, y) = white_to_blue[j];
            if y > cloud.y && y < cloud.y + CLOUD_HEIGHT &&
                    ((x <= cloud.x + CLOUD_WIDTH  && x > cloud.x - CLOUD_WIDTH) || (x + WINDOW_WIDTH as i32 <= cloud.x + CLOUD_WIDTH  && x + WINDOW_WIDTH as i32 > cloud.x - CLOUD_WIDTH)) {
                white_to_blue.remove(j);
            } else {
                j += 1;
            }
        }
        clouds.positions[i].x += 1;
        if clouds.positions[i].x == WINDOW_WIDTH as i32{
            clouds.positions[i].x = 0;
        }
    }
    for (x, y) in white_to_blue{
        set(raw_data, index(x, y), 135, 206, 235);
    }
}

fn generate_sky(raw_data: &mut Vec<u8>) {
    for _ in 0..WINDOW_WIDTH as usize * HEIGHT_OF_SKY_IN_PIXELS{
        raw_data.push(135);
        raw_data.push(206);
        raw_data.push(235);
        raw_data.push(255);
    }
}

fn generate_ground(raw_data: &mut Vec<u8>) {
    let mut rng = rand::thread_rng();
    let mut chance_of_stone = 0.;
    let start_pixel_count =  raw_data.len()/4;
    for i in start_pixel_count..TOTAL_PIXELS as usize{
        if i % WINDOW_WIDTH as usize == 0{
            chance_of_stone += 0.01;
        }
        if rng.gen_range(0..=100) as f32 <= chance_of_stone{
            raw_data.push(192);
            raw_data.push(192);
            raw_data.push(192);
        } else {
            raw_data.push(155);
            raw_data.push(118);
            raw_data.push(83);
        }
        raw_data.push(255);
    }
}

fn index(mut x: i32, y: i32) -> usize{
    if x > WINDOW_WIDTH as i32{
        x -= WINDOW_WIDTH as i32;
    } else if x < 0{
        x = WINDOW_WIDTH as i32 + x;
    }
    4 * (y * WINDOW_WIDTH as i32 + x) as usize
}

fn get(raw_data: &mut Vec<u8>, i: usize) -> (u8, u8, u8){
    (raw_data[i], raw_data[i+1], raw_data[i+2])
}

fn set(raw_data: &mut Vec<u8>, i: usize, r: u8, g: u8, b: u8){
    raw_data[i] = r;
    raw_data[i+1] = g;
    raw_data[i+2] = b;
}

fn generate_clouds(raw_data: &mut Vec<u8>) -> Vec<Position>{
    let mut rng = rand::thread_rng();
    let mut positions = vec![];
    for _ in 0..=0{
        // let top_y = rng.gen_range(0..=HEIGHT_OF_SKY_IN_PIXELS / 2) as i32;
        // let center_x = rng.gen_range(0..=WINDOW_WIDTH) as i32;
        let top_y = 100;
        let center_x = 1000;
        positions.push(Position{x: center_x, y: top_y});
        for y in top_y as usize..(top_y + CLOUD_HEIGHT) as usize{
            for x  in max(0, (center_x - CLOUD_WIDTH) as usize)..min((center_x + CLOUD_WIDTH) as usize, WINDOW_WIDTH){
                set(raw_data, index(x as i32, y as i32), 255, 255, 255)
            }
        }
    }
    for _ in 0..=0{
        // let top_y = rng.gen_range(0..=HEIGHT_OF_SKY_IN_PIXELS / 2) as i32;
        // let center_x = rng.gen_range(0..=WINDOW_WIDTH) as i32;
        let top_y = 120;
        let center_x = 1200;
        positions.push(Position{x: center_x, y: top_y});
        for y in top_y as usize..(top_y + CLOUD_HEIGHT) as usize{
            for x  in max(0, (center_x - CLOUD_WIDTH) as usize)..min((center_x + CLOUD_WIDTH) as usize, WINDOW_WIDTH){
                set(raw_data, index(x as i32, y as i32), 255, 255, 255)
            }
        }
    }
    positions
}