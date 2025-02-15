#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> size: vec2<f32>; // width, height
@group(2) @binding(1) var tile_map: texture_2d<f32>;
@group(2) @binding(2) var<uniform> decoder: array<vec4<f32>, 24>;
@group(2) @binding(4) var<storage, read> shadow_map: array<i32, u32(SHADOW_RESOLUTION)>;
@group(2) @binding(5) var<uniform> global_chunk_position: vec2<f32>;
@group(2) @binding(6) var<uniform> player_global_position: vec2<f32>;

const CHUNK_SIZE: f32 = 600.0;
const SHADOW_RESOLUTION: f32 = 2048.;
const LIGHT_PROJECTION : mat3x3<f32> = mat3x3<f32>(
    2.0 / (CHUNK_SIZE * 2),        0.0,                          0.0,
    0.0,                         -2.0 / (CHUNK_SIZE*2),         0.0,
    0.0, 0.0, 1.0
);

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let local_coord = vec2<i32>(i32(mesh.uv.x * size.x), i32(mesh.uv.y * size.y));
    let tile_map_value = textureLoad(tile_map, local_coord, 0).r * 255.0;
    var color = decoder[i32(tile_map_value)];
    let is_edge = local_coord.x == 0 || local_coord.x == i32(size.x) - 1 || local_coord.y == 0 || local_coord.y == i32(size.y) - 1;
    if is_edge {
        color = vec4<f32>(144/255., 238/255., 144/255., 1.0); // light green for edges
    }
    //hide ores that are in the dark
    // if (i32(tile_map_value) == 21 || i32(tile_map_value) == 4) && light < 0.2 {
    //     color = decoder.colors[5];
    // }
    let is_lit = shade(f32(local_coord.x), f32(local_coord.y), global_chunk_position.x, global_chunk_position.y);
    return vec4<f32>(
        color.r * is_lit,
        color.g * is_lit,
        color.b * is_lit,
        color.a
    );
}

fn shade(local_x: f32, local_y: f32, global_chunk_x: f32, global_chunk_y: f32) -> f32 {
    let global_x = get_global_x_coordinate(global_chunk_x, local_x);
    let global_y = get_global_y_coordinate(global_chunk_y, local_y);
    let light_position = LIGHT_PROJECTION * vec3<f32>(global_x - player_global_position.x, global_y + player_global_position.y, 1.0);
    let shadow_x = clamp(((light_position.x + 1.0) * 0.5) * SHADOW_RESOLUTION, 0.0, SHADOW_RESOLUTION - 1.0);
    let shadow_y = bitcast<f32>(shadow_map[i32(shadow_x)]);
    if light_position.y <= shadow_y {
        return 1.0;
    }
    let diff = light_position.y - shadow_y;
    let maxDiff = 0.15;
    return clamp(1.0 - diff / maxDiff, 0.0, 1.0);
}

fn get_global_y_coordinate(chunk_y_g: f32, local_y: f32) -> f32 {
    return chunk_y_g * CHUNK_SIZE + CHUNK_SIZE / 2. - local_y;
}

fn get_global_x_coordinate(chunk_x_g: f32, local_x: f32) -> f32 {
    return chunk_x_g * CHUNK_SIZE - CHUNK_SIZE / 2. + local_x;
}