use bevy::math::Vec4;
use rand::Rng;

pub const SKY: u8 = 0;
pub const DIRT1: u8 = 1;
pub const DIRT2: u8 = 2;
pub const DIRT3: u8 = 3;
pub const COPPER: u8 = 4;
pub const ROCK: u8 = 5;
pub const GRAVEL1: u8 = 6;
pub const GRAVEL2: u8 = 7;
pub const GRAVEL3: u8 = 8;
pub const LIGHT: u8 = 9;
pub const REFINED_COPPER: u8 = 10;
pub const SELL_BOX: u8 = 11;
pub const TRANSLUCENT_GREY: u8 = 12;
pub const CLEAR: u8 = 13;
pub const WHITE: u8 = 14;
pub const RED: u8 = 15;
pub const STEEL: u8 = 16;
pub const PLAYER_SKIN: u8 = 17;
pub const BLACK: u8 = 18;
pub const DRILL_BLACK: u8 = 19;
pub const DRILL_GREY: u8 = 20;
pub const SILVER: u8 = 21;

pub const GRAVITY_AFFECTED: [u8; 6] = [DIRT1, DIRT2, DIRT3, GRAVEL1, GRAVEL2, GRAVEL3];
pub const GROUND: [u8; 9] = [DIRT1, DIRT2, DIRT3, GRAVEL1, GRAVEL2, GRAVEL3, COPPER, SILVER, ROCK];
pub const SHOVEL_ABLE: [u8; 8] = [DIRT1, DIRT2, DIRT3, GRAVEL1, GRAVEL2, GRAVEL3, COPPER, SILVER];

pub fn dirt_variant_pmf() -> u8 {
    let rand_value = fastrand::f32();
    if rand_value < 0.5 {
        DIRT1
    } else if rand_value < 0.8333333 {
        DIRT2
    } else {
        DIRT3
    }
}

pub fn gravel_variant_pmf() -> u8 {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..6) {
        0 => GRAVEL1,
        4 => GRAVEL2,
        5 => GRAVEL3,
        1 => GRAVEL1,
        2 => GRAVEL1,
        _ => GRAVEL1,
    }
}

pub const RAW_DECODER_DATA: [(f32, f32, f32, f32); 24] = [
    (135.0 / 255.0, 206.0 / 255.0, 234.0 / 255.0, 1.0), // sky
    (88.0 / 255.0, 57.0 / 255.0, 39.0 / 255.0, 1.0),    // dirt1
    (92.0 / 255.0, 64.0 / 255.0, 51.0 / 255.0, 1.0),    // dirt2
    (155.0 / 255.0, 118.0 / 255.0, 83.0 / 255.0, 1.0),  // dirt3
    (196.0 / 255.0, 145.0 / 255.0, 2.0 / 255.0, 1.0),   // copper
    (100.0 / 255.0, 100.0 / 255.0, 100.0 / 255.0, 1.0), // rock
    (115.0 / 255.0, 115.0 / 255.0, 115.0 / 255.0, 1.0), // gravel1
    (72.0 / 255.0, 72.0 / 255.0, 72.0 / 255.0, 1.0),    // gravel2
    (220.0 / 255.0, 210.0 / 255.0, 195.0 / 255.0, 1.0), // gravel3
    (255.0 / 255.0, 255.0 / 255.0, 0.0, 1.0),           // light
    (205.0 / 255.0, 127.0 / 255.0, 50.0 / 255.0, 1.0),  // refined copper
    (106.0 / 255.0, 13.0 / 255.0, 173.0 / 255.0, 1.0),  // sell box
    (135.0 / 255.0, 206.0 / 255.0, 235.0 / 255.0, 150.0 / 255.0), // translucent grey
    (135.0 / 255.0, 206.0 / 255.0, 235.0 / 255.0, 0.0), // clear
    (255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 1.0), // white
    (255.0 / 255.0, 0.0, 0.0, 1.0),                     // red
    (176.0 / 255.0, 179.0 / 255.0, 183.0 / 255.0, 1.0), // steel
    (210.0 / 255.0, 180.0 / 255.0, 140.0 / 255.0, 1.0), // player skin
    (0.0, 0.0, 0.0, 1.0),                               // black
    (35.0 / 255.0, 36.0 / 255.0, 37.0 / 255.0, 1.0),    // drill black
    (132.0 / 255.0, 136.0 / 255.0, 136.0 / 255.0, 1.0), // drill grey
    (192.0 / 255.0, 192.0 / 255.0, 192.0 / 255.0, 1.0), // silver
    (0.0, 0.0, 0.0, 0.0),                               // empty
    (0.0, 0.0, 0.0, 0.0),                               // empty
];

pub fn inverse_gamma_correct(value: f32) -> f32 {
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

pub fn apply_gamma_correction(data: [(f32, f32, f32, f32); RAW_DECODER_DATA.len()]) -> [Vec4; RAW_DECODER_DATA.len()] {
    let mut result = [Vec4::ZERO; RAW_DECODER_DATA.len()]; // Initialize with zeroed Vec4
    let mut i = 0;

    while i < 24 {
        result[i] = Vec4::new(
            inverse_gamma_correct(data[i].0),
            inverse_gamma_correct(data[i].1),
            inverse_gamma_correct(data[i].2),
            data[i].3,
        );
        i += 1;
    }

    result
}
