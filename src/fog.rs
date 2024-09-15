// use std::{collections::HashSet, f64::consts::PI};

// use bevy::{asset::{AssetServer, Assets, Handle}, math::Vec3, prelude::{default, Commands, Image, Query, Res, ResMut, Transform, With}, sprite::SpriteBundle};
// use noise::{NoiseFn, Perlin};

// use crate::{components::{FogImageTag, FogIndicesToUncover, Grid, ImageBuffer, Pixel}, constants::{GROUND_HEIGHT, ROCK_HEIGHT, SKY_HEIGHT, STARTING_FOGLESS, WINDOW_HEIGHT, WINDOW_WIDTH}, util::grid_to_image};

// pub fn setup_fog(mut commands: Commands, assets: Res<AssetServer>) {
//     let perlin_mask = generate_layered_perlin_mask();
//     let fog_grid: Vec<Pixel> = vec![Pixel::Cloud(0.); WINDOW_WIDTH * WINDOW_HEIGHT];
//     let fog_image = grid_to_image(&fog_grid, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, Some(&perlin_mask));
//     commands.spawn(Grid { data: perlin_mask })
//             .insert(
//         SpriteBundle {
//                     texture: assets.add(fog_image.clone()),
//                     transform: Transform { translation: Vec3 { x: 0., y: 0., z: 10. }, ..default()},
//                     ..default()
//                 }
//             )
//             .insert(FogImageTag)
//             .insert(FogIndicesToUncover { indices: HashSet::new() })
//             .insert(ImageBuffer { data: fog_image.data });
// }

// fn generate_layered_perlin_mask() -> Vec<f32> {
//     let mut perlin_mask = Vec::with_capacity(WINDOW_WIDTH * WINDOW_HEIGHT);
//     let perlin = Perlin::new(0);
//     let octaves = 8;
//     let persistence = 0.9;
//     let lacunarity = 2.;
//     let mut max_amplitude = 0.0;
//     let mut amplitude = 1.0;
//     let layers_to_modify = [1, 3, 4];
//     for _ in 0..octaves {
//         max_amplitude += amplitude;
//         amplitude *= persistence;
//     }
//     for y in 0..WINDOW_HEIGHT {
//         for x in 0..WINDOW_WIDTH {
//             if y < SKY_HEIGHT + STARTING_FOGLESS {
//                 perlin_mask.push(0.0);
//                 continue;
//             }
//             let mut amplitude = 1.0;
//             let mut frequency = 1.0;
//             let mut noise_value = 0.0;
//             for octave in 0..octaves {
//                 let nx = x as f64 * frequency / WINDOW_WIDTH as f64;
//                 let ny = y as f64 * frequency / (GROUND_HEIGHT + ROCK_HEIGHT) as f64;
//                 let mut noise = perlin.get([nx, ny]);
//                 if layers_to_modify.contains(&octave) {
//                     noise = (noise * PI).sin();
//                 }
//                 noise_value += noise * amplitude;
//                 amplitude *= persistence;
//                 frequency *= lacunarity;
//             }
//             let normalized_value = (noise_value / max_amplitude + 1.0) / 2.0;
//             perlin_mask.push(normalized_value as f32);
//         }
//     }
//     perlin_mask
// }

// pub fn update_fog(
//     mut fog_image_buffer_query: Query<&mut ImageBuffer, With<FogImageTag>>,
//     mut images: ResMut<Assets<Image>>,
//     mut fog_image_query: Query<&Handle<Image>, With<FogImageTag>>,
//     mut fog_indicies_to_uncover_query: Query<&mut FogIndicesToUncover>,
// ) {
//     let mut fog_image_buffer = fog_image_buffer_query.get_single_mut().unwrap();
//     let mut fog_indices_to_uncover = fog_indicies_to_uncover_query.get_single_mut().unwrap();
//     for index in fog_indices_to_uncover.indices.iter(){
//         fog_image_buffer.data[4 * index + 3] = 0;
//     }
//     fog_indices_to_uncover.indices.clear();
//     if let Some(image) = images.get_mut(fog_image_query.single_mut()) {
//         image.data = fog_image_buffer.data.clone();
//     }
// }