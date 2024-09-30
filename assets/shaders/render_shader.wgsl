#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> size: vec2<f32>; // width, height
@group(2) @binding(1) var color_map: texture_2d<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_coords = vec2<i32>(i32(mesh.uv.x * size.x), i32(mesh.uv.y * size.y));
    let color_map_value = textureLoad(color_map, pixel_coords, 0).r * 255.0;
    if (color_map_value == 0) {
        return vec4<f32>(135/255., 206/255., 234/255., 1.0); //sky
    }
    if (color_map_value == 1) {
        return vec4<f32>(88/255., 57/255., 39/255., 1.0); //dirt1
    }
    if (color_map_value == 2) {
        return vec4<f32>(92/255., 64/255., 51/255., 1.0); //dirt2
    }
    if (color_map_value == 3) {
        return vec4<f32>(155/255., 118/255., 83/255., 1.0); //dirt3
    }
    if (color_map_value == 4) {
        return vec4<f32>(196/255., 145/255., 2/255., 1.0); //copper
    }
    if (color_map_value == 5) {
        return vec4<f32>(100./255., 100./255., 100./255., 1.0); //rock
    }
    if (color_map_value == 10) {
        return vec4<f32>(205/255., 127/255., 50/255., 1.0); //refined copper
    }
    if (color_map_value == 12) {
        return vec4<f32>(135/255., 206/255., 235/255., 150/255.0); //translucent grey
    }
    if (color_map_value == 13) {
        return vec4<f32>(135/255., 206/255., 235/255., 0.0); //clear
    }
    return vec4<f32>(1.0, 0.0, 1.0, 1.0); //default
}