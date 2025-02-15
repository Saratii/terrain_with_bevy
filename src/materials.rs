
use bevy::{asset::{Asset, Handle}, ecs::component::Component, math::{Vec2, Vec4}, prelude::Image, render::render_resource::{AsBindGroup, Buffer, ShaderRef}, sprite::Material2d};
use bevy_reflect::TypePath;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Component)]
pub struct DefaultMaterial {
    #[uniform(0)]
    pub size: Vec2,
    #[texture(1)]
    pub color_map_handle: Handle<Image>,
    #[uniform(2)]
    pub decoder: [Vec4; 24],
}

impl Material2d for DefaultMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/default_texture_shader.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Component)]
pub struct GridMaterial {
    #[uniform(0)]
    pub size: Vec2,
    #[texture(1)]
    pub color_map_handle: Handle<Image>,
    #[uniform(2)]
    pub decoder: [Vec4; 24],
    #[uniform(5)]
    pub global_chunk_pos: Vec2,
    pub on_screen_chunk_position: [i8; 2],
    #[uniform(6)]
    pub player_pos: Vec2,
    #[storage(4, read_only, buffer)]
    pub shadow_map: Buffer,
}

impl Material2d for GridMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/render_shader.wgsl".into()
    }
}