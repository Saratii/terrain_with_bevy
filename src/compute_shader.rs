use std::{borrow::Cow, collections::HashMap};

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
    }, sprite::MaterialMesh2dBundle,
};
use bytemuck::{Pod, Zeroable};
use wgpu::util;
use crate::{color_map::{apply_gamma_correction, RAW_DECODER_DATA}, components::TerrainImageTag, constants::{CHUNK_SIZE, SHADOW_RESOLUTION}, sun::GridMaterial, util::grid_to_image};

const SHADER_ASSET_PATH: &str = "shaders/shadow_compute.wgsl";
const INPUT_SIZE: (u32, u32) = (CHUNK_SIZE as u32 * 3, CHUNK_SIZE as u32 * 3);
const OUTPUT_SIZE: (u32, u32) = (SHADOW_RESOLUTION as u32, 1);
const WORKGROUP_SIZE: u32 = 1;

#[repr(C)]
#[derive(Resource, Copy, Clone, Pod, Zeroable)]
pub struct CurrentPlayerPosition {
    pub position: [f32; 2],
    pub _padding: [u32; 2],
}

impl Default for CurrentPlayerPosition {
    fn default() -> Self {
        Self {
            position: [0., 0.],
            _padding: [0, 0],
        }
    }
}

pub fn build_compute_shader(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<GridMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource::<CurrentPlayerPosition>(CurrentPlayerPosition::default());
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
    let shadows_output_texture_handle = images.add(output_texture);
    let mut map = HashMap::new();
    for x in [-1, 0, 1] {
        for y in [-1, 0, 1] {
            let mut input_image = grid_to_image(
                &vec![188; (CHUNK_SIZE * CHUNK_SIZE) as usize],
                CHUNK_SIZE as u32,
                CHUNK_SIZE as u32,
                None,
            );
            input_image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
            let input_handle = images.add(input_image);
            map.insert([x, y], input_handle.clone());
            commands.spawn(TerrainImageTag)
                    .insert(MaterialMesh2dBundle {
                        material: materials.add(GridMaterial {
                            color_map_handle: input_handle,
                            size: Vec2::new(CHUNK_SIZE as f32, CHUNK_SIZE as f32),
                            decoder: apply_gamma_correction(RAW_DECODER_DATA),
                            shadow_map: Some(shadows_output_texture_handle.clone()),
                            global_chunk_pos: Vec2::new(x as f32, y as f32),
                            on_screen_chunk_position: [x, y],
                            player_pos: Vec2::new(0., 0.),
                        }),
                        mesh: meshes.add(Rectangle { half_size: Vec2::new(CHUNK_SIZE/2., CHUNK_SIZE/2.) }).into(),
                        transform: Transform { translation: Vec3::new(Default::default(), Default::default(), -5.), ..Default::default() },
                        ..Default::default()
                    });
        }
    }
    commands.insert_resource(ScreenImageHandles {
        map,
        shadow_map_handle: shadows_output_texture_handle.clone(),
    });
}

#[derive(Resource, Clone, ExtractResource)]
pub struct ScreenImageHandles {
    pub map: HashMap<[i8; 2], Handle<Image>>,
    pub shadow_map_handle: Handle<Image>,
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

impl ExtractResource for CurrentPlayerPosition {
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
        app.add_plugins(ExtractResourcePlugin::<CurrentPlayerPosition>::default());
        app.add_plugins(ExtractResourcePlugin::<ScreenImageHandles>::default());
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
                BindGroupLayoutEntry {
                    binding: 5,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadOnly,
                        format: TextureFormat::R8Unorm,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 6,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadOnly,
                        format: TextureFormat::R8Unorm,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 7,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadOnly,
                        format: TextureFormat::R8Unorm,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 8,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadOnly,
                        format: TextureFormat::R8Unorm,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 9,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadOnly,
                        format: TextureFormat::R8Unorm,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 10,
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
    shadows_images: Res<ScreenImageHandles>,
    current_chunk: Res<CurrentPlayerPosition>,
    render_device: Res<RenderDevice>,
) {
    let view_a = gpu_images.get(shadows_images.map.get(&[0, 0]).unwrap()).unwrap();
    let view_b = gpu_images.get(&shadows_images.shadow_map_handle).unwrap();
    let view_left = gpu_images.get(shadows_images.map.get(&[-1, 0]).unwrap()).unwrap();
    let view_right = gpu_images.get(shadows_images.map.get(&[1, 0]).unwrap()).unwrap();
    let view_top_left = gpu_images.get(shadows_images.map.get(&[-1, 1]).unwrap()).unwrap();
    let view_top_center = gpu_images.get(shadows_images.map.get(&[0, 1]).unwrap()).unwrap();
    let view_top_right = gpu_images.get(shadows_images.map.get(&[1, 1]).unwrap()).unwrap();
    let view_bottom_left = gpu_images.get(shadows_images.map.get(&[-1, -1]).unwrap()).unwrap();
    let view_bottom_center = gpu_images.get(shadows_images.map.get(&[0, -1]).unwrap()).unwrap();
    let view_bottom_right = gpu_images.get(shadows_images.map.get(&[1, -1]).unwrap()).unwrap();
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
            BindGroupEntry {
                binding: 5,
                resource: BindingResource::TextureView(&view_top_left.texture_view),
            },
            BindGroupEntry {
                binding: 6,
                resource: BindingResource::TextureView(&view_top_center.texture_view),
            },
            BindGroupEntry {
                binding: 7,
                resource: BindingResource::TextureView(&view_top_right.texture_view),
            },
            BindGroupEntry {
                binding: 8,
                resource: BindingResource::TextureView(&view_bottom_left.texture_view),
            },
            BindGroupEntry {
                binding: 9,
                resource: BindingResource::TextureView(&view_bottom_center.texture_view),
            },
            BindGroupEntry {
                binding: 10,
                resource: BindingResource::TextureView(&view_bottom_right.texture_view),
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