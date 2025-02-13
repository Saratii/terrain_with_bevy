use bevy::{
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::{RenderAssetUsages, RenderAssets},
        render_graph::{self, RenderGraph, RenderLabel},
        render_resource::*,
        renderer::{RenderContext, RenderDevice},
        texture::GpuImage,
        Render, RenderApp, RenderSet,
    },
};
use bytemuck::{Pod, Zeroable};
use wgpu::util;
use std::borrow::Cow;
use crate::{constants::{CHUNK_SIZE, SHADOW_RESOLUTION}, util::grid_to_image};

const SHADER_ASSET_PATH: &str = "shaders/shadow_compute.wgsl";
const INPUT_SIZE: (u32, u32) = (CHUNK_SIZE as u32 * 3, CHUNK_SIZE as u32);
const OUTPUT_SIZE: (u32, u32) = (SHADOW_RESOLUTION as u32, 1);
const WORKGROUP_SIZE: u32 = 1;

#[repr(C)]
#[derive(Resource, Copy, Clone, Pod, Zeroable)]
pub struct CurrentPlayerChunk {
    pub current_chunk: [i32; 2],
    pub _padding: [u32; 2],
}

impl Default for CurrentPlayerChunk {
    fn default() -> Self {
        Self {
            current_chunk: [0, 0],
            _padding: [0, 0],
        }
    }
}

pub fn build_compute_shader(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    commands.insert_resource::<CurrentPlayerChunk>(CurrentPlayerChunk::default());
    let mut center_input_image = grid_to_image(
        &vec![188; (CHUNK_SIZE * CHUNK_SIZE) as usize],
        CHUNK_SIZE as u32,
        CHUNK_SIZE as u32,
        None,
    );
    center_input_image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let center_input_handle = images.add(center_input_image);
    let mut left_input_image = grid_to_image(
        &vec![188; (CHUNK_SIZE * CHUNK_SIZE) as usize],
        CHUNK_SIZE as u32,
        CHUNK_SIZE as u32,
        None,
    );
    left_input_image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let left_input_handle = images.add(left_input_image);
    let mut right_input_image = grid_to_image(
        &vec![188; (CHUNK_SIZE * CHUNK_SIZE) as usize],
        CHUNK_SIZE as u32,
        CHUNK_SIZE as u32,
        None,
    );
    right_input_image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let right_input_handle = images.add(right_input_image);
    let mut top_left_input_image = grid_to_image(
        &vec![188; (CHUNK_SIZE * CHUNK_SIZE) as usize],
        CHUNK_SIZE as u32,
        CHUNK_SIZE as u32,
        None,
    );
    top_left_input_image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let top_left_input_handle = images.add(top_left_input_image);
    let mut top_center_input_image = grid_to_image(
        &vec![188; (CHUNK_SIZE * CHUNK_SIZE) as usize],
        CHUNK_SIZE as u32,
        CHUNK_SIZE as u32,
        None,
    );
    top_center_input_image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let top_center_input_handle = images.add(top_center_input_image);
    let mut top_right_input_image = grid_to_image(
        &vec![188; (CHUNK_SIZE * CHUNK_SIZE) as usize],
        CHUNK_SIZE as u32,
        CHUNK_SIZE as u32,
        None,
    );
    top_right_input_image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let top_right_input_handle = images.add(top_right_input_image);
    let mut bottom_left_input_image = grid_to_image(
        &vec![188; (CHUNK_SIZE * CHUNK_SIZE) as usize],
        CHUNK_SIZE as u32,
        CHUNK_SIZE as u32,
        None,
    );
    bottom_left_input_image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let bottom_left_input_handle = images.add(bottom_left_input_image);
    let mut bottom_center_input_image = grid_to_image(
        &vec![188; (CHUNK_SIZE * CHUNK_SIZE) as usize],
        CHUNK_SIZE as u32,
        CHUNK_SIZE as u32,
        None,
    );
    bottom_center_input_image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let bottom_center_input_handle = images.add(bottom_center_input_image);
    let mut bottom_right_input_image = grid_to_image(
        &vec![188; (CHUNK_SIZE * CHUNK_SIZE) as usize],
        CHUNK_SIZE as u32,
        CHUNK_SIZE as u32,
        None,
    );
    bottom_right_input_image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let bottom_right_input_handle = images.add(bottom_right_input_image);
    let mut output_texture = Image::new_fill(
        Extent3d {
            width: OUTPUT_SIZE.0,
            height: OUTPUT_SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::R32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    output_texture.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    commands.insert_resource(ShadowsImages {
        center_texture: center_input_handle,
        shadow_map_handle: images.add(output_texture),
        left_texture: left_input_handle,
        right_texture: right_input_handle,
        top_left_texture: top_left_input_handle,
        top_center_texture: top_center_input_handle,
        top_right_texture: top_right_input_handle,
        bottom_left_texture: bottom_left_input_handle,
        bottom_center_texture: bottom_center_input_handle,
        bottom_right_texture: bottom_right_input_handle,
    });
}

#[derive(Resource, Clone, ExtractResource)]
pub struct ShadowsImages {
    pub center_texture: Handle<Image>,
    pub shadow_map_handle: Handle<Image>,
    pub left_texture: Handle<Image>,
    pub right_texture: Handle<Image>,
    pub top_left_texture: Handle<Image>,
    pub top_center_texture: Handle<Image>,
    pub top_right_texture: Handle<Image>,
    pub bottom_left_texture: Handle<Image>,
    pub bottom_center_texture: Handle<Image>,
    pub bottom_right_texture: Handle<Image>,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct CurrentChunkUniform {
    pub current_chunk: [i32; 2],
    pub _padding: [u32; 2],
}

impl Default for CurrentChunkUniform {
    fn default() -> Self {
        Self {
            current_chunk: [0, 0],
            _padding: [0, 0],
        }
    }
}

impl ExtractResource for CurrentPlayerChunk {
    type Source = Self;
    fn extract_resource(source: &Self::Source) -> Self {
        *source
    }
}

pub struct ShadowsComputePlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct ShadowsLabel;

impl Plugin for ShadowsComputePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<CurrentPlayerChunk>::default());
        app.add_plugins(ExtractResourcePlugin::<ShadowsImages>::default());
        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            prepare_bind_group.in_set(RenderSet::PrepareBindGroups),
        );
        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        render_graph.add_node(ShadowsLabel, ShadowsNode::default());
        render_graph.add_node_edge(ShadowsLabel, bevy::render::graph::CameraDriverLabel);
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<ShadowsPipeline>();
    }
}

