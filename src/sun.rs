use std::f32::consts::PI;

use bevy::{asset::AssetServer, math::Vec3, prelude::{default, Commands, Query, Res, Transform, With}, sprite::SpriteBundle};

use crate::{components::{Grid, ImageBuffer, Pixel, PixelType, SunTag, TerrainGridTag}, constants::{FLASHLIGHT_RADIUS, RAY_COUNT, SUN_HEIGHT, SUN_SPAWN_X, SUN_SPAWN_Y, SUN_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH}, util::{distance, flatten_index_standard_grid, grid_to_image, tl_to_c}};

#[derive(Debug)]
struct Triangle {
    p1: (usize, usize),
    p2: (usize, usize),
    p3: (usize, usize),
}

pub fn start_sun(
    mut grid_query: Query<&mut Grid<Pixel>, With<TerrainGridTag>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let mut grid = grid_query.get_single_mut().unwrap();
    ray_cast(&mut grid, RAY_COUNT, (SUN_SPAWN_X, SUN_SPAWN_Y));
    let sun_grid = generate_sun_grid();
    let sun_image = grid_to_image(&sun_grid, SUN_WIDTH as u32, SUN_HEIGHT as u32, None);
    let sun_spawn_c = tl_to_c(SUN_SPAWN_X as f32, SUN_SPAWN_Y as f32, SUN_WIDTH as f32, SUN_HEIGHT as f32);
    commands.spawn(SunTag)
            .insert(Grid{data: sun_grid})
            .insert(
        SpriteBundle{
                    texture: assets.add(sun_image.clone()),
                    transform: Transform { translation: Vec3 { x: sun_spawn_c.0 - SUN_WIDTH as f32 / 2., y: sun_spawn_c.1 as f32 + SUN_HEIGHT as f32 / 2., z: 10. }, ..default()},
                    ..default()
                }
            )
            .insert(ImageBuffer { data: sun_image.data });
}

fn generate_sun_grid() -> Vec<Pixel> {
    let mut data_buffer: Vec<Pixel> = vec![Pixel { pixel_type: PixelType::Clear, gamma: 0. }; SUN_WIDTH * SUN_HEIGHT];
    let angle_step = 2. * PI / RAY_COUNT as f32;
    for i in 0..RAY_COUNT {
        let mut ray_x = SUN_WIDTH as f32/2.;
        let mut ray_y = SUN_HEIGHT as f32/2.;
        let angle = i as f32 * angle_step;
        let dx = angle.cos();
        let dy = angle.sin();
        loop {
            ray_x += dx;
            ray_y += dy;
            let index = flatten_index_standard_grid(&(ray_x as usize), &(ray_y as usize), SUN_WIDTH);
            if ray_x < 0. || ray_x >= SUN_WIDTH as f32 || ray_y < 0. || ray_y >= SUN_HEIGHT as f32 || distance(ray_x as i32, ray_y as i32, SUN_WIDTH as i32/2, SUN_HEIGHT as i32/2) > SUN_WIDTH as f32/2. {
                break
            }
            data_buffer[index].pixel_type = PixelType::Light;
            data_buffer[index].gamma = 1.;
        }
    }
    data_buffer
}

#[inline(always)]
fn sign_area(p1: &(usize, usize), p2: &(usize, usize), p3: &(usize, usize)) -> i32 {
    let dx1 = p2.0 as i32 - p1.0 as i32;
    let dy1 = p2.1 as i32 - p1.1 as i32;
    let dx2 = p3.0 as i32 - p1.0 as i32;
    let dy2 = p3.1 as i32 - p1.1 as i32;
    dx1 * dy2 - dy1 * dx2
}

fn is_point_in_triangle(triangle: &Triangle, point: &(usize, usize)) -> bool {
    let area1 = sign_area(&triangle.p1, &triangle.p2, point);
    let area2 = sign_area(&triangle.p2, &triangle.p3, point);
    let area3 = sign_area(&triangle.p3, &triangle.p1, point);
    (area1 >= 0 && area2 >= 0 && area3 >= 0) || (area1 <= 0 && area2 <= 0 && area3 <= 0)
}

pub fn ray_cast(grid: &mut Grid<Pixel>, ray_count: usize, light_source: (usize, usize)) {
    reset_gamma(grid);
    let angle_step = 2. * PI / ray_count as f32;
    let mut triangles = Vec::with_capacity(ray_count - 1);
    let mut last_ray: (usize, usize) = (1000000, 1000000);
    for i in 0..ray_count {
        let mut ray_x = light_source.0 as f32;
        let mut ray_y = light_source.1 as f32;
        let angle = i as f32 * angle_step;
        let dx = angle.cos();
        let dy = angle.sin();
        loop {
            ray_x += dx;
            ray_y += dy;
            let index = flatten_index_standard_grid(&(ray_x as usize), &(ray_y as usize), WINDOW_WIDTH);
            if ray_x < 0. || ray_x >= WINDOW_WIDTH as f32 || ray_y < 0. || ray_y >= WINDOW_HEIGHT as f32 || !matches!(grid.data[index].pixel_type, PixelType::Sky | PixelType::Light){
                for _ in 0..FLASHLIGHT_RADIUS {
                    if ray_x > 0. && ray_x < WINDOW_WIDTH as f32 && ray_y > 0. && ray_y < WINDOW_HEIGHT as f32 {
                        ray_x += dx;
                        ray_y += dy;
                        // let new_index = flatten_index_standard_grid(&(ray_x as usize), &(ray_y as usize), WINDOW_WIDTH);
                        // grid.data[new_index].pixel_type = PixelType::Light;
                        // grid.data[new_index].gamma = 1.;
                    } else {
                        break
                    }
                }
                if i > 0 {
                    triangles.push(Triangle { 
                        p1: light_source,
                        p2: (ray_x as usize, ray_y as usize),
                        p3: (last_ray.0, last_ray.1),
                    });
                }
                last_ray = (ray_x as usize, ray_y as usize);
                break
            }
        //    grid.data[index].pixel_type = PixelType::Light;
        //    grid.data[index].gamma = 1.;
        }
    }
    triangles.push(Triangle {
        p1: light_source,
        p2: (last_ray.0, last_ray.1),
        p3: triangles[0].p3,
    });
    for y in 0..WINDOW_HEIGHT {
        for x in 0..WINDOW_WIDTH {
            for triangle in &triangles {
                if is_point_in_triangle(&triangle, &(x, y)) {
                    grid.data[flatten_index_standard_grid(&x, &y, WINDOW_WIDTH)].gamma = 1.;
                    break
                }
            }
        }
    }
}

fn reset_gamma(grid: &mut Grid<Pixel>) {
    for pixel in &mut grid.data {
        match pixel.pixel_type {
            PixelType::Light | PixelType::Black | PixelType::PlayerSkin | PixelType::TranslucentGrey => pixel.gamma = 1.0,
            _ => pixel.gamma = 0.0,
        }
    }
}