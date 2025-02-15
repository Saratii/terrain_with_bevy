@group(2) @binding(4) var<storage, read_write> shadow_map: array<atomic<i32>, u32(SHADOW_RESOLUTION)>;
@group(0) @binding(0) var tile_map: texture_storage_2d<r8unorm, read>;
@group(0) @binding(3) var tile_map_left: texture_storage_2d<r8unorm, read>;
@group(0) @binding(4) var tile_map_right: texture_storage_2d<r8unorm, read>;
@group(0) @binding(2) var<uniform> player_global_pos: vec2<f32>;
@group(0) @binding(5) var tile_map_top_left: texture_storage_2d<r8unorm, read>;
@group(0) @binding(6) var tile_map_top: texture_storage_2d<r8unorm, read>;
@group(0) @binding(7) var tile_map_top_right: texture_storage_2d<r8unorm, read>;
@group(0) @binding(8) var tile_map_bottom_left: texture_storage_2d<r8unorm, read>;
@group(0) @binding(9) var tile_map_bottom: texture_storage_2d<r8unorm, read>;
@group(0) @binding(10) var tile_map_bottom_right: texture_storage_2d<r8unorm, read>;

const CHUNK_SIZE: f32 = 600.0;
const SHADOW_RESOLUTION: f32 = 2048.;
const LIGHT_PROJECTION : mat3x3<f32> = mat3x3<f32>(
    2.0 / (CHUNK_SIZE * 2),        0.0,                          0.0,
    0.0,                         -2.0 / (CHUNK_SIZE * 2),         0.0,
    0.0, 0.0, 1.0
);

@compute @workgroup_size(1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let flattened = location.x + location.y * i32(CHUNK_SIZE);
    if (flattened < i32(SHADOW_RESOLUTION)) {
        shadow_map[flattened] = bitcast<i32>(100000.0);
    }
}

fn calculate_shadows(local_x: i32, local_y: i32, relative_chunk_x: i32, relative_chunk_y: i32) -> vec2<f32> {
    let player_global_chunk = vec2<i32>(get_chunk_x_g(i32(player_global_pos.x)), get_chunk_y_g(i32(player_global_pos.y)));
    let global_x = get_global_x_coordinate(f32(player_global_chunk.x + relative_chunk_x), f32(local_x));
    let global_y = get_global_y_coordinate(f32(player_global_chunk.y + relative_chunk_y), f32(local_y));
    let light_position = LIGHT_PROJECTION * vec3<f32>(global_x - player_global_pos.x, global_y + player_global_pos.y, 1.0);
    let shadow_x = ((light_position.x + 1.0) * 0.5 * SHADOW_RESOLUTION);
    var tile_map_value: f32;
    if (relative_chunk_x == -1) {
        if relative_chunk_y == - 1 { 
            tile_map_value = textureLoad(tile_map_bottom_left, vec2<i32>(local_x, local_y)).r * 255.0;
        } else if relative_chunk_y == 0 {
            tile_map_value = textureLoad(tile_map_left, vec2<i32>(local_x, local_y)).r * 255.0;
        } else if relative_chunk_y == 1 {
            tile_map_value = textureLoad(tile_map_top_left, vec2<i32>(local_x, local_y)).r * 255.0;
        }
    } else if (relative_chunk_x == 0) {
        if relative_chunk_y == - 1 { 
            tile_map_value = textureLoad(tile_map_bottom, vec2<i32>(local_x, local_y)).r * 255.0;
        } else if relative_chunk_y == 0 {
            tile_map_value = textureLoad(tile_map, vec2<i32>(local_x, local_y)).r * 255.0;
        } else if relative_chunk_y == 1 {
            tile_map_value = textureLoad(tile_map_top, vec2<i32>(local_x, local_y)).r * 255.0;
        }
    } else if (relative_chunk_x == 1) {
        if relative_chunk_y == - 1 { 
            tile_map_value = textureLoad(tile_map_bottom_right, vec2<i32>(local_x, local_y)).r * 255.0;
        } else if relative_chunk_y == 0 {
            tile_map_value = textureLoad(tile_map_right, vec2<i32>(local_x, local_y)).r * 255.0;
        } else if relative_chunk_y == 1 {
            tile_map_value = textureLoad(tile_map_top_right, vec2<i32>(local_x, local_y)).r * 255.0;
        }
    }
    if (tile_map_value == 0.0) {
        return vec2<f32>(shadow_x, 10000.0);
    }
    return vec2<f32>(shadow_x, light_position.y);
}