#[derive(Resource)]
pub struct ShadowsPipeline {
    pub texture_bind_group_layout: BindGroupLayout,
    pub init_pipeline: CachedComputePipelineId,
    pub update_pipeline: CachedComputePipelineId,
}

impl FromWorld for ShadowsPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let texture_bind_group_layout = render_device.create_bind_group_layout(
            "ShadowsImages",
            &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadOnly,
                        format: TextureFormat::R8Unorm,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadWrite,
                        format: TextureFormat::R32Float,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(16),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadOnly,
                        format: TextureFormat::R8Unorm,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 4,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadOnly,
                        format: TextureFormat::R8Unorm,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        );
        let shader = world.load_asset(SHADER_ASSET_PATH);
        let pipeline_cache = world.resource::<PipelineCache>();
        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
        });
        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });
        ShadowsPipeline {
            texture_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

#[derive(Resource)]
pub struct ShadowsImageBindGroups([BindGroup; 2]);

fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<ShadowsPipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    shadows_images: Res<ShadowsImages>,
    current_chunk: Res<CurrentPlayerChunk>,
    render_device: Res<RenderDevice>,
) {
    let view_a = gpu_images.get(&shadows_images.center_texture).unwrap();
    let view_b = gpu_images.get(&shadows_images.shadow_map_handle).unwrap();
    let view_left = gpu_images.get(&shadows_images.left_texture).unwrap();
    let view_right = gpu_images.get(&shadows_images.right_texture).unwrap();
    let current_chunk_buffer = render_device.create_buffer_with_data(&util::BufferInitDescriptor {
        label: Some("CurrentChunk Uniform Buffer"),
        contents: bytemuck::bytes_of(&*current_chunk),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });
    let bind_group = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&view_a.texture_view),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::TextureView(&view_b.texture_view),
            },
            BindGroupEntry {
                binding: 2,
                resource: current_chunk_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 3,
                resource: BindingResource::TextureView(&view_left.texture_view),
            },
            BindGroupEntry {
                binding: 4,
                resource: BindingResource::TextureView(&view_right.texture_view),
            },
        ],
    );
    commands.insert_resource(ShadowsImageBindGroups([bind_group.clone(), bind_group]));
}

enum ShadowsState {
    Loading,
    Init,
    Update,
}

struct ShadowsNode {
    state: ShadowsState,
}

impl Default for ShadowsNode {
    fn default() -> Self {
        Self {
            state: ShadowsState::Loading,
        }
    }
}

impl render_graph::Node for ShadowsNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<ShadowsPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        match self.state {
            ShadowsState::Loading => {
                match pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline) {
                    CachedPipelineState::Ok(_) => {
                        self.state = ShadowsState::Init;
                    }
                    CachedPipelineState::Err(ref err) => {
                        panic!("Initializing assets/{SHADER_ASSET_PATH}:\n{err}")
                    }
                    _ => {}
                }
            }
            ShadowsState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = ShadowsState::Update;
                }
            }
            ShadowsState::Update => {}
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let bind_groups = &world.resource::<ShadowsImageBindGroups>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<ShadowsPipeline>();
        let mut pass = render_context.command_encoder().begin_compute_pass(&ComputePassDescriptor::default());
        if let Some(init_pipeline) = pipeline_cache.get_compute_pipeline(pipeline.init_pipeline) {
            pass.set_bind_group(0, &bind_groups[0], &[]);
            pass.set_pipeline(init_pipeline);
            pass.dispatch_workgroups(INPUT_SIZE.0 / WORKGROUP_SIZE, INPUT_SIZE.1 / WORKGROUP_SIZE, 1);
        } else {
            return Ok(());
        }
        if let Some(update_pipeline) = pipeline_cache.get_compute_pipeline(pipeline.update_pipeline) {
            pass.set_bind_group(0, &bind_groups[0], &[]);
            pass.set_pipeline(update_pipeline);
            pass.dispatch_workgroups(INPUT_SIZE.0 / WORKGROUP_SIZE, INPUT_SIZE.1 / WORKGROUP_SIZE, 1);
        } else {
            return Ok(());
        }
        Ok(())
    }
}