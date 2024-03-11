use std::f32::consts::PI;

use bevy::app::*;
use bevy::prelude::*;
use bevy::render::camera::NormalizedRenderTarget;
use bevy::render::render_resource::*;
use bevy::window::PresentMode;
use bevy_fps_counter::FpsCounterPlugin;
use rand::Rng;


const WINDOW_WIDTH: usize = 2000;
const WINDOW_HEIGHT: f32 = 1300.;
const SKY_HEIGHT: f32 = 600.;
const GROUND_HEIGHT: f32 = WINDOW_HEIGHT - SKY_HEIGHT;
const MIN_CLOUD_HEIGHT: usize = 30;
const MAX_CLOUD_HEIGHT: usize = 80;
const MAX_CLOUD_SPEED: f32 = 20.;
const SUN_SIZE: usize = 100;
const MOON_SIZE: usize = 50;

#[derive(Component)]
struct Cloud;

#[derive(Component)]
struct Sky;

#[derive(Component)]
struct Sun;

#[derive(Component)]
struct Moon;

#[derive(Component)]
struct Speed{
    speed: Vec2
}

#[derive(Component)]
struct Angle{
    angle: f32
}

#[derive(Component)]
struct Size{
    size: Vec2
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
    let mut rng = rand::thread_rng();
    commands.spawn(Camera2dBundle::default());
    let sky_image = generate_sky();
    let (sun_image, moon_image, angle) = generate_sun_moon();
    commands.spawn((
        SpriteBundle{
        texture: assets.add(sun_image),
        transform: Transform { translation: Vec3 { x: 1000. * f32::cos(angle.angle.clone()), y: 650. * f32::sin(angle.angle.clone()), z: 1. }, ..default()},
        ..default()},
        Sun,
        Angle{angle: angle.angle.clone()},
    ));
    commands.spawn((
        SpriteBundle{
        texture: assets.add(moon_image),
        transform: Transform { translation: Vec3 { x: 1000. * f32::cos(angle.angle.clone() + PI), y: 650. * f32::sin(angle.angle.clone() + PI), z: 1. }, ..default()},
        ..default()},
        Moon,
        angle,
    ));
    for _ in 0..=rng.gen_range(0..13){
        let (cloud_image, speed, size) = generate_cloud();
        commands.spawn((
            SpriteBundle{
            texture: assets.add(cloud_image),
            transform: Transform { translation: Vec3 { x: rng.gen_range(-(WINDOW_WIDTH as f32)/2. ..=WINDOW_WIDTH as f32/2.) as f32, y: rng.gen_range((WINDOW_HEIGHT / 2. - 300.)..= WINDOW_HEIGHT / 2.), z: 1. }, ..default()},
            ..default()},
            Cloud,
            speed,
            size,
        ));
    }
    
    commands.spawn((SpriteBundle {
        texture: assets.add(sky_image),
        transform: Transform::from_xyz(0., (WINDOW_HEIGHT - SKY_HEIGHT) as f32/2., -2.),
        ..default()
    },
    Sky
    ));

    let ground_image = generate_ground();
    commands.spawn(SpriteBundle {
        texture: assets.add(ground_image),
        transform: Transform::from_xyz(0., (-(WINDOW_HEIGHT as f32) + GROUND_HEIGHT as f32)/2., 3.),
        ..default()
    });
}

