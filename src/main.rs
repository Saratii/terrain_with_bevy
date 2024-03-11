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

#[derive(Component)]
struct Cloud;

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
    let mut rng = rand::thread_rng();
    commands.spawn(Camera2dBundle::default());
    let mut raw_data: Vec<u8> = Vec::with_capacity(4 * TOTAL_PIXELS as usize);
    generate_sky(&mut raw_data);
    generate_ground(&mut raw_data);
    for _ in 0..=rng.gen_range(0..10){
        commands.spawn((
            SpriteBundle{
            texture: assets.add(generate_cloud()),
            transform: Transform { translation: Vec3 { x: rng.gen_range(-(WINDOW_WIDTH as f32)/2. ..=WINDOW_WIDTH as f32/2.) as f32, y: rng.gen_range((WINDOW_HEIGHT / 2 - 300) as f32 ..= (WINDOW_HEIGHT/2) as f32), z: 1. }, ..default()},
            ..default()},
            Cloud,
        ));
    }
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

fn generate_cloud() -> Image{
    let mut cloud_data: Vec<u8> = Vec::with_capacity(4 * CLOUD_HEIGHT as usize * CLOUD_WIDTH as usize);
    for _ in 0..CLOUD_HEIGHT as usize * CLOUD_WIDTH as usize{
        cloud_data.push(255);
        cloud_data.push(255);
        cloud_data.push(255);
        cloud_data.push(255);
    }
    Image::new(
        Extent3d {
            width: CLOUD_WIDTH as u32,
            height: CLOUD_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        cloud_data,
        TextureFormat::Rgba8UnormSrgb
    )
}

fn update(time: Res<Time>, mut c: Query<&mut Transform, With<Cloud>>) {
    for mut cloud in c.iter_mut(){
        cloud.translation.x += 10. * time.delta_seconds();
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