@group(0) @binding(0) var input: texture_storage_2d<r32float, read>;

@group(0) @binding(1) var output: texture_storage_2d<r32float, write>;

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
}

const LEFT: f32 = -1. * 1200. / 2.;
const RIGHT: f32 = 1200. / 2.;
const TOP: f32 = 1200. / 2.;
const BOTTOM: f32 = -1. * 1200. / 2.;

const LIGHT_PROJECTION : mat3x3<f32> = mat3x3<f32>(
    2.0 / (RIGHT - LEFT), 0.0, 0.0,
    0.0, -2.0 / (TOP - BOTTOM), 0.0,
    -(RIGHT + LEFT) / (RIGHT - LEFT), (TOP + BOTTOM) / (TOP - BOTTOM), 1.0
);

const SHADOW_RESOLUTION: f32 = 1024.;

fn calculate_shadows(local_x: i32, local_y: i32) -> vec2<f32> {
    let global_x = get_global_x_coordinate(0., f32(local_x));
    let global_y = get_global_y_coordinate(0., f32(local_y));
    let light_position = LIGHT_PROJECTION * vec3<f32>(global_x, global_y, 1.0);
    let shadow_x = ((light_position.x + 1.0) * 0.5 * SHADOW_RESOLUTION);
    return vec2<f32>(shadow_x, light_position.y);
}


@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    //index from (0 to width, 0 to height) of the shadow map
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y)); //(0-600, 0-600)
    let shadow_coord = calculate_shadows(location.x, location.y);
    let old_shadow = textureLoad(output, vec2<i32>(i32(shadow_coord.x), 0));
    if old_shadow.y < shadow_coord.y {
        let shadow = vec4<f32>(f32(1.0));
        textureStore(output, vec2<i32>(i32(shadow_coord.x), 0), shadow.y);
    }
}

const CHUNK_SIZE: f32 = 600.0;

fn get_global_y_coordinate(chunk_y_g: f32, local_y: f32) -> f32 {
    return chunk_y_g * CHUNK_SIZE + CHUNK_SIZE / 2. - local_y;
}

fn get_global_x_coordinate(chunk_x_g: f32, local_x: f32) -> f32 {
    return chunk_x_g * CHUNK_SIZE - CHUNK_SIZE / 2. + local_x;
}