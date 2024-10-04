use bevy::{asset::{Assets, Handle}, input::ButtonInput, prelude::{Image, MouseButton, Query, Res, ResMut, Transform, With, Without}};

use crate::{color_map::ROCK, components::{Bool, ContentList, GravityCoords, TerrainGridTag}, constants::{MAX_SHOVEL_CAPACITY, WINDOW_WIDTH}, tools::{left_click_hoe, left_click_pickaxe, left_click_shovel, right_click_hoe, right_click_shovel, CurrentTool, HoeTag, PickaxeTag, ShovelTag, Tool}, util::flatten_index_standard_grid, world_generation::GridMaterial};

pub fn check_mouse_click(
    buttons: Res<ButtonInput<MouseButton>>,
    mut shovel_position_query: Query<&mut Transform, (With<ShovelTag>, Without<PickaxeTag>)>,
    mut pickaxe_position_query: Query<&mut Transform, (With<PickaxeTag>, Without<ShovelTag>)>,
    mut hoe_position_query: Query<&mut Transform, (With<HoeTag>, Without<PickaxeTag>, Without<ShovelTag>)>,
    mut cursor_contents_query: Query<&mut ContentList, With<ShovelTag>>,
    mut gravity_coords_query: Query<&mut GravityCoords>,
    current_tool_query: Query<&CurrentTool>,
    mut is_hoe_locked: Query<&mut Bool, With<HoeTag>>,
    terrain_material_handle: Query<&Handle<GridMaterial>, (With<TerrainGridTag>, Without<ShovelTag>)>,
    mut materials: ResMut<Assets<GridMaterial>>,
    mut images: ResMut<Assets<Image>>,
    shovel_material_handle: Query<&Handle<GridMaterial>, (With<ShovelTag>, Without<TerrainGridTag>)>,
) {
    let mut cursor_contents = cursor_contents_query.get_single_mut().unwrap();
    let current_tool = current_tool_query.get_single().unwrap();
    if buttons.just_pressed(MouseButton::Left) && cursor_contents.contents.len() < MAX_SHOVEL_CAPACITY {
        let terrain_material_handle = terrain_material_handle.get_single().unwrap();
        let terrain_id = materials.get_mut(terrain_material_handle).unwrap().color_map.clone();
        let mut terrain_grid = images.remove(&terrain_id).unwrap();
        let mut gravity_coords = gravity_coords_query.get_single_mut().unwrap();
        match current_tool.tool {
            Tool::Shovel => {
                let shovel_material_handle = shovel_material_handle.get_single().unwrap();
                let shovel_id = materials.get_mut(shovel_material_handle).unwrap().color_map.clone();
                let mut shovel_image = images.remove(&shovel_id).unwrap();
                left_click_shovel(&shovel_position_query.get_single_mut().unwrap(), &mut cursor_contents.contents, &mut terrain_grid.data, &mut gravity_coords, &mut shovel_image.data);    
                images.insert(&shovel_id, shovel_image);        
            },
            Tool::Pickaxe => {
                left_click_pickaxe(&pickaxe_position_query.get_single_mut().unwrap(), &mut terrain_grid.data, &mut gravity_coords);
            },
            Tool::Hoe => {
                left_click_hoe(&mut hoe_position_query.get_single_mut().unwrap(), &mut terrain_grid.data, &mut is_hoe_locked.get_single_mut().unwrap().bool);
            },
        }
        images.insert(&terrain_id, terrain_grid);
    }
    if buttons.just_pressed(MouseButton::Middle) {
        let mut gravity_coords = gravity_coords_query.get_single_mut().unwrap();
        let terrain_material_handle = terrain_material_handle.get_single().unwrap();
        let terrain_id = materials.get_mut(terrain_material_handle).unwrap().color_map.clone();
        let terrain_grid = &mut images.get_mut(&terrain_id).unwrap().data;
        for x in 50..100{
            for i in 0..40{
                terrain_grid[flatten_index_standard_grid(&x, &((50 + i) as usize), WINDOW_WIDTH)] = ROCK;
                gravity_coords.coords.insert(( x, 50 + i));
            }
        }
    }
    if buttons.just_pressed(MouseButton::Right) {
        let mut gravity_coords = gravity_coords_query.get_single_mut().unwrap();
        match current_tool.tool {
            Tool::Shovel => {
                let tool_position = shovel_position_query.get_single_mut().unwrap();
                let terrain_id = &materials.get(terrain_material_handle.get_single().unwrap()).unwrap().color_map.clone();
                let shovel_material_handle = shovel_material_handle.get_single().unwrap();
                let shovel_id = materials.get_mut(shovel_material_handle).unwrap().color_map.clone();
                let mut terrain_grid = images.remove(terrain_id).unwrap();
                let mut shovel_image = images.remove(&shovel_id).unwrap();
                right_click_shovel(&mut shovel_image.data, &mut terrain_grid.data, &tool_position, &mut cursor_contents.contents, &mut gravity_coords);
                images.insert(terrain_id, terrain_grid);
                images.insert(&shovel_id, shovel_image);
            },
            Tool::Pickaxe => {},
            Tool::Hoe => right_click_hoe(&mut is_hoe_locked.get_single_mut().unwrap().bool),
        }
    }
}