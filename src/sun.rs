use std::f32::consts::PI;

use bevy::{asset::{AssetServer, Assets, Handle}, math::Vec3, prelude::{default, Commands, Component, Image, Query, Res, ResMut, Transform, Visibility, With, Without}, sprite::SpriteBundle, time::Time};

use crate::{components::{Grid, SunTag, F32}, constants::{RAY_COUNT, SHOW_RAYS, SUN_HEIGHT, SUN_ORBIT_RADIUS, SUN_SPEED, SUN_WIDTH}, util::c_to_tl};

// // #[derive(Debug)]
// // pub struct Triangle {
// //     p1: (f32, f32),
// //     p2: (f32, f32),
// //     p3: (f32, f32),
// // }

// #[derive(Component)]
// pub struct RayGridTag;

// // pub fn spawn_sun(
// //     mut commands: Commands,
// //     assets: Res<AssetServer>,
// // ) {
// //     if SHOW_RAYS {
// //         let ray_grid = vec![Pixel { pixel_type: PixelType::Clear, gamma: 0. }; WINDOW_WIDTH * WINDOW_HEIGHT];
// //         let ray_image = grid_to_image(&ray_grid, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, None);
// //         commands.spawn(RayGridTag)
// //             .insert(Grid { data: ray_grid.clone() })
// //             .insert(
// //         SpriteBundle{
// //                     texture: assets.add(ray_image.clone()),
// //                     transform: Transform { translation: Vec3 { x: 0., y: 0., z: 0. }, ..default()},
// //                     ..default()
// //                 }
// //             )
// //             .insert(ImageBuffer { data: ray_image.data });
// //     }
// //     if SHOW_SUN {
// //         let sun_grid = generate_sun_grid();
// //         let sun_image = grid_to_image(&sun_grid, SUN_WIDTH as u32, SUN_HEIGHT as u32, None);
// //         commands.spawn(SunTag)
// //                 .insert(Grid { data: sun_grid })
// //                 .insert(F32 { f32: 1. })
// //                 .insert(
// //             SpriteBundle {
// //                         texture: assets.add(sun_image.clone()),
// //                         transform: Transform { translation: Vec3 { x: SUN_SPAWN_X as f32, y: SUN_SPAWN_Y as f32, z: 10. }, ..default() },
// //                         ..default()
// //                     }
// //                 )
// //                 .insert(ImageBuffer { data: sun_image.data });
// //     } else {
// //         commands.spawn(SunTag)
// //                 .insert(F32 { f32: 1. })
// //                 .insert(Transform { translation: Vec3 { x: SUN_SPAWN_X as f32, y: SUN_SPAWN_Y as f32, z: 10. }, rotation: Default::default(), scale: Vec3::splat(1.) });
// //     }
// // }

// // fn generate_sun_grid() -> Vec<Pixel> {
// //     let mut data_buffer: Vec<Pixel> = vec![Pixel { pixel_type: PixelType::Clear, gamma: 0. }; (SUN_WIDTH * SUN_HEIGHT) as usize];
// //     let angle_step = 2. * PI / RAY_COUNT as f32;
// //     for i in 0..RAY_COUNT {
// //         let mut ray_x = SUN_WIDTH as f32/2.;
// //         let mut ray_y = SUN_HEIGHT as f32/2.;
// //         let angle = i as f32 * angle_step;
// //         let dx = angle.cos();
// //         let dy = angle.sin();
// //         loop {
// //             ray_x += dx;
// //             ray_y += dy;
// //             let index = flatten_index_standard_grid(&(ray_x as usize), &(ray_y as usize), SUN_WIDTH as usize);
// //             if ray_x < 0. || ray_x >= SUN_WIDTH as f32 || ray_y < 0. || ray_y >= SUN_HEIGHT as f32 || distance(ray_x as i32, ray_y as i32, SUN_WIDTH as i32/2, SUN_HEIGHT as i32/2) > SUN_WIDTH as f32/2. {
// //                 break
// //             }
// //             data_buffer[index].pixel_type = PixelType::Light;
// //             data_buffer[index].gamma = 1.;
// //         }
// //     }
// //     data_buffer
// // }

