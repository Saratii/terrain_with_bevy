#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> size: vec2<f32>; // width, height
@group(2) @binding(1) var tile_map: texture_2d<f32>;
@group(2) @binding(2) var<uniform> decoder: array<vec4<f32>, 24>;

const CHUNK_SIZE: f32 = 600.0;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let local_coord = vec2<i32>(i32(mesh.uv.x * size.x), i32(mesh.uv.y * size.y));
    let tile_map_value = textureLoad(tile_map, local_coord, 0).r * 255.0;
    var color = decoder[i32(tile_map_value)];
    let is_edge = local_coord.x == 0 || local_coord.x == i32(size.x) - 1 || local_coord.y == 0 || local_coord.y == i32(size.y) - 1;
    if is_edge {
        color = vec4<f32>(144/255., 238/255., 144/255., 1.0); // light green for edges
    }
    return vec4<f32>(
        color.r,
        color.g,
        color.b,
        color.a
    );
}