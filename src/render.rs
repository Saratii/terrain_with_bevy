use bevy::{asset::{Assets, Handle}, math::Vec3, prelude::{Image, Query, ResMut, Transform, With, Without}};

use crate::{components::{ChunkMap, PlayerTag, TerrainImageTag}, constants::{CHUNKS_HORIZONTAL, CHUNK_SIZE, WINDOW_WIDTH}, util::{flatten_index_standard_grid, get_chunk_x_g, get_chunk_x_v, get_chunk_y_g, get_chunk_y_v}, world_generation::{CameraTag, GridMaterial}};

pub fn render(
    mut materials: ResMut<Assets<GridMaterial>>,
    mut terrain_material_handle: Query<(&Handle<GridMaterial>, &mut Transform), (With<TerrainImageTag>, Without<PlayerTag>)>,
    mut images: ResMut<bevy::asset::Assets<Image>>,
    player_query: Query<&Transform, (With<PlayerTag>, Without<TerrainImageTag>, Without<CameraTag>)>,
    chunk_map_query: Query<&ChunkMap>,
    mut camera_query: Query<&mut Transform, (With<CameraTag>, Without<PlayerTag>, Without<TerrainImageTag>)>,
) {
    let mut i = 0;
    let chunk_map = &chunk_map_query.get_single().unwrap().map;
    let player_pos = player_query
        .get_single()
        .map(|player| player.translation)
        .unwrap_or(Vec3::ZERO);    let mut camera_transform = camera_query.get_single_mut().unwrap();
    camera_transform.translation = player_pos;
    let chunk_x_g = get_chunk_x_g(player_pos.x);
    let chunk_y_g = get_chunk_y_g(player_pos.y);
    let chunk_x_v = get_chunk_x_v(chunk_x_g);
    let chunk_y_v = get_chunk_y_v(chunk_y_g);
    for (material_handle, mut rendered_box_transform) in terrain_material_handle.iter_mut() {
        let grid = &mut images.get_mut(&materials.get_mut(material_handle).unwrap().color_map).unwrap().data;
        if i == 0 { //top left
            *grid = chunk_map[flatten_index_standard_grid(&(chunk_x_v - 1), &(chunk_y_v + 1), CHUNKS_HORIZONTAL as usize)].clone();
            rendered_box_transform.translation.x = (chunk_x_g as f32 - 1.) * CHUNK_SIZE;
            rendered_box_transform.translation.y = (chunk_y_g as f32 + 1.) * CHUNK_SIZE;
        } else if i == 1 { //top center
            *grid = chunk_map[flatten_index_standard_grid(&chunk_x_v, &(chunk_y_v + 1), CHUNKS_HORIZONTAL as usize)].clone();
            rendered_box_transform.translation.x = chunk_x_g as f32 * CHUNK_SIZE;
            rendered_box_transform.translation.y = (chunk_y_g as f32 + 1.) * CHUNK_SIZE;
        } else if i == 2 { //top left
            *grid = chunk_map[flatten_index_standard_grid(&(chunk_x_v + 1), &(chunk_y_v + 1), CHUNKS_HORIZONTAL as usize)].clone();
            rendered_box_transform.translation.x = (chunk_x_g as f32 + 1.) * CHUNK_SIZE;
            rendered_box_transform.translation.y = (chunk_y_g as f32 + 1.) * CHUNK_SIZE;
        } else if i == 3 { //center left
            *grid = chunk_map[flatten_index_standard_grid(&(chunk_x_v - 1), &chunk_y_v, CHUNKS_HORIZONTAL as usize)].clone();
            rendered_box_transform.translation.x = (chunk_x_g as f32 - 1.) * CHUNK_SIZE;
            rendered_box_transform.translation.y = chunk_y_g as f32 * CHUNK_SIZE;
        } else if i == 4 { //center center
            *grid = chunk_map[flatten_index_standard_grid(&chunk_x_v, &chunk_y_v, CHUNKS_HORIZONTAL as usize)].clone();
            rendered_box_transform.translation.x = chunk_x_g as f32 * CHUNK_SIZE;
            rendered_box_transform.translation.y = chunk_y_g as f32 * CHUNK_SIZE;
        } else if i == 5 { //center right
            *grid = chunk_map[flatten_index_standard_grid(&(chunk_x_v + 1), &chunk_y_v, CHUNKS_HORIZONTAL as usize)].clone();
            rendered_box_transform.translation.x = (chunk_x_g as f32 + 1.) * CHUNK_SIZE;
            rendered_box_transform.translation.y = chunk_y_g as f32 * CHUNK_SIZE;
        } else if i == 6 { //bottom left
            *grid = chunk_map[flatten_index_standard_grid(&(chunk_x_v - 1), &(chunk_y_v - 1), CHUNKS_HORIZONTAL as usize)].clone();
            rendered_box_transform.translation.x = (chunk_x_g as f32 - 1.) * CHUNK_SIZE;
            rendered_box_transform.translation.y = (chunk_y_g as f32 - 1.) * CHUNK_SIZE;
        } else if i == 7 { //bottom center
            *grid = chunk_map[flatten_index_standard_grid(&chunk_x_v, &(chunk_y_v - 1), CHUNKS_HORIZONTAL as usize)].clone();
            rendered_box_transform.translation.x = chunk_x_g as f32 * CHUNK_SIZE;
            rendered_box_transform.translation.y = (chunk_y_g as f32 - 1.) * CHUNK_SIZE;
        } else if i == 8 { //bottom right
            *grid = chunk_map[flatten_index_standard_grid(&(chunk_x_v + 1), &(chunk_y_v - 1), CHUNKS_HORIZONTAL as usize)].clone();
            rendered_box_transform.translation.x = (chunk_x_g as f32 + 1.) * CHUNK_SIZE;
            rendered_box_transform.translation.y = (chunk_y_g as f32 - 1.) * CHUNK_SIZE;
        }
        i += 1;
    }
}