// // #[inline(always)]
// // fn sign_area(p1: &(f32, f32), p2: &(f32, f32), p3: &(f32, f32)) -> i32 {
// //     let dx1 = p2.0 as i32 - p1.0 as i32;
// //     let dy1 = p2.1 as i32 - p1.1 as i32;
// //     let dx2 = p3.0 as i32 - p1.0 as i32;
// //     let dy2 = p3.1 as i32 - p1.1 as i32;
// //     dx1 * dy2 - dy1 * dx2
// // }

// // fn is_point_in_triangle(triangle: &Triangle, point: &(f32, f32)) -> bool {
// //     let area1 = sign_area(&triangle.p1, &triangle.p2, point);
// //     let area2 = sign_area(&triangle.p2, &triangle.p3, point);
// //     let area3 = sign_area(&triangle.p3, &triangle.p1, point);
// //     (area1 >= 0 && area2 >= 0 && area3 >= 0) || (area1 <= 0 && area2 <= 0 && area3 <= 0)
// // }

// // pub fn cast_rays(grid: &mut Grid<Pixel>, ray_count: usize, light_source_tl: (f32, f32), ray_grid: &mut Option<&mut Vec<Pixel>>) {
// //     let angle_step = 2. * PI / ray_count as f32;
// //     let mut triangles = Vec::with_capacity(ray_count - 1);
// //     let mut last_ray: (f32, f32) = (1000000., 1000000.);
// //     for i in 0..ray_count {
// //         let mut ray_x = light_source_tl.0 + SUN_WIDTH as f32/2.;
// //         let mut ray_y = light_source_tl.1 + SUN_HEIGHT as f32/2.;
// //         let angle = i as f32 * angle_step;
// //         let dx = angle.cos();
// //         let dy = angle.sin();
// //         loop {
// //             ray_x += dx;
// //             ray_y += dy;
// //             let index = flatten_index_standard_grid(&(ray_x as usize), &(ray_y as usize), WINDOW_WIDTH);
// //             if ray_x < 0. || ray_x >= WINDOW_WIDTH as f32 || ray_y < 0. || ray_y >= WINDOW_HEIGHT as f32 || !matches!(grid.data[index].pixel_type, PixelType::Sky | PixelType::Light){
// //                 if i > 0 {
// //                     triangles.push(Triangle { 
// //                         p1: (light_source_tl.0 + SUN_WIDTH as f32/2., light_source_tl.1 + SUN_HEIGHT as f32/2.),
// //                         p2: (ray_x, ray_y),
// //                         p3: (last_ray.0, last_ray.1),
// //                     });
// //                 }
// //                 last_ray = (ray_x, ray_y);
// //                 break
// //             }
// //             match ray_grid {
// //                 Some(ray_grid) => {
// //                     ray_grid[index].pixel_type = PixelType::Light;
// //                     ray_grid[index].gamma = 1.;
// //                 },
// //                 None => {}
// //             }
// //         }
// //     }
// //     triangles.push(Triangle {
// //         p1: (light_source_tl.0 + SUN_WIDTH as f32/2., light_source_tl.1 + SUN_HEIGHT as f32/2.),
// //         p2: (last_ray.0, last_ray.1),
// //         p3: triangles[0].p3,
// //     });
// //     for triangle in triangles {
// //         light_triangle(&triangle, &mut grid.data);
// //     }
// // }

// // fn reset_gamma(grid: &mut Grid<Pixel>) {
// //     for pixel in &mut grid.data {
// //         match pixel.pixel_type {
// //             PixelType::Light | PixelType::Black | PixelType::PlayerSkin | PixelType::TranslucentGrey => pixel.gamma = 1.0,
// //             _ => pixel.gamma = 0.0,
// //         }
// //     }
// // }

