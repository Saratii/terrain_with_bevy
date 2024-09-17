use bevy::{input::ButtonInput, prelude::{MouseButton, Query, Res, Transform, With, Without}};

use crate::{components::{Bool, ContentList, GravityCoords, Grid, Pixel, PixelType, TerrainGridTag}, constants::{MAX_SHOVEL_CAPACITY, WINDOW_WIDTH}, tools::{left_click_hoe, left_click_pickaxe, left_click_shovel, right_click_hoe, right_click_shovel, CurrentTool, HoeTag, PickaxeTag, ShovelTag, Tool}, util::flatten_index_standard_grid};

pub fn check_mouse_click(
    buttons: Res<ButtonInput<MouseButton>>,
    mut shovel_position_query: Query<&mut Transform, (With<ShovelTag>, Without<PickaxeTag>)>,
    mut pickaxe_position_query: Query<&mut Transform, (With<PickaxeTag>, Without<ShovelTag>)>,
    mut hoe_position_query: Query<&mut Transform, (With<HoeTag>, Without<PickaxeTag>, Without<ShovelTag>)>,
    mut grid_query: Query<&mut Grid<Pixel>, (With<TerrainGridTag>, Without<ShovelTag>)>,
    mut cursor_contents_query: Query<&mut ContentList, With<ShovelTag>>,
    mut shovel_grid_query: Query<&mut Grid<Pixel>, (With<ShovelTag>, Without<TerrainGridTag>)>,
    mut gravity_coords_query: Query<&mut GravityCoords>,
    current_tool_query: Query<&CurrentTool>,
    mut is_hoe_locked: Query<&mut Bool, With<HoeTag>>,
) {
    let mut cursor_contents = cursor_contents_query.get_single_mut().unwrap();
    let mut grid = grid_query.get_single_mut().unwrap();
    let mut shovel_grid = shovel_grid_query.get_single_mut().unwrap();
    let mut gravity_coords = gravity_coords_query.get_single_mut().unwrap();
    let current_tool = current_tool_query.get_single().unwrap();
    if buttons.just_pressed(MouseButton::Right) {
        match current_tool.tool {
            Tool::Shovel => {
                let tool_position = shovel_position_query.get_single_mut().unwrap();
                right_click_shovel(&mut shovel_grid.data, &mut grid.data, &tool_position, &mut cursor_contents.contents, &mut gravity_coords)
            },
            Tool::Pickaxe => {},
            Tool::Hoe => right_click_hoe(&mut is_hoe_locked.get_single_mut().unwrap().bool),
        }
    }
    if buttons.just_pressed(MouseButton::Left) && cursor_contents.contents.len() < MAX_SHOVEL_CAPACITY {
        match current_tool.tool {
            Tool::Shovel => {
                left_click_shovel(&shovel_position_query.get_single_mut().unwrap(), &mut cursor_contents.contents, &mut grid.data, &mut gravity_coords, &mut shovel_grid.data);
            },
            Tool::Pickaxe => {
                left_click_pickaxe(&pickaxe_position_query.get_single_mut().unwrap(), &mut grid.data, &mut gravity_coords);
            },
            Tool::Hoe => {
                left_click_hoe(&mut hoe_position_query.get_single_mut().unwrap(), &mut grid.data, &mut is_hoe_locked.get_single_mut().unwrap().bool);
            },
        }
    }
    if buttons.just_pressed(MouseButton::Middle) {
        for x in 50..100{
            for i in 0..40{
                grid.data[flatten_index_standard_grid(&x, &((225 + i) as usize), WINDOW_WIDTH)] = Pixel { pixel_type: PixelType::Rock, gamma: 0.};
                gravity_coords.coords.insert(( x, 225 + i));
            }
        }
    }
}