use bevy::{asset::{Assets, Handle}, ecs::event::EventWriter, math::Vec3, prelude::{Image, Query, ResMut, Transform, With, Without}};

use crate::{chunk_generator::NewChunkEvent, components::{ChunkMap, PlayerTag, TerrainImageTag}, constants::CHUNK_SIZE, util::{get_chunk_x_g, get_chunk_y_g}, world_generation::{seed_chunk_with_ore, CameraTag, GridMaterial}};

pub fn render(
    mut materials: ResMut<Assets<GridMaterial>>,
    mut terrain_material_handle: Query<(&Handle<GridMaterial>, &mut Transform), (With<TerrainImageTag>, Without<PlayerTag>)>,
    mut images: ResMut<bevy::asset::Assets<Image>>,
    player_query: Query<&Transform, (With<PlayerTag>, Without<TerrainImageTag>, Without<CameraTag>)>,
    mut chunk_map_query: Query<&mut ChunkMap>,
    mut camera_query: Query<&mut Transform, (With<CameraTag>, Without<PlayerTag>, Without<TerrainImageTag>)>,
    mut chunk_event_writer: EventWriter<NewChunkEvent>,
) {
    let mut i = 0;
    let mut chunk_map = &mut chunk_map_query.get_single_mut().unwrap().map;
    let player_pos = player_query
        .get_single()
        .map(|player| player.translation)
        .unwrap_or(Vec3::ZERO);    let mut camera_transform = camera_query.get_single_mut().unwrap();
    camera_transform.translation = player_pos;
    let chunk_x_g = get_chunk_x_g(player_pos.x as i32);
    let chunk_y_g = get_chunk_y_g(player_pos.y as i32);
    for (material_handle, mut rendered_box_transform) in terrain_material_handle.iter_mut() {
        let grid = &mut images.get_mut(&materials.get_mut(material_handle).unwrap().color_map).unwrap().data;
        if i == 0 { //top left
            if let Some(chunk) = chunk_map.get(&(chunk_x_g - 1, chunk_y_g + 1)) {
                *grid = chunk.clone();
            } else {
                chunk_event_writer.send(NewChunkEvent {
                    chunk_x_g: chunk_x_g - 1,
                    chunk_y_g: chunk_y_g + 1,
                });
                seed_chunk_with_ore((chunk_x_g - 1, chunk_y_g + 1), &mut chunk_map);
            }
            rendered_box_transform.translation.x = (chunk_x_g as f32 - 1.) * CHUNK_SIZE;
            rendered_box_transform.translation.y = (chunk_y_g as f32 + 1.) * CHUNK_SIZE;
        } else if i == 1 { //top center
            if let Some(chunk) = chunk_map.get(&(chunk_x_g, chunk_y_g + 1)) {
                *grid = chunk.clone();
            } else {
                chunk_event_writer.send(NewChunkEvent {
                    chunk_x_g: chunk_x_g,
                    chunk_y_g: chunk_y_g + 1,
                });
                seed_chunk_with_ore((chunk_x_g, chunk_y_g + 1), &mut chunk_map);
            }
            rendered_box_transform.translation.x = chunk_x_g as f32 * CHUNK_SIZE;
            rendered_box_transform.translation.y = (chunk_y_g as f32 + 1.) * CHUNK_SIZE;
        } else if i == 2 { //top right
            if let Some(chunk) = chunk_map.get(&(chunk_x_g + 1, chunk_y_g + 1)) {
                *grid = chunk.clone();
            } else {
                chunk_event_writer.send(NewChunkEvent {
                    chunk_x_g: chunk_x_g + 1,
                    chunk_y_g: chunk_y_g + 1,
                });
                seed_chunk_with_ore((chunk_x_g + 1, chunk_y_g + 1), &mut chunk_map);
            }
            rendered_box_transform.translation.x = (chunk_x_g as f32 + 1.) * CHUNK_SIZE;
            rendered_box_transform.translation.y = (chunk_y_g as f32 + 1.) * CHUNK_SIZE;
        } else if i == 3 { //center left
            if let Some(chunk) = chunk_map.get(&(chunk_x_g - 1, chunk_y_g)) {
                *grid = chunk.clone();
            } else {
                chunk_event_writer.send(NewChunkEvent {
                    chunk_x_g: chunk_x_g - 1,
                    chunk_y_g: chunk_y_g,
                });
                seed_chunk_with_ore((chunk_x_g - 1, chunk_y_g), &mut chunk_map);
            }
            rendered_box_transform.translation.x = (chunk_x_g as f32 - 1.) * CHUNK_SIZE;
            rendered_box_transform.translation.y = chunk_y_g as f32 * CHUNK_SIZE;
        } else if i == 4 { //center center
            if let Some(chunk) = chunk_map.get(&(chunk_x_g, chunk_y_g)) {
                *grid = chunk.clone();
            } else {
                chunk_event_writer.send(NewChunkEvent {
                    chunk_x_g: chunk_x_g,
                    chunk_y_g: chunk_y_g,
                });
                seed_chunk_with_ore((chunk_x_g, chunk_y_g), &mut chunk_map);
            }
            rendered_box_transform.translation.x = chunk_x_g as f32 * CHUNK_SIZE;
            rendered_box_transform.translation.y = chunk_y_g as f32 * CHUNK_SIZE;
        } else if i == 5 { //center right
            if let Some(chunk) = chunk_map.get(&(chunk_x_g + 1, chunk_y_g)) {
                *grid = chunk.clone();
            } else {
                chunk_event_writer.send(NewChunkEvent {
                    chunk_x_g: chunk_x_g + 1,
                    chunk_y_g: chunk_y_g,
                });
                seed_chunk_with_ore((chunk_x_g + 1, chunk_y_g), &mut chunk_map);
            }
            rendered_box_transform.translation.x = (chunk_x_g as f32 + 1.) * CHUNK_SIZE;
            rendered_box_transform.translation.y = chunk_y_g as f32 * CHUNK_SIZE;
        } else if i == 6 { //bottom left
            if let Some(chunk) = chunk_map.get(&(chunk_x_g - 1, chunk_y_g - 1)) {
                *grid = chunk.clone();
            } else {
                chunk_event_writer.send(NewChunkEvent {
                    chunk_x_g: chunk_x_g - 1,
                    chunk_y_g: chunk_y_g - 1,
                });
                seed_chunk_with_ore((chunk_x_g - 1, chunk_y_g - 1), &mut chunk_map);
            }
            rendered_box_transform.translation.x = (chunk_x_g as f32 - 1.) * CHUNK_SIZE;
            rendered_box_transform.translation.y = (chunk_y_g as f32 - 1.) * CHUNK_SIZE;
        } else if i == 7 { //bottom center
            if let Some(chunk) = chunk_map.get(&(chunk_x_g, chunk_y_g - 1)) {
                *grid = chunk.clone();
            } else {
                chunk_event_writer.send(NewChunkEvent {
                    chunk_x_g: chunk_x_g,
                    chunk_y_g: chunk_y_g - 1,
                });
                seed_chunk_with_ore((chunk_x_g, chunk_y_g - 1), &mut chunk_map);
            }
            rendered_box_transform.translation.x = chunk_x_g as f32 * CHUNK_SIZE;
            rendered_box_transform.translation.y = (chunk_y_g as f32 - 1.) * CHUNK_SIZE;
        } else if i == 8 { //bottom right
            if let Some(chunk) = chunk_map.get(&(chunk_x_g + 1, chunk_y_g - 1)) {
                *grid = chunk.clone();
            } else {
                chunk_event_writer.send(NewChunkEvent {
                    chunk_x_g: chunk_x_g + 1,
                    chunk_y_g: chunk_y_g - 1,
                });
                seed_chunk_with_ore((chunk_x_g + 1, chunk_y_g - 1), &mut chunk_map);
            }
            rendered_box_transform.translation.x = (chunk_x_g as f32 + 1.) * CHUNK_SIZE;
            rendered_box_transform.translation.y = (chunk_y_g as f32 - 1.) * CHUNK_SIZE;
        }
        i += 1;
    }
}