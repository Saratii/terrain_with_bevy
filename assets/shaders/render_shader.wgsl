#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> size: vec2<f32>; // width, height
@group(2) @binding(1) var tile_map: texture_2d<f32>;

fn inverse_gamma_correct(value: f32) -> f32 {
    if (value <= 0.04045) {
        return value / 12.92;
    }
    return pow((value + 0.055) / 1.055, 2.4);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_coords = vec2<i32>(i32(mesh.uv.x * size.x), i32(mesh.uv.y * size.y));
    let tile_map_value = textureLoad(tile_map, pixel_coords, 0).r * 255.0;
    var color = vec4<f32>(9., 1.0, 1.0, 1.0); //default
    
    if (tile_map_value == 0) {
        color = vec4<f32>(135/255., 206/255., 234/255., 1.0); //sky
    } else if (tile_map_value == 1) {
        color = vec4<f32>(88/255., 57/255., 39/255., 1.0); //dirt1
    } else if (tile_map_value == 2) {
        color = vec4<f32>(92/255., 64/255., 51/255., 1.0); //dirt2
    } else if (tile_map_value == 3) {
        color = vec4<f32>(155/255., 118/255., 83/255., 1.0); //dirt3
    } else if (tile_map_value == 4) {
        color = vec4<f32>(196/255., 145/255., 2/255., 1.0); //copper
    } else if (tile_map_value == 5) {
        color = vec4<f32>(100./255., 100./255., 100./255., 1.0); //rock
    } else if (tile_map_value == 6) {
        color = vec4<f32>(115/255., 115/255., 115/255., 1.0); //gravel1
    } else if (tile_map_value == 7) {
        color = vec4<f32>(72/255., 72/255., 72/255., 1.0); //gravel2
    } else if (tile_map_value == 8) {
        color = vec4<f32>(220/255., 210/255., 195/255., 1.0); //gravel3
    } else if (tile_map_value == 9) {
        color = vec4<f32>(255/255., 255/255., 0., 1.0); //light
    } else if (tile_map_value == 10) {
        color = vec4<f32>(205/255., 127/255., 50/255., 1.0); //refined copper
    } else if (tile_map_value == 11) {
        color = vec4<f32>(106/255., 13/255., 173/255., 1.0); //sell box
    } else if (tile_map_value == 12) {
        color = vec4<f32>(135/255., 206/255., 235/255., 150/255.0); //translucent grey
    } else if (tile_map_value == 13) {
        color = vec4<f32>(135/255., 206/255., 235/255., 0.0); //clear
    } else if (tile_map_value == 14) {
        color = vec4<f32>(255., 255., 255., 1.0); //white
    } else if (tile_map_value == 15) {
        color = vec4<f32>(255., 0., 0., 1.0); //red
    } else if (tile_map_value == 16) {
        color = vec4<f32>(176/255., 179/255., 183/255., 1.0); //steel
    } else if (tile_map_value == 17) {
        color = vec4<f32>(210/255., 180/255., 140/255., 1.0); //player skin
    } else if (tile_map_value == 18) {
        color = vec4<f32>(0., 0., 0., 1.0); //black
    } else if (tile_map_value == 19) {
        color = vec4<f32>(35/255., 36/255., 37/255., 1.0); //drill black
    } else if (tile_map_value == 20) {
        color = vec4<f32>(132/255., 136/255., 136/255., 1.0); //drill grey
    }
    let is_edge = pixel_coords.x == 0 || pixel_coords.x == i32(size.x) - 1 ||
                  pixel_coords.y == 0 || pixel_coords.y == i32(size.y) - 1;

    if is_edge {
        color = vec4<f32>(144/255., 238/255., 144/255., 1.0); // light green for edges
    }
    return vec4<f32>(
        inverse_gamma_correct(color.r),
        inverse_gamma_correct(color.g),
        inverse_gamma_correct(color.b),
        color.a
    );
}