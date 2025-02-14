
use std::collections::HashMap;

use bevy::{asset::{Asset, Assets, Handle}, ecs::{component::Component, system::Resource}, math::{Vec2, Vec4}, prelude::{Commands, Image, ResMut}, render::render_resource::{AsBindGroup, ShaderRef}, sprite::Material2d};
use bevy_reflect::TypePath;

use crate::{constants::CHUNK_SIZE, util::grid_to_image};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Component)]
pub struct GridMaterial {
    #[uniform(0)]
    pub size: Vec2,
    #[texture(1)]
    pub color_map_handle: Handle<Image>,
    #[uniform(2)]
    pub decoder: [Vec4; 24],
    #[texture(4)]
    pub shadow_map: Option<Handle<Image>>,
    #[uniform(5)]
    pub global_chunk_pos: Vec2,
    pub on_screen_chunk_position: [i8; 2],
    #[uniform(6)]
    pub player_pos: Vec2,
}

impl Material2d for GridMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/render_shader.wgsl".into()
    }
}

#[derive(Resource)]
pub struct ChunkImageHandle {
    pub handle: Handle<Image>,
}

#[derive(Resource)]
pub struct Othereers {
    pub handles: HashMap<u32, Handle<Image>>,
}

pub fn initialize_shadows(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    initialize_9_chunks(&mut images, &mut commands);
}

fn initialize_9_chunks(images: &mut Assets<Image>, commands: &mut Commands) -> Handle<Image>{
    let mut center_handle = Handle::default();
    let mut map = HashMap::new();
    for i in 0..9 {
        let image_handle = images.add(grid_to_image(&vec![188; (CHUNK_SIZE * CHUNK_SIZE) as usize], CHUNK_SIZE as u32, CHUNK_SIZE as u32, None));
        if i == 4 {
            center_handle = image_handle.clone();
            commands.insert_resource(ChunkImageHandle { handle: image_handle });
        } else {
            map.insert(i, image_handle);
        }
    }
    commands.insert_resource(Othereers { handles: map });
    center_handle
}