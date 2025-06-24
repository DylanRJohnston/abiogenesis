use std::{borrow::Cow, num::NonZeroU64};

use bevy::{
    asset::RenderAssetUsages,
    core_pipeline::core_2d::graph::Core2d,
    ecs::query::QueryItem,
    prelude::*,
    render::{
        Render, RenderApp, RenderSet,
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        globals::{GlobalsBuffer, GlobalsUniform},
        render_asset::RenderAssets,
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, BufferUsages,
            CachedComputePipelineId, CachedPipelineState, ComputePassDescriptor,
            ComputePipelineDescriptor, PipelineCache, PipelineCacheError, ShaderStages,
            binding_types::{storage_buffer_read_only_sized, storage_buffer_sized, uniform_buffer},
        },
        renderer::{RenderContext, RenderDevice},
        storage::{GpuShaderStorageBuffer, ShaderStorageBuffer},
        view::{ViewTarget, ViewUniform, ViewUniformOffset, ViewUniforms},
    },
};

pub const NUM_PARTICLES: usize = 1024;
pub const BUFFER_SIZE: Option<NonZeroU64> =
    NonZeroU64::new(1024 * 2 * std::mem::size_of::<f32>() as u64);
pub const WORKGROUP_SIZE: usize = 64;
pub const SHADER_ASSET_PATH: &str = "shaders/compute_shader.wgsl";

pub struct ComputeShaderPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct ParticleLifeLabel;

impl Plugin for ComputeShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_buffers);
        let asset_server = app.world_mut().resource_mut::<AssetServer>();

        // Leak the handle for shared shader libs to ensure they are not dropped prematurely
        let load_shader = |path: &str| Box::leak(Box::new(asset_server.load::<Shader>(path)));

        load_shader("shaders/utils/colours.wgsl");
        load_shader("shaders/utils/math.wgsl");

        app.insert_resource(ParticleLifeState::default());
        app.add_plugins(ExtractResourcePlugin::<ParticleLifeBuffers>::default());
        app.add_plugins(ExtractResourcePlugin::<ParticleLifeState>::default());

        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            prepare_bind_group
                .in_set(RenderSet::PrepareBindGroups)
                .run_if(not(resource_exists::<GameOfLifeImageBindGroups>)),
        );

        render_app
            .add_render_graph_node::<ViewNodeRunner<ParticleLifeNode>>(Core2d, ParticleLifeLabel);
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<ParticleLifePipeline>();
    }
}

#[derive(Resource, Clone, ExtractResource)]
pub struct ParticleLifeBuffers {
    pub positions_a: Handle<ShaderStorageBuffer>,
    pub velocity_a: Handle<ShaderStorageBuffer>,
    pub colours: Handle<ShaderStorageBuffer>,
    pub positions_b: Handle<ShaderStorageBuffer>,
    pub velocity_b: Handle<ShaderStorageBuffer>,
}

#[derive(Resource)]
struct GameOfLifeImageBindGroups([BindGroup; 2]);

fn setup_buffers(mut commands: Commands, mut buffers: ResMut<Assets<ShaderStorageBuffer>>) {
    let size = BUFFER_SIZE.unwrap().get() as usize;

    let mut positions_a = ShaderStorageBuffer::with_size(size, RenderAssetUsages::RENDER_WORLD);
    positions_a.buffer_description.usage |= BufferUsages::COPY_SRC;
    let positions_a = buffers.add(positions_a);

    let velocity_a = buffers.add(ShaderStorageBuffer::with_size(
        size,
        RenderAssetUsages::RENDER_WORLD,
    ));

    let colours = buffers.add(ShaderStorageBuffer::with_size(
        size,
        RenderAssetUsages::RENDER_WORLD,
    ));

    let mut positions_b = ShaderStorageBuffer::with_size(size, RenderAssetUsages::RENDER_WORLD);
    positions_b.buffer_description.usage |= BufferUsages::COPY_SRC;
    let positions_b = buffers.add(positions_b);

    let velocity_b = buffers.add(ShaderStorageBuffer::with_size(
        size,
        RenderAssetUsages::RENDER_WORLD,
    ));

    commands.insert_resource(ParticleLifeBuffers {
        positions_a: positions_a.clone(),
        velocity_a: velocity_a,
        colours: colours,
        positions_b: positions_b.clone(),
        velocity_b: velocity_b,
    });

    // commands
    //     .spawn(Readback::buffer(positions_a))
    //     .observe(|trigger: Trigger<ReadbackComplete>| {
    //         tracing::info!(data = ?trigger.event().to_shader_type::<Vec<f32>>(), "Buffer A Readback complete");
    //     });

    // commands
    //     .spawn(Readback::buffer(positions_b))
    //     .observe(|trigger: Trigger<ReadbackComplete>| {
    //         tracing::info!(data = ?trigger.event().to_shader_type::<Vec<f32>>(), "Buffer B Readback complete");
    //     });
}

fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<ParticleLifePipeline>,
    gpu_buffers: Res<RenderAssets<GpuShaderStorageBuffer>>,
    particle_life_buffers: Res<ParticleLifeBuffers>,
    view_uniforms: Res<ViewUniforms>,
    globals_buffer: Res<GlobalsBuffer>,
    render_device: Res<RenderDevice>,
) {
    let positions_a = gpu_buffers.get(&particle_life_buffers.positions_a).unwrap();
    let velocity_a = gpu_buffers.get(&particle_life_buffers.velocity_a).unwrap();
    let colours = gpu_buffers.get(&particle_life_buffers.colours).unwrap();
    let positions_b = gpu_buffers.get(&particle_life_buffers.positions_b).unwrap();
    let velocity_b = gpu_buffers.get(&particle_life_buffers.velocity_b).unwrap();

    let bind_group_0 = render_device.create_bind_group(
        None,
        &pipeline.bind_group_layout,
        &BindGroupEntries::sequential((
            view_uniforms.uniforms.binding().unwrap(),
            globals_buffer.buffer.binding().unwrap(),
            colours.buffer.as_entire_buffer_binding(),
            positions_a.buffer.as_entire_buffer_binding(),
            velocity_a.buffer.as_entire_buffer_binding(),
            positions_b.buffer.as_entire_buffer_binding(),
            velocity_b.buffer.as_entire_buffer_binding(),
        )),
    );

    let bind_group_1 = render_device.create_bind_group(
        None,
        &pipeline.bind_group_layout,
        &BindGroupEntries::sequential((
            view_uniforms.uniforms.binding().unwrap(),
            globals_buffer.buffer.binding().unwrap(),
            colours.buffer.as_entire_buffer_binding(),
            positions_b.buffer.as_entire_buffer_binding(),
            velocity_b.buffer.as_entire_buffer_binding(),
            positions_a.buffer.as_entire_buffer_binding(),
            velocity_a.buffer.as_entire_buffer_binding(),
        )),
    );

    commands.insert_resource(GameOfLifeImageBindGroups([bind_group_0, bind_group_1]));
}

#[derive(Resource)]
struct ParticleLifePipeline {
    bind_group_layout: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

impl FromWorld for ParticleLifePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let buffer_bind_group_layout = render_device.create_bind_group_layout(
            "ParticleLifeBuffers",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    uniform_buffer::<ViewUniform>(true),
                    uniform_buffer::<GlobalsUniform>(false),
                    storage_buffer_sized(false, BUFFER_SIZE),
                    storage_buffer_read_only_sized(false, BUFFER_SIZE),
                    storage_buffer_read_only_sized(false, BUFFER_SIZE),
                    storage_buffer_sized(false, BUFFER_SIZE),
                    storage_buffer_sized(false, BUFFER_SIZE),
                ),
            ),
        );
        let shader = world.load_asset(SHADER_ASSET_PATH);
        let pipeline_cache = world.resource::<PipelineCache>();
        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![buffer_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
            zero_initialize_workgroup_memory: false,
        });
        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![buffer_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
            zero_initialize_workgroup_memory: false,
        });

        ParticleLifePipeline {
            bind_group_layout: buffer_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource, ExtractResource, Default)]
pub enum ParticleLifeState {
    #[default]
    Loading,
    Init,
    Update(usize),
}

#[derive(Default)]
struct ParticleLifeNode;

impl ViewNode for ParticleLifeNode {
    type ViewQuery = (&'static ViewUniformOffset, &'static ViewTarget);

    fn update(&mut self, world: &mut World) {
        // SAFETY: ParticleLifeState is unaliased in this function and so this is safe
        // Rust's borrow checker isn't smart enough to know this doesn't conflict with the immutable borrows below
        let pipeline_state =
            unsafe { &mut *(world.resource_mut::<ParticleLifeState>().into_inner() as *mut _) };

        let pipeline = world.resource::<ParticleLifePipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match pipeline_state {
            ParticleLifeState::Loading => {
                match pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline) {
                    CachedPipelineState::Ok(_) => {
                        *pipeline_state = ParticleLifeState::Init;
                    }
                    // If the shader hasn't loaded yet, just wait.
                    CachedPipelineState::Err(PipelineCacheError::ShaderNotLoaded(_)) => {}
                    CachedPipelineState::Err(err) => {
                        panic!("Initializing assets/{SHADER_ASSET_PATH}:\n{err}")
                    }
                    _ => {}
                }
            }
            ParticleLifeState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    *pipeline_state = ParticleLifeState::Update(1);
                }
            }
            ParticleLifeState::Update(0) => {
                *pipeline_state = ParticleLifeState::Update(1);
            }
            ParticleLifeState::Update(1) => {
                *pipeline_state = ParticleLifeState::Update(0);
            }
            ParticleLifeState::Update(_) => unreachable!(),
        }
    }

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_uniforms_offset, view_target): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let bind_groups = &world.resource::<GameOfLifeImageBindGroups>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<ParticleLifePipeline>();
        let pipeline_state = world.resource::<ParticleLifeState>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        // select the pipeline based on the current state
        match pipeline_state {
            ParticleLifeState::Loading => {}
            ParticleLifeState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_groups[0], &[view_uniforms_offset.offset]);
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups((NUM_PARTICLES / WORKGROUP_SIZE) as u32, 1, 1);
            }
            &ParticleLifeState::Update(index) => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_groups[index], &[view_uniforms_offset.offset]);
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups((NUM_PARTICLES / WORKGROUP_SIZE) as u32, 1, 1);
            }
        }

        Ok(())
    }
}
