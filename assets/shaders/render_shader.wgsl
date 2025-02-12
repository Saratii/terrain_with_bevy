#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct Decoder {
    colors: array<vec4<f32>, 24>, // Ensure 16-byte alignment for vec4<f32>
};

@group(2) @binding(0) var<uniform> size: vec2<f32>; // width, height
@group(2) @binding(1) var tile_map: texture_2d<f32>;
@group(2) @binding(2) var<uniform> decoder: Decoder;
@group(2) @binding(4) var shadow_map: texture_2d<f32>;

const CHUNK_SIZE: f32 = 600.0;
const SHADOW_RESOLUTION: f32 = 2048.;
const LEFT: f32 = -1. * 1200. / 2.;
const RIGHT: f32 = 1200. / 2.;
const TOP: f32 = 1200. / 2.;
const BOTTOM: f32 = -1. * 1200. / 2.;

const LIGHT_PROJECTION : mat3x3<f32> = mat3x3<f32>(
    2.0 / (RIGHT - LEFT), 0.0, 0.0,
    0.0, -2.0 / (TOP - BOTTOM), 0.0,
    -(RIGHT + LEFT) / (RIGHT - LEFT), (TOP + BOTTOM) / (TOP - BOTTOM), 1.0
);

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let local_coord = vec2<i32>(i32(mesh.uv.x * size.x), i32(mesh.uv.y * size.y)); //ranges 0 to CHUNK_SIZE
    let shadow: vec4<f32> = textureLoad(shadow_map, vec2<i32>(0, 0), 0);
    let tile_map_value = textureLoad(tile_map, local_coord, 0).r * 255.0;
    var color = decoder.colors[i32(tile_map_value)];
    let is_edge = local_coord.x == 0 || local_coord.x == i32(size.x) - 1 || local_coord.y == 0 || local_coord.y == i32(size.y) - 1;
    if is_edge {
        color = vec4<f32>(144/255., 238/255., 144/255., 1.0); // light green for edges
    }
    //hide ores that are in the dark
    // if (i32(tile_map_value) == 21 || i32(tile_map_value) == 4) && light < 0.2 {
    //     color = decoder.colors[5];
    // }
    let is_lit = shade(f32(local_coord.x), f32(local_coord.y));
    return vec4<f32>(
        color.r * is_lit,
        color.g * is_lit,
        color.b * is_lit,
        color.a
    );
}

fn shade(local_x: f32, local_y: f32) -> f32 {
    let global_x = get_global_x_coordinate(0., local_x);
    let global_y = get_global_y_coordinate(0., local_y);
    let light_position = LIGHT_PROJECTION * vec3<f32>(global_x, global_y, 1.0);
    let shadow_x = clamp(((light_position.x + 1.0) * 0.5) * SHADOW_RESOLUTION, 0.0, SHADOW_RESOLUTION - 1.0);
    let shadow_y = textureLoad(shadow_map, vec2<i32>(i32(shadow_x), 0), 0).x;
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