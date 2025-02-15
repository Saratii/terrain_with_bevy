use bevy::{asset::{Assets, Handle}, ecs::event::EventWriter, math::Vec3, prelude::{Image, Query, ResMut, Transform, With, Without}};

use crate::{chunk_generator::NewChunkEvent, components::{CameraTag, ChunkMap, PlayerTag, TerrainImageTag}, compute_shader::CurrentPlayerPosition, constants::CHUNK_SIZE, materials::GridMaterial, util::{get_chunk_x_g, get_chunk_y_g}};

pub fn render(
    mut materials: ResMut<Assets<GridMaterial>>,
    mut terrain_material_handle: Query<(&Handle<GridMaterial>, &mut Transform), (With<TerrainImageTag>, Without<PlayerTag>)>,
    mut images: ResMut<bevy::asset::Assets<Image>>,
    player_query: Query<&Transform, (With<PlayerTag>, Without<TerrainImageTag>, Without<CameraTag>)>,
    mut chunk_map_query: Query<&mut ChunkMap>,
    mut camera_query: Query<&mut Transform, (With<CameraTag>, Without<PlayerTag>, Without<TerrainImageTag>)>,
    mut chunk_event_writer: EventWriter<NewChunkEvent>,
    mut current_player_position: ResMut<CurrentPlayerPosition>,
) {
    let chunk_map = &mut chunk_map_query.get_single_mut().unwrap().map;
    let player_pos = player_query
        .get_single()
        .map(|player| player.translation)
        .unwrap_or(Vec3::ZERO);    let mut camera_transform = camera_query.get_single_mut().unwrap();
    camera_transform.translation = player_pos;
    let chunk_x_g = get_chunk_x_g(player_pos.x as i32);
    let chunk_y_g: i32 = get_chunk_y_g(player_pos.y as i32);
    current_player_position.position[0] = player_pos.x;
    current_player_position.position[1] = player_pos.y;
    for (material_handle, mut rendered_box_transform) in terrain_material_handle.iter_mut() {
        let material_handle = materials.get_mut(material_handle).unwrap();
        let grid_image = images.get_mut(&material_handle.color_map_handle).unwrap();
        let global_chunk_pos = &mut material_handle.global_chunk_pos;
        let player_pos_on_texture = &mut material_handle.player_pos;
        player_pos_on_texture.x = player_pos.x;
        player_pos_on_texture.y = player_pos.y;
        if let Some(chunk) = chunk_map.get(&(chunk_x_g + material_handle.on_screen_chunk_position[0] as i32, chunk_y_g + material_handle.on_screen_chunk_position[1] as i32)) {
            grid_image.data = chunk.clone();
        } else {
            chunk_event_writer.send(NewChunkEvent {
                chunk_x_g: chunk_x_g + material_handle.on_screen_chunk_position[0] as i32,
                chunk_y_g: chunk_y_g + material_handle.on_screen_chunk_position[1] as i32,
            });
        }
        global_chunk_pos.x = chunk_x_g as f32 + material_handle.on_screen_chunk_position[0] as f32;
        global_chunk_pos.y = chunk_y_g as f32 + material_handle.on_screen_chunk_position[1] as f32;
        rendered_box_transform.translation.x = (chunk_x_g as f32 + material_handle.on_screen_chunk_position[0] as f32) * CHUNK_SIZE;
        rendered_box_transform.translation.y = (chunk_y_g as f32 + material_handle.on_screen_chunk_position[1] as f32) * CHUNK_SIZE;
    }
}