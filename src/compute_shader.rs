use bevy::{
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::{RenderAssetUsages, RenderAssets},
        render_graph::{self, RenderGraph, RenderLabel},
        render_resource::{binding_types::texture_storage_2d, *},
        renderer::{RenderContext, RenderDevice},
        texture::GpuImage,
        Render, RenderApp, RenderSet,
    },
};
use std::borrow::Cow;
use crate::{constants::{CHUNK_SIZE, SHADOW_RESOLUTION}, sun::{ChunkImageHandle, Othereers}, util::grid_to_image};

const SHADER_ASSET_PATH: &str = "shaders/shadow_compute.wgsl";
const INPUT_SIZE: (u32, u32) = (CHUNK_SIZE as u32 * 3, CHUNK_SIZE as u32);
const OUTPUT_SIZE: (u32, u32) = (SHADOW_RESOLUTION as u32, 1);
const WORKGROUP_SIZE: u32 = 1;

pub fn build_compute_shader(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    center_chunk_handle: Res<ChunkImageHandle>,
    mut others: ResMut<Othereers>,
) {
    let input_texture = images.get_mut(&center_chunk_handle.handle).unwrap();
    input_texture.texture_descriptor.usage = TextureUsages::COPY_DST
        | TextureUsages::STORAGE_BINDING
        | TextureUsages::TEXTURE_BINDING;
    let mut left_input_image = grid_to_image(&vec![188; (CHUNK_SIZE * CHUNK_SIZE) as usize], CHUNK_SIZE as u32, CHUNK_SIZE as u32, None);
    left_input_image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let left_input_handle = images.add(left_input_image);
    others.handles.insert(3, left_input_handle.clone());
    let mut right_input_image = grid_to_image(&vec![188; (CHUNK_SIZE * CHUNK_SIZE) as usize], CHUNK_SIZE as u32, CHUNK_SIZE as u32, None);
    right_input_image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let right_input_handle = images.add(right_input_image);
    others.handles.insert(5, right_input_handle.clone());
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
    let output_texture_handle = images.add(output_texture);
    commands.insert_resource(ShadowsImages {
        texture_a: center_chunk_handle.handle.clone(),
        texture_b: output_texture_handle,
        texture_left: left_input_handle,
        texture_right: right_input_handle,
    });
}

#[derive(Resource, Clone, ExtractResource)]
pub struct ShadowsImages {
    pub texture_a: Handle<Image>,
    pub texture_b: Handle<Image>,
    pub texture_left: Handle<Image>,
    pub texture_right: Handle<Image>,
}
pub struct ShadowsComputePlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct ShadowsLabel;

impl Plugin for ShadowsComputePlugin {
    fn build(&self, app: &mut App) {
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
    render_device: Res<RenderDevice>,
) {
    let view_a = gpu_images.get(&shadows_images.texture_a).unwrap();
    let view_b = gpu_images.get(&shadows_images.texture_b).unwrap();
    let view_left = gpu_images.get(&shadows_images.texture_left).unwrap();
    let view_right = gpu_images.get(&shadows_images.texture_right).unwrap();
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