use bevy::{asset::{Assets, Handle}, ecs::event::EventWriter, input::ButtonInput, prelude::{Camera, GlobalTransform, Image, MouseButton, Query, Res, ResMut, Transform, With, Without}, window::{PrimaryWindow, Window}};

use crate::{chunk_generator::NewChunkEvent, components::{Bool, CameraTag, ChunkMap, ContentList, GravityCoords, TerrainImageTag}, constants::MAX_SHOVEL_CAPACITY, sun::GridMaterial, tools::{left_click_hoe, left_click_pickaxe, left_click_shovel, right_click_hoe, right_click_shovel, CurrentTool, HoeTag, PickaxeTag, ShovelTag, Tool}};

pub fn check_mouse_click(
    buttons: Res<ButtonInput<MouseButton>>,
    mut shovel_position_query: Query<&mut Transform, (With<ShovelTag>, Without<PickaxeTag>)>,
    mut pickaxe_position_query: Query<&mut Transform, (With<PickaxeTag>, Without<ShovelTag>)>,
    mut hoe_position_query: Query<&mut Transform, (With<HoeTag>, Without<PickaxeTag>, Without<ShovelTag>)>,
    mut cursor_contents_query: Query<&mut ContentList, With<ShovelTag>>,
    mut gravity_coords_query: Query<&mut GravityCoords>,
    current_tool_query: Query<&CurrentTool>,
    mut is_hoe_locked: Query<&mut Bool, With<HoeTag>>,
    mut materials: ResMut<Assets<GridMaterial>>,
    mut images: ResMut<Assets<Image>>,
    shovel_material_handle: Query<&Handle<GridMaterial>, (With<ShovelTag>, Without<TerrainImageTag>)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<CameraTag>>,
    mut chunk_map_query: Query<&mut ChunkMap>,
    mut chunk_writer: EventWriter<NewChunkEvent>,
) {
    let mut cursor_contents = cursor_contents_query.get_single_mut().unwrap();
    let current_tool = current_tool_query.get_single().unwrap();
    let mut chunk_map = chunk_map_query.get_single_mut().unwrap();
    if buttons.just_pressed(MouseButton::Left) && cursor_contents.contents.len() < MAX_SHOVEL_CAPACITY {
        match current_tool.tool {
            Tool::Shovel => {
                let mut gravity_coords = gravity_coords_query.get_single_mut().unwrap();
                let shovel_material_handle = shovel_material_handle.get_single().unwrap();
                let shovel_id = materials.get_mut(shovel_material_handle).unwrap().color_map_handle.clone();
                let mut shovel_image = images.remove(&shovel_id).unwrap();
                left_click_shovel(&shovel_position_query.get_single_mut().unwrap(), &mut cursor_contents.contents, &mut chunk_map.map, &mut shovel_image.data, &mut gravity_coords, &mut chunk_writer);    
                images.insert(&shovel_id, shovel_image);        
            },
            Tool::Pickaxe => {
                let mut gravity_coords = gravity_coords_query.get_single_mut().unwrap();
                left_click_pickaxe(&pickaxe_position_query.get_single_mut().unwrap(), &mut chunk_map.map, &mut gravity_coords);
            },
            Tool::Hoe => {
                left_click_hoe(&mut hoe_position_query.get_single_mut().unwrap(), &mut chunk_map.map, &mut is_hoe_locked.get_single_mut().unwrap().bool);
            },
            Tool::SpawnDrill => {
                let (camera, camera_transform) = q_camera.single();
                if let Some(_) = q_windows.single().cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin) {
                    // if valid_machine_spawn(&chunk_map.map, position_g, DRILL_WIDTH as usize, DRILL_HEIGHT as usize) {
                    //     spawn_drill(commands, asset_server, position_g, &chunk_map.map);
                    // }
                }
            }
        }
    }
    // if buttons.just_pressed(MouseButton::Middle) {
    //     let mut gravity_coords = gravity_coords_query.get_single_mut().unwrap();
    //     for x in 50..100{
    //         for i in 0..40{
    //             terrain_grid[flatten_index_standard_grid(&x, &((50 + i) as usize), CHUNK_SIZE as usize)] = ROCK;
    //             // gravity_coords.coords.insert((x, 50 + i));
    //         }
    //     }
    // }
    if buttons.just_pressed(MouseButton::Right) {
        match current_tool.tool {
            Tool::Shovel => {
                let mut gravity_coords = gravity_coords_query.get_single_mut().unwrap();
                let tool_position = shovel_position_query.get_single_mut().unwrap();
                let shovel_material_handle = shovel_material_handle.get_single().unwrap();
                let shovel_id = materials.get_mut(shovel_material_handle).unwrap().color_map_handle.clone();
                let mut shovel_image = images.remove(&shovel_id).unwrap();
                right_click_shovel(&mut shovel_image.data, &mut chunk_map.map, &tool_position, &mut cursor_contents.contents, &mut gravity_coords);
                images.insert(&shovel_id, shovel_image);
            },
            Tool::Pickaxe => {},
            Tool::Hoe => right_click_hoe(&mut is_hoe_locked.get_single_mut().unwrap().bool),
            Tool::SpawnDrill => {},
        }
    }
}