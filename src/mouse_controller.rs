use bevy::{input::ButtonInput, prelude::{MouseButton, Query, Res, Transform, With, Without}};
use rand::Rng;

use crate::{components::{ContentList, CurrentTool, GravityCoords, Grid, PickaxeTag, Pixel, PixelType, ShovelTag, TerrainGridTag, Tool}, constants::{CURSOR_BORDER_WIDTH, CURSOR_RADIUS, MAX_SHOVEL_CAPACITY, WINDOW_WIDTH}, player::update_shovel_content_visual, util::{distance, flatten_index, flatten_index_standard_grid}};

pub fn check_mouse_click(
    buttons: Res<ButtonInput<MouseButton>>,
    mut shovel_position_query: Query<&mut Transform, (With<ShovelTag>, Without<PickaxeTag>)>,
    mut pickaxe_position_query: Query<&mut Transform, (With<PickaxeTag>, Without<ShovelTag>)>,
    mut grid_query: Query<&mut Grid<Pixel>, (With<TerrainGridTag>, Without<ShovelTag>)>,
    mut cursor_contents_query: Query<&mut ContentList, With<ShovelTag>>,
    mut shovel_grid_query: Query<&mut Grid<Pixel>, (With<ShovelTag>, Without<TerrainGridTag>)>,
    mut gravity_coords_query: Query<&mut GravityCoords>,
    current_tool_query: Query<&CurrentTool>,
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
            Tool::Pickaxe => {}
        }
    }
    if buttons.just_pressed(MouseButton::Left) && cursor_contents.contents.len() < MAX_SHOVEL_CAPACITY {
        match current_tool.tool {
            Tool::Shovel => {
                let tool_position = shovel_position_query.get_single_mut().unwrap();
                left_click_shovel(&tool_position, &mut cursor_contents.contents, &mut grid.data, &mut gravity_coords, &mut shovel_grid.data);
            },
            Tool::Pickaxe => {
                let tool_position = pickaxe_position_query.get_single_mut().unwrap();
                left_click_pickaxe(&tool_position, &mut grid.data, &mut gravity_coords)
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

pub fn right_click_shovel(shovel_grid: &mut Vec<Pixel>, terrain_grid: &mut Vec<Pixel>, cursor_position: &Transform, cursor_contents: &mut Vec<Pixel>, gravity_coords: &mut GravityCoords){
    for y in 0..CURSOR_RADIUS * 2 {
        for x in 0..CURSOR_RADIUS * 2 {
            if cursor_contents.len() == 0{
                update_shovel_content_visual(shovel_grid, cursor_contents);
                return
            }
            let shovel_grid_index = flatten_index_standard_grid(&x, &y, CURSOR_RADIUS * 2);
            if matches!(shovel_grid[shovel_grid_index].pixel_type, PixelType::Ground(_) | PixelType::Gravel(_) | PixelType::Chalcopyrite) {
                let main_grid_index = flatten_index(cursor_position.translation.x as i32 - CURSOR_RADIUS as i32 + x as i32, cursor_position.translation.y as i32 - CURSOR_RADIUS as i32 + (CURSOR_RADIUS * 2 - y - 1) as i32);
                if let PixelType::Sky = terrain_grid[main_grid_index].pixel_type {
                    let pixel = cursor_contents.pop().unwrap();
                    terrain_grid[main_grid_index] = pixel;
                    gravity_coords.coords.insert((main_grid_index % WINDOW_WIDTH, main_grid_index / WINDOW_WIDTH));
                }
            }
        }
    }
    update_shovel_content_visual(shovel_grid, cursor_contents);
}

fn search_upward_for_non_sky_pixel(grid: &Vec<Pixel>, x: usize, y: usize) -> Option<usize> {
    let mut y_level = 1;
    loop {
        if y - y_level == 0{
            return None
        }
        if !matches!(grid[flatten_index_standard_grid(&x, &(y - y_level), WINDOW_WIDTH)].pixel_type, PixelType::Sky) {
            return Some(y - y_level)
        }
        y_level += 1;
    }
}

pub fn left_click_shovel(shovel_position: &Transform, shovel_contents: &mut Vec<Pixel>, grid: &mut Vec<Pixel>, gravity_coords: &mut GravityCoords, shovel_grid: &mut Vec<Pixel>) {
    let left = shovel_position.translation.x as i32 - CURSOR_RADIUS as i32;
    let right = shovel_position.translation.x as i32 + CURSOR_RADIUS as i32;
    let top = shovel_position.translation.y as i32 + CURSOR_RADIUS as i32; 
    let bottom = shovel_position.translation.y as i32 - CURSOR_RADIUS as i32;
    let mut min_x = WINDOW_WIDTH + 1;
    let mut max_x = 0;
    let starting_count = shovel_contents.len();
    for y in bottom..top {
        for x in left..right {
            if distance(x, y, shovel_position.translation.x as i32, shovel_position.translation.y as i32) < CURSOR_RADIUS as f32 - CURSOR_BORDER_WIDTH {
                let index = flatten_index(x as i32, y as i32);
                if let PixelType::Ground(dirt_variant) = grid[index].pixel_type.clone() {
                    shovel_contents.push(Pixel { pixel_type: PixelType::Ground(dirt_variant), gamma: 1.});
                    grid[index] = Pixel { pixel_type: PixelType::Sky, gamma: 1.};
                    if let Some(y) = search_upward_for_non_sky_pixel(grid, index % WINDOW_WIDTH, index / WINDOW_WIDTH){
                        gravity_coords.coords.insert((index % WINDOW_WIDTH, y));
                    }
                    if index % WINDOW_WIDTH < min_x {
                        min_x = index % WINDOW_WIDTH;
                    } else if index % WINDOW_WIDTH > max_x {
                        max_x = index % WINDOW_WIDTH;
                    }
                    if shovel_contents.len() == MAX_SHOVEL_CAPACITY {
                        update_shovel_content_visual(shovel_grid, shovel_contents);
                        return
                    }
                } else if let PixelType::Gravel(gravel_variant) = grid[index].pixel_type.clone() {
                    shovel_contents.push(Pixel { pixel_type: PixelType::Gravel(gravel_variant), gamma: 1.});
                    grid[index] = Pixel { pixel_type: PixelType::Sky, gamma: 1.};
                    if let Some(y) = search_upward_for_non_sky_pixel(grid, index % WINDOW_WIDTH, index / WINDOW_WIDTH){
                        gravity_coords.coords.insert((index % WINDOW_WIDTH, y));
                    }
                    if index % WINDOW_WIDTH < min_x {
                        min_x = index % WINDOW_WIDTH;
                    } else if index % WINDOW_WIDTH > max_x {
                        max_x = index % WINDOW_WIDTH;
                    }
                    if shovel_contents.len() == MAX_SHOVEL_CAPACITY {
                        update_shovel_content_visual(shovel_grid, shovel_contents);
                        return
                    }
                } else if let PixelType::Chalcopyrite = grid[index].pixel_type {
                    shovel_contents.push(Pixel { pixel_type: PixelType::Chalcopyrite, gamma: 1.});
                    grid[index] = Pixel { pixel_type: PixelType::Sky, gamma: 1.};
                    gravity_coords.coords.insert((index % WINDOW_WIDTH, index / WINDOW_WIDTH));
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
        update_shovel_content_visual(shovel_grid, shovel_contents);
    }
}

fn left_click_pickaxe(pickaxe_position: &Transform, grid: &mut Vec<Pixel>, gravity_coords: &mut GravityCoords) {
    let left = pickaxe_position.translation.x as i32 - CURSOR_RADIUS as i32;
    let right = pickaxe_position.translation.x as i32 + CURSOR_RADIUS as i32;
    let top = pickaxe_position.translation.y as i32 + CURSOR_RADIUS as i32; 
    let bottom = pickaxe_position.translation.y as i32 - CURSOR_RADIUS as i32;
    let rng = &mut rand::thread_rng();
    for y in bottom..top{
        for x in left..right{
            if distance(x, y, pickaxe_position.translation.x as i32, pickaxe_position.translation.y as i32) < CURSOR_RADIUS as f32 - CURSOR_BORDER_WIDTH {
                let index = flatten_index(x as i32, y as i32);
                if matches!(grid[index].pixel_type, PixelType::Rock) {
                    grid[index] = Pixel { pixel_type: PixelType::Gravel(rng.gen()), gamma: 1.};
                    gravity_coords.coords.insert((index % WINDOW_WIDTH, index / WINDOW_WIDTH));
                }
            }
        }
    }
}