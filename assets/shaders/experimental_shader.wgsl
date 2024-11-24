#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct Decoder {
    colors: array<vec4<f32>, 24>, // Ensure 16-byte alignment for vec4<f32>
};

@group(2) @binding(0) var<uniform> size: vec2<f32>; // width, height
@group(2) @binding(1) var tile_map: texture_2d<f32>;
@group(2) @binding(2) var<uniform> decoder: Decoder;

const SUN_X: f32 = 600.;
const SUN_Y: f32 = 1000.;
const SUN_POS: vec2<f32> = vec2<f32>(SUN_X, SUN_Y);
const INTENSITY: i32 = 30;

fn swangus(point: vec2<i32>) -> f32 {
    let point2 = vec2<f32>(f32(point.x), f32(point.y));
    var x_sum = 0.0;
    var y_sum = 0.0;
    var x_count = 0.0;
    var y_count = 0.0;
    if point.x >= INTENSITY / 2 && point.y >= INTENSITY / 2 && point.x < 1200 - 1 - INTENSITY/2 && point.y < 1200 - 1 - INTENSITY/2 {
        for (var x = point.x - (INTENSITY / 2); x < point.x + (INTENSITY / 2); x = x + 1) {
            for (var y = point.y - (INTENSITY / 2); y < point.y + (INTENSITY / 2); y = y + 1) {
                let distance = sqrt((f32(point.x) - f32(x)) * (f32(point.x) - f32(x)) + (f32(point.y) - f32(y)) * (f32(point.y) - f32(y)));
                if distance < f32(INTENSITY/2) {
                    if textureLoad(tile_map, vec2<i32>(x, y), 0).r * 255.0 == 0.0 {
                        x_sum += f32(x);
                        y_sum += f32(y);
                        x_count += 1.0;
                        y_count += 1.0;
                    }
                }
            }
        }
    }
    let average = vec2<f32>(x_sum / x_count, y_sum / y_count);
    let sun_vector = vec2<f32>(1.0/sqrt(2.0), 1.0/sqrt(2.0));
    let normal_vector = normalize(point2 - average);
    return abs(clamp(dot(normal_vector, sun_vector), -1., 0.));
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_coords = vec2<i32>(i32(mesh.uv.x * size.x), i32(mesh.uv.y * size.y));
    let tile_map_value = textureLoad(tile_map, pixel_coords, 0).r * 255.0;
    var light = swangus(pixel_coords);
    var color = decoder.colors[i32(tile_map_value)];
    let is_edge = pixel_coords.x == 0 || pixel_coords.x == i32(size.x) - 1 || pixel_coords.y == 0 || pixel_coords.y == i32(size.y) - 1;
    if is_edge {
        color = vec4<f32>(144/255., 238/255., 144/255., 1.0); // light green for edges
    }
    return vec4<f32>(
        color.r * light,
        color.g * light,
        color.b * light,
        color.a
    );
}