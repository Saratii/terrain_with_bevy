@group(0) @binding(0) var tile_map: texture_storage_2d<r8unorm, read>;
@group(0) @binding(3) var tile_map_left: texture_storage_2d<r8unorm, read>;
@group(0) @binding(4) var tile_map_right: texture_storage_2d<r8unorm, read>;
@group(0) @binding(1) var output: texture_storage_2d<r32float, read_write>;
@group(0) @binding(2) var<uniform> current_chunk: vec2<i32>;

const CHUNK_SIZE: f32 = 600.0;
const SHADOW_RESOLUTION: f32 = 2048.;
const LEFT: f32 = -900.0;
const RIGHT: f32 = 900.0;
const TOP: f32 = 1200. / 2.;
const BOTTOM: f32 = -1. * 1200. / 2.;
const LIGHT_PROJECTION : mat3x3<f32> = mat3x3<f32>(
    2.0 / (RIGHT - LEFT),        0.0,                          0.0,
    0.0,                         -2.0 / (TOP - BOTTOM),         0.0,
    0.0, 0.0, 1.0
);

@compute @workgroup_size(1, 1, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let flattened = location.x + location.y * i32(CHUNK_SIZE);
    if (flattened < i32(SHADOW_RESOLUTION)) {
        textureStore(output, vec2<i32>(flattened, 0), vec4<f32>(100000.0, 0.0, 0.0, 0.0));
    }
}

fn calculate_shadows(local_x: i32, local_y: i32, relative_chunk_x: i32, relative_chunk_y: i32) -> vec2<f32> {
    let absolute_chunk_x: i32 = current_chunk.x + relative_chunk_x;
    let absolute_chunk_y: i32 = current_chunk.y + relative_chunk_y;
    let clamped_local_x = clamp(local_x, 0, i32(CHUNK_SIZE) - 1);
    let clamped_local_y = clamp(local_y, 0, i32(CHUNK_SIZE) - 1);
    let global_x = get_global_x_coordinate(f32(absolute_chunk_x), f32(clamped_local_x));
    let global_y = get_global_y_coordinate(f32(absolute_chunk_y), f32(clamped_local_y));
    let light_position = LIGHT_PROJECTION * vec3<f32>(global_x, global_y, 1.0);
    let shadow_x = ((light_position.x + 1.0) * 0.5 * SHADOW_RESOLUTION);
    var tile_map_value: f32;
    if (relative_chunk_x == -1) {
        tile_map_value = textureLoad(tile_map_left, vec2<i32>(clamped_local_x, clamped_local_y)).r * 255.0;
    } else if (relative_chunk_x == 0) {
        tile_map_value = textureLoad(tile_map, vec2<i32>(clamped_local_x, clamped_local_y)).r * 255.0;
    } else if (relative_chunk_x == 1) {
        tile_map_value = textureLoad(tile_map_right, vec2<i32>(clamped_local_x, clamped_local_y)).r * 255.0;
    } else {
        tile_map_value = 0.0;
    }
    if (tile_map_value == 0.0) {
        return vec2<f32>(shadow_x, 10000.0);
    }
    return vec2<f32>(shadow_x, light_position.y);
}

@compute @workgroup_size(1, 1, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    var shadow_coord: vec2<f32>;
    if (location.x < i32(CHUNK_SIZE)) {
        shadow_coord = calculate_shadows(location.x, location.y, -1, 0);
    } else if (location.x < i32(CHUNK_SIZE * 2)) {
        shadow_coord = calculate_shadows(location.x - i32(CHUNK_SIZE), location.y, 0, 0);
    } else {
        shadow_coord = calculate_shadows(location.x - i32(CHUNK_SIZE * 2), location.y, 1, 0);
    }
    let old_shadow = textureLoad(output, vec2<i32>(i32(shadow_coord.x), 0));
    if (shadow_coord.y < old_shadow.x) {
        textureStore(output, vec2<i32>(i32(shadow_coord.x), 0), vec4<f32>(shadow_coord.y, 0.0, 0.0, 0.0));
    }
}

fn get_global_x_coordinate(chunk_x_g: f32, local_x: f32) -> f32 {
    return chunk_x_g * CHUNK_SIZE - CHUNK_SIZE / 2.0 + local_x;
}

fn get_global_y_coordinate(chunk_y_g: f32, local_y: f32) -> f32 {
    return chunk_y_g * CHUNK_SIZE + CHUNK_SIZE / 2.0 - local_y;
}