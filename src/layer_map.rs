use std::collections::HashMap;

pub const LAYER_MAP: &[&str] = &[
"sky", 
"sun",
"moon",
"cloud",
"ground",
"plant",
"player",
"collision box"
];

pub fn load_layer_map() -> HashMap<String, f32>{
    LAYER_MAP.iter().enumerate().map(|(i, &layer)| (layer.to_string(), i as f32)).collect()
}