use bevy::{asset::Assets, math::{Vec2, Vec3}, prelude::{Commands, Image, Mesh, Rectangle, Res, ResMut, Transform}, sprite::MaterialMesh2dBundle, time::Time};

use crate::{color_map::{BLACK, LIGHT, PLAYER_SKIN, RED, SELL_BOX, SKY, WHITE}, components::{PlayerTag, Velocity}, constants::{PLAYER_HEIGHT, PLAYER_SPAWN_X, PLAYER_SPAWN_Y, PLAYER_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH}, tools::{CurrentTool, Tool}, util::{c_to_tl, flatten_index_standard_grid, grid_to_image}, world_generation::GridMaterial};


pub fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<GridMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(PlayerTag)
            .insert(Velocity { vx: 0.0, vy: 0.0})
            .insert(MaterialMesh2dBundle {
                material: materials.add(GridMaterial {
                    color_map: images.add(generate_player_image()),
                    size: Vec2::new(PLAYER_WIDTH as f32, PLAYER_HEIGHT as f32),
                }),
                mesh: meshes
                .add(Rectangle {
                    half_size: Vec2::new((PLAYER_WIDTH/2) as f32, (PLAYER_HEIGHT/2) as f32),
                })
                .into(),
                transform: Transform { translation: Vec3::new(PLAYER_SPAWN_X as f32, PLAYER_SPAWN_Y as f32, 0.0), ..Default::default() },
                ..Default::default()
            })
            .insert(CurrentTool { tool: Tool::Shovel });
}

pub fn generate_player_image() -> Image{
    let mut data_buffer: Vec<u8> = Vec::new();
    for y in 0..PLAYER_HEIGHT {
        for _ in 0..PLAYER_WIDTH {
            if y < 15 {
                data_buffer.push(PLAYER_SKIN);
            } else {
                data_buffer.push(BLACK);
            }
        }
    }
    for i in 0..2 {
        data_buffer[flatten_index_standard_grid(&(2 + i), &5, PLAYER_WIDTH)] = WHITE;
        data_buffer[flatten_index_standard_grid(&(PLAYER_WIDTH - 2 - i), &5, PLAYER_WIDTH)] = WHITE;
    }
    for i in 0..PLAYER_WIDTH - 4 {
        data_buffer[flatten_index_standard_grid(&(2 + i), &10, PLAYER_WIDTH)] = RED;
    }
    grid_to_image(&mut data_buffer, PLAYER_WIDTH as u32, PLAYER_HEIGHT as u32, None)
}

pub fn apply_velocity(
    entity_position_c: &mut Vec3,
    velocity: &mut Velocity,
    terrain_grid: &Vec<u8>,
    time: &Res<Time>,
) {
    let min_x_c = -1. * WINDOW_WIDTH as f32 / 2. + PLAYER_WIDTH as f32 / 2.;
    let max_x_c = WINDOW_WIDTH as f32 / 2. - PLAYER_WIDTH as f32 / 2.;
    let entity_position_tl = c_to_tl(entity_position_c, PLAYER_WIDTH as f32, PLAYER_HEIGHT as f32);
    if velocity.vx != 0. && horizontal_collision(&velocity.vx, terrain_grid, &entity_position_tl) {
        velocity.vx = 0.;
    }
    if entity_position_c.x < min_x_c {
        entity_position_c.x = min_x_c;
        velocity.vx = 0.;
    } else if entity_position_c.x > max_x_c {
        entity_position_c.x = max_x_c;
        velocity.vx = 0.;
    }
    if velocity.vy > 0. && ((entity_position_c.y as i32 + PLAYER_HEIGHT as i32 / 2) >= (WINDOW_HEIGHT as i32 / 2) - 1 
        || vertical_collision(terrain_grid, &entity_position_tl)) {
        velocity.vy = 0.;
    }
    entity_position_c.x += velocity.vx * time.delta_seconds();
    entity_position_c.y += velocity.vy * time.delta_seconds();
}

fn horizontal_collision(velocity: &f32, terrain_grid: &Vec<u8>, entity_position_tl: &(f32, f32)) -> bool {
    if velocity < &0. {
        for y in 0..PLAYER_HEIGHT {
            let index = flatten_index_standard_grid(&(entity_position_tl.0 as usize - 1), &(y as usize + entity_position_tl.1 as usize), WINDOW_WIDTH);
            if terrain_grid[index] != SKY && terrain_grid[index] != SELL_BOX && terrain_grid[index] != LIGHT {
                return true
            }
        }
    } else if velocity > &0.{
        for y in 0..PLAYER_HEIGHT {
            let index = flatten_index_standard_grid(&(entity_position_tl.0 as usize + PLAYER_WIDTH + 1), &(y as usize + entity_position_tl.1 as usize), WINDOW_WIDTH);
            if terrain_grid[index] != SKY && terrain_grid[index] != SELL_BOX && terrain_grid[index] != LIGHT {
                return true
            }
        }
    }
    false
}

fn vertical_collision(terrain_grid: &Vec<u8>, entity_position_tl: &(f32, f32)) -> bool {
    for x in entity_position_tl.0 as usize..entity_position_tl.0 as usize + PLAYER_WIDTH as usize {
        let index = flatten_index_standard_grid(&x, &(entity_position_tl.1 as usize - 1), WINDOW_WIDTH);
        if terrain_grid[index] != SKY && terrain_grid[index] != SELL_BOX && terrain_grid[index] != LIGHT {
            return true
        }
    }
    false
}