fn generate_sun_moon() -> (Image, Image, Angle){
    let mut sun_data: Vec<u8> = Vec::with_capacity(4 * SUN_SIZE * SUN_SIZE);
    for _ in 0..SUN_SIZE * SUN_SIZE{
        sun_data.push(255);
        sun_data.push(255);
        sun_data.push(102);
        sun_data.push(255);
    }
    let mut moon_data: Vec<u8> = Vec::with_capacity(4 * MOON_SIZE * MOON_SIZE);
    for _ in 0..MOON_SIZE * MOON_SIZE{
        moon_data.push(255);
        moon_data.push(255);
        moon_data.push(255);
        moon_data.push(255);
    }
    (
        Image::new(
            Extent3d {
                width: SUN_SIZE as u32,
                height: SUN_SIZE as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            sun_data,
            TextureFormat::Rgba8UnormSrgb
        ),
        Image::new(
            Extent3d {
                width: MOON_SIZE as u32,
                height: MOON_SIZE as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            moon_data,
            TextureFormat::Rgba8UnormSrgb
        ),
        Angle{angle: 0.}
        )
}

fn generate_cloud() -> (Image, Speed, Size){
    let mut rng = rand::thread_rng();
    let cloud_height = rng.gen_range(MIN_CLOUD_HEIGHT..MAX_CLOUD_HEIGHT);
    let cloud_width = cloud_height*2;
    let normalized = (cloud_height as f32 - MIN_CLOUD_HEIGHT as f32)/(MAX_CLOUD_HEIGHT - MIN_CLOUD_HEIGHT) as f32;
    let mut cloud_data: Vec<u8> = Vec::with_capacity(4 * cloud_height * cloud_width);
    let transparency = 255. * normalized;
    for _ in 0..cloud_height * cloud_width{
        cloud_data.push(255);
        cloud_data.push(255);
        cloud_data.push(255);
        cloud_data.push(transparency as u8);
    }
    (
    Image::new(
        Extent3d {
            width: cloud_width as u32,
            height: cloud_height as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        cloud_data,
        TextureFormat::Rgba8UnormSrgb
    ),
        Speed{speed: Vec2 { x: normalized * MAX_CLOUD_SPEED, y: 0.}},
        Size{size: Vec2 {x: cloud_width as f32, y: cloud_height as f32}}
    )
}

fn update(time: Res<Time>, mut c: Query<(&mut Transform, &Speed, &Size), (With<Cloud>, Without<Sun>, Without<Moon>)>,
mut s: Query<(&mut Transform, &mut Angle), (With<Sun>, Without<Cloud>, Without<Moon>)>,
mut m: Query<&mut Transform, (With<Moon>, Without<Cloud>, Without<Sun>)>,
mut sky_query: Query<&mut Sprite, (Or<(With<Cloud>, With<Sky>)>, Without<Moon>, Without<Sun>)>
){
    for (mut cloud, speed, size) in c.iter_mut(){
        cloud.translation.x += speed.speed.x * time.delta_seconds();
        if cloud.translation.x - size.size.x/2.> (WINDOW_WIDTH / 2) as f32{
            cloud.translation.x *= -1.;
        }
    }
    let (mut sun, mut angle) = s.single_mut();
    let mut moon = m.single_mut();
    angle.angle += 0.03 * time.delta_seconds();
    if angle.angle > 2. * PI{
        angle.angle = 0.;
    }
    sun.translation.x = 1000. * f32::cos(angle.angle);
    sun.translation.y = 650. * f32::sin(angle.angle);
    moon.translation.x = 1000. * f32::cos(angle.angle + PI);
    moon.translation.y = 650. * f32::sin(angle.angle + PI);

    let normalized_theta = 0.5+f32::sin(angle.angle)/2.;
    for mut cloud_or_sky in sky_query.iter_mut(){
        cloud_or_sky.color = Color::Rgba { red: 1., green: 1., blue: 1., alpha: normalized_theta}
    }

}

fn generate_sky() -> Image {
    let mut data_buffer: Vec<u8> = Vec::with_capacity(4 * WINDOW_WIDTH as usize * SKY_HEIGHT as usize);
    for _ in 0..WINDOW_WIDTH as usize * SKY_HEIGHT as usize{
        data_buffer.push(135);
        data_buffer.push(206);
        data_buffer.push(235);
        data_buffer.push(255);
    }
    Image::new(
        Extent3d {
            width: WINDOW_WIDTH as u32,
            height: SKY_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data_buffer,
        TextureFormat::Rgba8UnormSrgb 
    )
}

fn generate_ground() -> Image{
    let mut rng = rand::thread_rng();
    let mut chance_of_stone = 0.;
    let mut chance_of_grass = 0.;
    let mut data_buffer: Vec<u8> = Vec::with_capacity(4 * GROUND_HEIGHT as usize * WINDOW_WIDTH as usize);
    for i in 0..GROUND_HEIGHT as usize * WINDOW_WIDTH as usize{
        if i % WINDOW_WIDTH as usize == 0{
            chance_of_stone += 0.01;
            chance_of_grass += 0.6;
        }
        if rng.gen_range(0..=100) as f32 >= chance_of_grass{
            data_buffer.push(38);
            data_buffer.push(139);
            data_buffer.push(7);
        } else if rng.gen_range(0..=100) as f32 <= chance_of_stone{
            data_buffer.push(192);
            data_buffer.push(192);
            data_buffer.push(192);
        } else {
            data_buffer.push(155);
            data_buffer.push(118);
            data_buffer.push(83);
        }
        data_buffer.push(255);
    }
    Image::new(
        Extent3d {
            width: WINDOW_WIDTH as u32,
            height: GROUND_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data_buffer,
        TextureFormat::Rgba8UnormSrgb 
    )
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