// // fn light_triangle(triangle: &Triangle, grid: &mut Vec<Pixel>) {
// //     let min_x = triangle.p1.0.min(triangle.p2.0).min(triangle.p3.0) as usize;
// //     let max_x = triangle.p1.0.max(triangle.p2.0).max(triangle.p3.0) as usize;
// //     let min_y = triangle.p1.1.min(triangle.p2.1).min(triangle.p3.1) as usize;
// //     let max_y = triangle.p1.1.max(triangle.p2.1).max(triangle.p3.1).min(WINDOW_HEIGHT as f32) as usize;
// //     for y in min_y..max_y {
// //         for x in min_x..max_x {
// //             if is_point_in_triangle(&triangle, &(x as f32, y as f32)) {
// //                 let distance = distance(x as i32, y as i32, triangle.p1.0 as i32, triangle.p1.1 as i32);
// //                 let gamma = if distance < MAX_SUN_DECAY_DISTANCE {
// //                     1.0 - (distance / MAX_SUN_DECAY_DISTANCE)
// //                 } else {
// //                     0.0
// //                 };
// //                 grid[flatten_index_standard_grid(&x, &y, WINDOW_WIDTH)].gamma = gamma;
// //             }
// //         }
// //     }
// // }

// // fn reset_ray_grid(grid: &mut Vec<Pixel>) {
// //     for pixel in grid {
// //         pixel.pixel_type = PixelType::Clear;
// //     }
// // }

// pub fn move_sun(
//     mut sun_query: Query<&mut Transform, With<SunTag>>,
//     mut sun_theta_query: Query<&mut F32, With<SunTag>>,
//     time: Res<Time>,
// ) {
//     let mut sun_transform = sun_query.single_mut();
//     let mut sun_theta = sun_theta_query.single_mut();
//     sun_theta.f32 += SUN_SPEED * time.delta_seconds();
//     sun_transform.translation.x = SUN_ORBIT_RADIUS * sun_theta.f32.cos();
//     sun_transform.translation.y = SUN_ORBIT_RADIUS * sun_theta.f32.sin();
// }

// pub fn lighting_update(
//     mut grid_query: Query<&mut Grid<Pixel>, (With<TerrainGridTag>, Without<RayGridTag>)>,
//     mut ray_grid_query: Query<&mut Grid<Pixel>, (With<RayGridTag>, Without<TerrainGridTag>)>,
//     sun_position_query: Query<&Transform, With<SunTag>>,
// ) {
//     let mut grid = grid_query.get_single_mut().unwrap();
//     reset_gamma(&mut grid);
//     let sun_position_c = sun_position_query.single();
//     let sun_position_tl = c_to_tl(&sun_position_c.translation, SUN_WIDTH, SUN_HEIGHT);
//     if SHOW_RAYS {
//         let mut ray_grid = ray_grid_query.get_single_mut().unwrap();
//         reset_ray_grid(&mut ray_grid.data);
//         cast_rays(&mut grid, RAY_COUNT, (sun_position_tl.0, sun_position_tl.1), &mut Some(&mut ray_grid.data));
//     } else {
//         cast_rays(&mut grid, RAY_COUNT, (sun_position_tl.0, sun_position_tl.1), &mut None);
//     }
// }

// // pub fn render_rays(
// //     mut ray_grid_query: Query<&mut Grid<Pixel>, With<RayGridTag>>,
// //     mut ray_image_buffer_query: Query<&mut ImageBuffer, With<RayGridTag>>,
// //     mut images: ResMut<Assets<Image>>,
// //     mut ray_image_query: Query<&Handle<Image>, With<RayGridTag>>,
// // ) {
// //     if SHOW_RAYS {
// //         let ray_grid = ray_grid_query.get_single_mut().unwrap();
// //         let mut ray_image_buffer = ray_image_buffer_query.get_single_mut().unwrap();
// //         render_grid(&ray_grid.data, &mut ray_image_buffer.data, None);
// //         if let Some(image) = images.get_mut(ray_image_query.single_mut()) {
// //             image.data = ray_image_buffer.data.clone();
// //         }
// //     }
// // }

// use bevy::{asset::Handle, ecs::{query::With, system::Query}, render::camera::Camera, transform::components::{GlobalTransform, Transform}, window::{PrimaryWindow, Window}};

// use crate::{components::{HeightMap, HeightMapTextureTag}, world_generation::{CameraTag, GridMaterial}};

// pub fn lighting_update(
//     height_map_query: Query<&HeightMap>,
//     height_map_texture: Query<(&Handle<GridMaterial>, &mut Transform), With<HeightMapTextureTag>>,
//     q_windows: Query<&Window, With<PrimaryWindow>>,
//     camera_query: Query<&GlobalTransform, With<CameraTag>>,
// ) {
//     let camera_transform = camera_query.single().translation();
//     println!("{:?}", camera_transform);
    
// }