@compute @workgroup_size(1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    var shadow_coord: vec2<f32>;
    let local_x = i32(invocation_id.x) % i32(CHUNK_SIZE);
    let local_y = i32(invocation_id.y) % i32(CHUNK_SIZE);
    if (i32(invocation_id.x) < i32(CHUNK_SIZE)) {
        if i32(invocation_id.y) < i32(CHUNK_SIZE) {
            shadow_coord = calculate_shadows(local_x, local_y, -1, 1);
        } else if i32(invocation_id.y) < i32(CHUNK_SIZE) * 2 {
            shadow_coord = calculate_shadows(local_x, local_y, -1, 0);
        } else if i32(invocation_id.y) < i32(CHUNK_SIZE) * 3 {
            shadow_coord = calculate_shadows(local_x, local_y, -1, -1);
        }
    } else if (i32(invocation_id.x) < i32(CHUNK_SIZE * 2)) {
        if i32(invocation_id.y) < i32(CHUNK_SIZE) {
            shadow_coord = calculate_shadows(local_x, local_y, 0, 1);
        } else if i32(invocation_id.y) < i32(CHUNK_SIZE) * 2 {
            shadow_coord = calculate_shadows(local_x, local_y, 0, 0);
        } else if i32(invocation_id.y) < i32(CHUNK_SIZE) * 3 {
            shadow_coord = calculate_shadows(local_x, local_y, 0, -1);
        }
    } else if (i32(invocation_id.x) < i32(CHUNK_SIZE * 3)) {
        if i32(invocation_id.y) < i32(CHUNK_SIZE) {
            shadow_coord = calculate_shadows(local_x, local_y, 1, 1);
        } else if i32(invocation_id.y) < i32(CHUNK_SIZE) * 2 {
            shadow_coord = calculate_shadows(local_x, local_y, 1, 0);
        } else if i32(invocation_id.y) < i32(CHUNK_SIZE) * 3 {
            shadow_coord = calculate_shadows(local_x, local_y, 1, -1);
        }
    }
    let old_shadow = bitcast<f32>(shadow_map[i32(shadow_coord.x)]);
    if (shadow_coord.y < old_shadow) {
        shadow_map[i32(shadow_coord.x)] = bitcast<i32>(shadow_coord.y);
    }
}

fn get_global_x_coordinate(chunk_x_g: f32, local_x: f32) -> f32 {
    return chunk_x_g * CHUNK_SIZE - CHUNK_SIZE / 2.0 + local_x;
}

fn get_global_y_coordinate(chunk_y_g: f32, local_y: f32) -> f32 {
    return chunk_y_g * CHUNK_SIZE + CHUNK_SIZE / 2.0 - local_y;
}

fn get_chunk_x_g(x_g: i32) -> i32 {
    return div_euclid(x_g + i32(CHUNK_SIZE) / 2, i32(CHUNK_SIZE));
}

fn get_chunk_y_g(y_g: i32) -> i32 {
    return div_euclid(y_g + i32(CHUNK_SIZE) / 2, i32(CHUNK_SIZE));
}

fn div_euclid(a : i32, b : i32) -> i32 {
    let quotient = a / b;
    let remainder = a % b;
    if remainder < 0 && b > 0 {
        return quotient - 1;
    } else if remainder > 0 && b < 0 {
        return quotient - 1;
    } else {
        return quotient;
    }
}