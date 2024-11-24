use bevy::{asset::Assets, math::{Vec2, Vec3}, prelude::{Commands, Image, Mesh, Rectangle, Res, ResMut, Transform}, sprite::MaterialMesh2dBundle, time::Time};

use crate::{color_map::{apply_gamma_correction, BLACK, LIGHT, PLAYER_SKIN, RAW_DECODER_DATA, RED, SELL_BOX, SKY, WHITE}, components::{PlayerTag, Velocity}, constants::{CHUNKS_HORIZONTAL, CHUNK_SIZE, GLOBAL_MAX_X, GLOBAL_MIN_X, PLAYER_HEIGHT, PLAYER_SPAWN_X, PLAYER_SPAWN_Y, PLAYER_WIDTH}, tools::{CurrentTool, Tool}, util::{flatten_index_standard_grid, get_chunk_x_g, get_chunk_x_v, get_chunk_y_g, get_chunk_y_v, get_local_x, get_local_y, grid_to_image}, world_generation::GridMaterial};


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
                    decoder: apply_gamma_correction(RAW_DECODER_DATA),
                }),
                mesh: meshes
                .add(Rectangle {
                    half_size: Vec2::new((PLAYER_WIDTH/2) as f32, (PLAYER_HEIGHT/2) as f32),
                })
                .into(),
                transform: Transform { translation: Vec3::new(PLAYER_SPAWN_X as f32, PLAYER_SPAWN_Y as f32, -1.), ..Default::default() },
                ..Default::default()
            })
            .insert(CurrentTool { tool: Tool::Shovel });
}

pub fn generate_player_image() -> Image{
    let mut data_buffer: Vec<u8> = Vec::new();
    for y in 0..PLAYER_HEIGHT {
        for _ in 0..PLAYER_WIDTH {
            if y < PLAYER_HEIGHT / 3 {
                data_buffer.push(PLAYER_SKIN);
            } else {
                data_buffer.push(BLACK);
            }
        }
    }
    for i in 0..7 {
        data_buffer[flatten_index_standard_grid(&(2 + i), &5, PLAYER_WIDTH)] = WHITE;
        data_buffer[flatten_index_standard_grid(&(PLAYER_WIDTH - 3 - i), &5, PLAYER_WIDTH)] = WHITE;
        data_buffer[flatten_index_standard_grid(&(2 + i), &4, PLAYER_WIDTH)] = WHITE;
        data_buffer[flatten_index_standard_grid(&(PLAYER_WIDTH - 3 - i), &4, PLAYER_WIDTH)] = WHITE;
    }
    data_buffer[flatten_index_standard_grid(&(2+3), &5, PLAYER_WIDTH)] = BLACK;
    data_buffer[flatten_index_standard_grid(&(PLAYER_WIDTH - 3 - 3), &5, PLAYER_WIDTH)] = BLACK;

    for i in 0..PLAYER_WIDTH - 4 {
        data_buffer[flatten_index_standard_grid(&(2 + i), &10, PLAYER_WIDTH)] = RED;
    }
    grid_to_image(&mut data_buffer, PLAYER_WIDTH as u32, PLAYER_HEIGHT as u32, None)
}

pub fn apply_velocity(
    entity_position_c: &mut Vec3,
    velocity: &mut Velocity,
    chunk_map: &Vec<Vec<u8>>,
    time: &Res<Time>,
) {
    let min_x_c = GLOBAL_MIN_X as f32 + PLAYER_WIDTH as f32 / 2.;
    let max_x_c = GLOBAL_MAX_X as f32 - PLAYER_WIDTH as f32 / 2.;
    if velocity.vx != 0. && horizontal_collision(&velocity.vx, chunk_map, &entity_position_c) {
        velocity.vx = 0.;
    }
    if entity_position_c.x < min_x_c {
        entity_position_c.x = min_x_c;
        velocity.vx = 0.;
    } else if entity_position_c.x > max_x_c {
        entity_position_c.x = max_x_c;
        velocity.vx = 0.;
    }
    if velocity.vy > 0. && vertical_collision(chunk_map, &entity_position_c) {
        velocity.vy = 0.;
    }
    entity_position_c.x += velocity.vx * time.delta_seconds();
    entity_position_c.y += velocity.vy * time.delta_seconds();
}

fn horizontal_collision(velocity: &f32, chunk_map: &Vec<Vec<u8>>, entity_position_c: &Vec3) -> bool {
    if velocity < &0. || velocity > &0. {
        for y in entity_position_c.y as i32 - PLAYER_HEIGHT as i32/2 + 1..entity_position_c.y as i32 + PLAYER_HEIGHT as i32/2 {
            let chunk_x_g: i32;
            let local_x: usize;
            if velocity < &0. {
                chunk_x_g = get_chunk_x_g(entity_position_c.x - PLAYER_WIDTH as f32/2. - 1.);
                local_x = get_local_x((entity_position_c.x - PLAYER_WIDTH as f32/2. - 1.) as i32);
            } else {
                chunk_x_g = get_chunk_x_g(entity_position_c.x + PLAYER_WIDTH as f32/2. + 1.);
                local_x = get_local_x((entity_position_c.x + PLAYER_WIDTH as f32/2. + 1.) as i32);
            }
            let chunk_y_g = get_chunk_y_g(y as f32);
            let chunk_x_v = get_chunk_x_v(chunk_x_g);
            let chunk_y_v = get_chunk_y_v(chunk_y_g);
            let chunk_index = flatten_index_standard_grid(&chunk_x_v, &chunk_y_v, CHUNKS_HORIZONTAL as usize);
            let local_y = get_local_y(y as i32);
            let local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
            if chunk_map[chunk_index][local_index] != SKY && chunk_map[chunk_index][local_index] != SELL_BOX && chunk_map[chunk_index][local_index] != LIGHT {
                return true
            }
        }
    }
    false
}

fn vertical_collision(chunk_map: &Vec<Vec<u8>>, entity_position_c: &Vec3) -> bool {
    for x in entity_position_c.x as i32 - PLAYER_WIDTH as i32 /2..entity_position_c.x as i32 + PLAYER_WIDTH as i32 /2 {
        let chunk_x_g = get_chunk_x_g(x as f32);
        let local_x = get_local_x(x);
        let chunk_y_g = get_chunk_y_g(entity_position_c.y + PLAYER_HEIGHT as f32 / 2. + 1.);
        let chunk_x_v = get_chunk_x_v(chunk_x_g);
        let chunk_y_v = get_chunk_y_v(chunk_y_g);
        let chunk_index = flatten_index_standard_grid(&chunk_x_v, &chunk_y_v, CHUNKS_HORIZONTAL as usize);
        let local_y = get_local_y((entity_position_c.y + PLAYER_HEIGHT as f32 / 2. + 1.) as i32);
        let local_index = flatten_index_standard_grid(&local_x, &local_y, CHUNK_SIZE as usize);
        if chunk_map[chunk_index][local_index] != SKY && chunk_map[chunk_index][local_index] != SELL_BOX && chunk_map[chunk_index][local_index] != LIGHT {
            return true
        }
    }
    false
}