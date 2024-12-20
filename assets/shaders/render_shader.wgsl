#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct Decoder {
    colors: array<vec4<f32>, 24>, // Ensure 16-byte alignment for vec4<f32>
};

@group(2) @binding(0) var<uniform> size: vec2<f32>; // width, height
@group(2) @binding(1) var tile_map: texture_2d<f32>;
@group(2) @binding(2) var<uniform> decoder: Decoder;
@group(2) @binding(3) var abovemap: texture_2d<f32>;
// @group(2) @binding(2) var<uniform> point: vec2<f32>; // 2D coordinate

const SUN_ANGLE: f32 = -1.7;
const SUN_VECTOR: vec2<f32> = vec2<f32>(cos(SUN_ANGLE), sin(SUN_ANGLE));
const CHUNK_SIZE: f32 = 600.0;

//expects angle in radians of non sun point
fn ray_cast(start: vec2<f32>, chunk: texture_2d<f32>) -> f32 {
    var start_x = start.x;
    var start_y = start.y;
    let dx = SUN_VECTOR.x;
    let dy = SUN_VECTOR.y;
    var cast_strength = 1.3;
    while cast_strength > 0.0 && start_y > -CHUNK_SIZE + 1 {
        start_x += dx;
        start_y += dy;
        var pixel = 0.0;
        if i32(start_y) >= 0 {
            pixel = textureLoad(chunk, vec2<i32>(i32(start_x), i32(start_y)), 0).r * 255.0;
        } else {
            pixel = textureLoad(abovemap, vec2<i32>(i32(start_x), i32(start_y) + i32(CHUNK_SIZE)), 0).r * 255.0;
        }
        if pixel == 0.0 || pixel == 10.0 || pixel == 11.0 {
            cast_strength -= 0.00;
        } else {
            cast_strength -= 0.02;
        }
    }
    let light = clamp(cast_strength, 0.0, 1.);
    return mix(0.05, light, light);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_coords = vec2<i32>(i32(mesh.uv.x * size.x), i32(mesh.uv.y * size.y));
    let tile_map_value = textureLoad(tile_map, pixel_coords, 0).r * 255.0;
    // let light = ray_cast(vec2<f32>(f32(pixel_coords.x), f32(pixel_coords.y)), tile_map);
    let light = 1.;
    var color = decoder.colors[i32(tile_map_value)];
    let is_edge = pixel_coords.x == 0 || pixel_coords.x == i32(size.x) - 1 || pixel_coords.y == 0 || pixel_coords.y == i32(size.y) - 1;
    if is_edge {
        color = vec4<f32>(144/255., 238/255., 144/255., 1.0); // light green for edges
    }
    //hide ores that are in the dark
    if (i32(tile_map_value) == 21 || i32(tile_map_value) == 4) && light < 0.2 {
        color = decoder.colors[5];
    }
    return vec4<f32>(
        color.r * light * 1.,
        color.g * light * 1.,
        color.b * light,
        color.a
    );
}