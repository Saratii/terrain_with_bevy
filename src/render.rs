// use bevy::{asset::{Assets, Handle}, prelude::{Image, Query, ResMut, With, Without}};

// use crate::{components::{DirtVariant, FogImageTag, GravelVariant, Grid, ImageBuffer, SunTag, TerrainGridTag}, constants::DEFAULT_GAMMA, sun::RayGridTag, tools::{HoeTag, PickaxeTag, ShovelTag}};

// pub fn render_grid(grid: &Vec<Pixel>, image_buffer: &mut Vec<u8>, perlin_mask: Option<&Vec<f32>>) {
//     for i in 0..grid.len() {
//         match &grid[i].pixel_type {
//             PixelType::Ground(variant) => {
//                 match variant{
//                     DirtVariant::Dirt1 => {
//                         image_buffer[4*i] = (88. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+1] = (57. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+2] = (39. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+3] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                     },
//                     DirtVariant::Dirt2 => {
//                         image_buffer[4*i] = (92. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+1] = (64. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+2] = (51. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+3] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                     },
//                     DirtVariant::Dirt3 => {
//                         image_buffer[4*i] = (155. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+1] = (118. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+2] = (83. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+3] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                     },
//                 }
//             },
//             PixelType::Gravel(variant) => {
//                 match variant{
//                     GravelVariant::Gravel1 => {
//                         image_buffer[4*i] = (115. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+1] = (115. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+2] = (115. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+3] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                     },
//                     GravelVariant::Gravel2 => {
//                         image_buffer[4*i] = (72. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+1] = (72. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+2] = (72. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+3] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                     },
//                     GravelVariant::Gravel3 => {
//                         image_buffer[4*i] = (220. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+1] = (210. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+2] = (195. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                         image_buffer[4*i+3] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                     },
//                 }
//             },
//             PixelType::Sky => {
//                 image_buffer[4*i] = (135. * grid[i].gamma) as u8;
//                 image_buffer[4*i+1] = (206. * grid[i].gamma) as u8;
//                 image_buffer[4*i+2] = (235. * grid[i].gamma) as u8;
//                 image_buffer[4*i+3] = (255. * grid[i].gamma) as u8;
//             },
//             PixelType::Clear => {
//                 image_buffer[4*i] = 0;
//                 image_buffer[4*i+1] = 0;
//                 image_buffer[4*i+2] = 0;
//                 image_buffer[4*i+3] = 0;
//             },
//             PixelType::TranslucentGrey => {
//                 image_buffer[4*i] = (135. * 1.) as u8;
//                 image_buffer[4*i+1] = (206. * 1.) as u8;
//                 image_buffer[4*i+2] = (235. * 1.) as u8;
//                 image_buffer[4*i+3] = (150. * 1.) as u8;
//             },
//             PixelType::White => {
//                 image_buffer[4*i] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+1] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+2] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+3] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//             },
//             PixelType::Rock => {
//                 image_buffer[4*i] = (100. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+1] = (100. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+2] = (100. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+3] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//             },
//             PixelType::Red => {
//                 image_buffer[4*i] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+1] = 0;
//                 image_buffer[4*i+2] = 0;
//                 image_buffer[4*i+3] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//             },
//             PixelType::SellBox => {
//                 image_buffer[4*i] = (106. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+1] = (13. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+2] = (173. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+3] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//             },
//             PixelType::RefinedCopper => {
//                 image_buffer[4*i] = (205. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+1] = (127. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+2] = (50. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+3] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//             },
//             PixelType::Black => {
//                 image_buffer[4*i] = 0;
//                 image_buffer[4*i+1] = 0;
//                 image_buffer[4*i+2] = 0;
//                 image_buffer[4*i+3] = (255. * 1.) as u8;
//             },
//             PixelType::PlayerSkin => {
//                 image_buffer[4*i] = (210. * 1.) as u8;
//                 image_buffer[4*i+1] = (180. * 1.) as u8;
//                 image_buffer[4*i+2] = (140. * 1.) as u8;
//                 image_buffer[4*i+3] = (255. * 1.) as u8;
//             },
//             PixelType::Chalcopyrite => {
//                 image_buffer[4*i] = (196. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+1] = (145. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+2] = (2. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+3] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//             },
//             PixelType::Cloud => {
//                 image_buffer[4*i] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+1] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+2] = (255. * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//                 image_buffer[4*i+3] = (255. * perlin_mask.unwrap()[i] * grid[i].gamma.max(DEFAULT_GAMMA)) as u8;
//             },
//             PixelType::Light => {
//                 image_buffer[4*i] = 255;
//                 image_buffer[4*i+1] = 255;
//                 image_buffer[4*i+2] = 0;
//                 image_buffer[4*i+3] = (255) as u8;
//             },
//             PixelType::Steel => {
//                 let gamma = grid[i].gamma.max(DEFAULT_GAMMA);
//                 image_buffer[4*i] = (176. * gamma) as u8;
//                 image_buffer[4*i+1] = (179. * gamma) as u8;
//                 image_buffer[4*i+2] = (183. * gamma) as u8;
//                 image_buffer[4*i+3] = (255. * gamma) as u8;
//             },
//         };
//     }
// }

// pub fn render_scene(
//     mut grid_query: Query<&mut Grid<Pixel>, (With<TerrainGridTag>, Without<ShovelTag>)>,
//     mut shovel_grid_query: Query<&mut Grid<Pixel>, (With<ShovelTag>, Without<TerrainGridTag>)>,
//     mut grid_image_buffer_query: Query<&mut ImageBuffer, (Without<PickaxeTag>, Without<ShovelTag>, Without<FogImageTag>, Without<SunTag>, Without<HoeTag>, Without<RayGridTag>)>,
//     mut cursor_image_buffer_query: Query<&mut ImageBuffer, With<ShovelTag>>,
//     mut images: ResMut<Assets<Image>>,
//     mut grid_image_query: Query<&Handle<Image>, With<TerrainGridTag>>,
//     mut cursor_image_query: Query<&Handle<Image>, With<ShovelTag>>,
// ) {
//     let mut grid_image_buffer = grid_image_buffer_query.get_single_mut().unwrap();
//     // let mut cursor_image_buffer = cursor_image_buffer_query.get_single_mut().unwrap();
//     // let shovel_grid = shovel_grid_query.get_single_mut().unwrap();
//     let grid = grid_query.get_single_mut().unwrap();
//     render_grid(&grid.data, &mut grid_image_buffer.data, None);
//     // render_grid(&shovel_grid.data, &mut cursor_image_buffer.data, None);
//     if let Some(image) = images.get_mut(grid_image_query.single_mut()) {
//         image.data = grid_image_buffer.data.clone();
//     }
//     // if let Some(image) = images.get_mut(cursor_image_query.single_mut()) {
//         // image.data = cursor_image_buffer.data.clone()
//     // }
// }