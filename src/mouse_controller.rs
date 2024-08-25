use bevy::{input::ButtonInput, prelude::{MouseButton, Query, Res, Transform, With, Without}};

use crate::{components::{ContentList, CurrentTool, ErosionColumns, Grid, PickaxeTag, Pixel, ShovelTag, TerrainGridTag, TerrainPositionsAffectedByGravity, Tool}, constants::{CURSOR_BORDER_WIDTH, CURSOR_RADIUS, MAX_SHOVEL_CAPACITY, WINDOW_WIDTH}, player::update_shovel_content_visual, util::{distance, flatten_index, flatten_index_standard_grid}};

pub fn check_mouse_click(
    buttons: Res<ButtonInput<MouseButton>>,
    mut shovel_position_query: Query<&mut Transform, (With<ShovelTag>, Without<PickaxeTag>)>,
    mut pickaxe_position_query: Query<&mut Transform, (With<PickaxeTag>, Without<ShovelTag>)>,
    mut grid_query: Query<&mut Grid, (With<TerrainGridTag>, Without<ShovelTag>)>,
    mut cursor_contents_query: Query<&mut ContentList, With<ShovelTag>>,
    mut shovel_grid_query: Query<&mut Grid, (With<ShovelTag>, Without<TerrainGridTag>)>,
    mut gravity_columns_query: Query<&mut TerrainPositionsAffectedByGravity>,
    mut erosion_columns_query: Query<&mut ErosionColumns>,
    current_tool_query: Query<&CurrentTool>,
){
    let mut cursor_contents = cursor_contents_query.get_single_mut().unwrap();
    let mut grid = grid_query.get_single_mut().unwrap();
    let mut shovel_grid = shovel_grid_query.get_single_mut().unwrap();
    let mut gravity_columns = gravity_columns_query.get_single_mut().unwrap();
    let mut erosion_columns = erosion_columns_query.get_single_mut().unwrap();
    let current_tool = current_tool_query.get_single().unwrap();
    if buttons.just_pressed(MouseButton::Right) {
        match current_tool.tool {
            Tool::Shovel => {
                let tool_position = shovel_position_query.get_single_mut().unwrap();
                right_click_shovel(&mut shovel_grid.data, &mut grid.data, &tool_position, &mut cursor_contents.contents, &mut gravity_columns)
            },
            Tool::Pickaxe => {}
        }
    }
    if buttons.just_pressed(MouseButton::Left) && cursor_contents.contents.len() < MAX_SHOVEL_CAPACITY {
        match current_tool.tool {
            Tool::Shovel => {
                let tool_position = shovel_position_query.get_single_mut().unwrap();
                left_click_shovel(&tool_position, &mut cursor_contents.contents, &mut grid.data, &mut gravity_columns, &mut erosion_columns, &mut shovel_grid.data)}
                ,
            Tool::Pickaxe => {
                let tool_position = pickaxe_position_query.get_single_mut().unwrap();
                left_click_pickaxe(&tool_position, &mut grid.data, &mut gravity_columns)
            },
        }
    }
}

pub fn right_click_shovel(shovel_grid: &mut Vec<Pixel>, terrain_grid: &mut Vec<Pixel>, cursor_position: &Transform, cursor_contents: &mut Vec<Pixel>, gravity_columns: &mut TerrainPositionsAffectedByGravity){
    for y in 0..CURSOR_RADIUS * 2 {
        for x in 0..CURSOR_RADIUS * 2 {
            let shovel_grid_index = flatten_index_standard_grid(&x, &y, CURSOR_RADIUS * 2);
            if matches!(shovel_grid[shovel_grid_index], Pixel::Ground(_) | Pixel::Gravel){
                let main_grid_index = flatten_index(cursor_position.translation.x as i32 - CURSOR_RADIUS as i32 + x as i32, cursor_position.translation.y as i32 - CURSOR_RADIUS as i32 + (CURSOR_RADIUS * 2 - y - 1) as i32);
                if terrain_grid[main_grid_index] == Pixel::Sky {
                    let pixel = cursor_contents.pop().unwrap();
                    terrain_grid[main_grid_index] = pixel;
                    gravity_columns.positions.insert(main_grid_index % WINDOW_WIDTH);
                }
            }
        }
    }
    update_shovel_content_visual(shovel_grid, cursor_contents);
}

