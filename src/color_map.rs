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

pub fn dirt_variant_pmf() -> u8 {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..6) {
        0 => DIRT1,
        4 => DIRT1,
        5 => DIRT1,
        1 => DIRT2,
        2 => DIRT2,
        _ => DIRT3,
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