use bevy::ecs::{event::{Event, EventReader}, system::Query};
use noise::{NoiseFn, Perlin};

use crate::{color_map::{dirt_variant_pmf, grass_variant_pmf, ROCK, SKY}, components::{ChunkMap, PerlinHandle}, constants::{CHUNK_SIZE, DIRT_NOISE_SMOOTHNESS, DIRT_VARIATION, ROCK_NOISE_SMOOTHNESS, ROCK_VARIATION}, util::{get_global_x_coordinate, get_global_y_coordinate}};

#[derive(Event)]
pub struct NewChunkEvent{
    pub chunk_x_g: i32,
    pub chunk_y_g: i32,
}
 
pub fn generate_chunk_listener(
    mut events: EventReader<NewChunkEvent>,
    mut chunk_map_query: Query<&mut ChunkMap>,
    perlin_query: Query<&PerlinHandle>,
) {
    for event in events.read() {
        let perlin = perlin_query.get_single().unwrap().handle;
        let (grid, _local_height_map) = generate_chunk(event.chunk_x_g, event.chunk_y_g, &perlin);
        let mut chunk_map = chunk_map_query.get_single_mut().unwrap();
        chunk_map.map.insert((event.chunk_x_g, event.chunk_y_g), grid);
    }
}

pub fn generate_chunk(chunk_x_g: i32, chunk_y_g: i32, perlin: &Perlin) -> (Vec<u8>, Vec<i32>) {
    println!("Generating chunk at {}, {}", chunk_x_g, chunk_y_g);
    let mut local_height_map = Vec::new();
    let mut grid = vec![0; (CHUNK_SIZE * CHUNK_SIZE) as usize];
    let mut grass_variant_pmf = grass_variant_pmf();
    let mut dirt_variant_pmf = dirt_variant_pmf();
    for x in 0..CHUNK_SIZE as usize {
        let mut max_column_height = i32::MIN;
        let global_x = get_global_x_coordinate(chunk_x_g, x);
        let dirt_perlin =  perlin.get([global_x as f64 * DIRT_NOISE_SMOOTHNESS, 0.0]) * DIRT_VARIATION;
        let rock_perlin = perlin.get([global_x as f64 * ROCK_NOISE_SMOOTHNESS, 0.0]) * ROCK_VARIATION;
        let grass_perlin_top = perlin.get([global_x as f64 * DIRT_NOISE_SMOOTHNESS, 0.0]) * 10.;
        let grass_perlin_bottom = perlin.get([global_x as f64 * 0.1, 0.0]) * 10.;
        for y in 0..CHUNK_SIZE as usize {
            let global_y = get_global_y_coordinate(chunk_y_g, y);
            let index = y * CHUNK_SIZE as usize + x;
            if global_y > grass_perlin_top as i32 + 10 {
                grid[index] = SKY;
            } else if global_y > grass_perlin_bottom as i32 - 5 {
                grid[index] = grass_variant_pmf.next().unwrap();
                if global_y > max_column_height {
                    max_column_height = global_y;
                }
            } else if global_y > dirt_perlin as i32 {
                grid[index] = dirt_variant_pmf.next().unwrap();
                if global_y > max_column_height {
                    max_column_height = global_y;
                }
            } else if global_y > -100 + rock_perlin as i32 {
                grid[index] = dirt_variant_pmf.next().unwrap();
            } else {
                grid[index] = ROCK;
            }
        }
        local_height_map.push(max_column_height);
    }
    (grid, local_height_map)
}