pub fn left_click_shovel(shovel_position: &Transform, shovel_contents: &mut Vec<Pixel>, grid: &mut Vec<Pixel>, gravity_columns: &mut TerrainPositionsAffectedByGravity, erosion_columns: &mut ErosionColumns, shovel_grid: &mut Vec<Pixel>){
    let left = shovel_position.translation.x as i32 - CURSOR_RADIUS as i32;
    let right = shovel_position.translation.x as i32 + CURSOR_RADIUS as i32;
    let top = shovel_position.translation.y as i32 + CURSOR_RADIUS as i32; 
    let bottom = shovel_position.translation.y as i32 - CURSOR_RADIUS as i32;
    let mut min_x = WINDOW_WIDTH+1;
    let mut max_x = 0;
    let starting_count = shovel_contents.len();
    for y in bottom..top{
        for x in left..right{
            if distance(x, y, shovel_position.translation.x as i32, shovel_position.translation.y as i32) < CURSOR_RADIUS as f32 - CURSOR_BORDER_WIDTH {
                let index = flatten_index(x as i32, y as i32);
                if let Pixel::Ground(dirt_variant ) = grid[index].clone(){
                    shovel_contents.push(Pixel::Ground(dirt_variant));
                    grid[index] = Pixel::Sky;
                    gravity_columns.positions.insert(index % WINDOW_WIDTH);
                    if index % WINDOW_WIDTH < min_x {
                        min_x = index % WINDOW_WIDTH;
                    } else if index % WINDOW_WIDTH > max_x {
                        max_x = index % WINDOW_WIDTH;
                    }
                    if shovel_contents.len() == MAX_SHOVEL_CAPACITY {
                        update_shovel_content_visual(shovel_grid, shovel_contents);
                        return
                    }
                } else if grid[index] == Pixel::Gravel{
                    shovel_contents.push(Pixel::Gravel);
                    grid[index] = Pixel::Sky;
                    gravity_columns.positions.insert(index % WINDOW_WIDTH);
                    if index % WINDOW_WIDTH < min_x {
                        min_x = index % WINDOW_WIDTH;
                    } else if index % WINDOW_WIDTH > max_x {
                        max_x = index % WINDOW_WIDTH;
                    }
                    if shovel_contents.len() == MAX_SHOVEL_CAPACITY {
                        update_shovel_content_visual(shovel_grid, shovel_contents);
                        return
                    }
                }
            }
        }
    }
    if starting_count != shovel_contents.len() {
        erosion_columns.columns.insert(min_x-1);
        erosion_columns.columns.insert(max_x+1);
        update_shovel_content_visual(shovel_grid, shovel_contents);
    }
}

fn left_click_pickaxe(pickaxe_position: &Transform, grid: &mut Vec<Pixel>, gravity_columns: &mut TerrainPositionsAffectedByGravity) {
    let left = pickaxe_position.translation.x as i32 - CURSOR_RADIUS as i32;
    let right = pickaxe_position.translation.x as i32 + CURSOR_RADIUS as i32;
    let top = pickaxe_position.translation.y as i32 + CURSOR_RADIUS as i32; 
    let bottom = pickaxe_position.translation.y as i32 - CURSOR_RADIUS as i32;
    for y in bottom..top{
        for x in left..right{
            if distance(x, y, pickaxe_position.translation.x as i32, pickaxe_position.translation.y as i32) < CURSOR_RADIUS as f32 - CURSOR_BORDER_WIDTH {
                let index = flatten_index(x as i32, y as i32);
                if grid[index] == Pixel::Rock{
                    grid[index] = Pixel::Gravel;
                    gravity_columns.positions.insert(index % WINDOW_WIDTH);
                }
            }
        }
    }
}