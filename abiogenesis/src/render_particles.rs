use bevy::{
    core_pipeline::core_2d::graph::{Core2d, Node2d},
    prelude::*,
    render::{
        Render, RenderApp, RenderSet,
        camera::ExtractedCamera,
        render_asset::RenderAssets,
        render_graph::{RenderGraphApp, RenderLabel, ViewNode, ViewNodeRunner},
        render_resource::{
            BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, BlendState,
            Buffer, BufferInitDescriptor, BufferUsages, CachedRenderPipelineId, ColorTargetState,
            ColorWrites, FragmentState, LoadOp, MultisampleState, Operations, PipelineCache,
            PrimitiveState, RenderPassDescriptor, RenderPipelineDescriptor, ShaderStages, StoreOp,
            VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
            binding_types::{storage_buffer_read_only_sized, uniform_buffer},
        },
        renderer::{RenderContext, RenderDevice},
        storage::GpuShaderStorageBuffer,
        view::{ViewTarget, ViewUniform, ViewUniformOffset, ViewUniforms},
    },
};

use crate::compute_shader::{
    BUFFER_SIZE, NUM_PARTICLES, ParticleLifeBuffers, ParticleLifeLabel, ParticleLifeState,
};

const PARTICLE_RENDER_SHADER_PATH: &str = "shaders/particle_render.wgsl";

pub struct ParticleRenderPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct ParticleRenderLabel;

impl Plugin for ParticleRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_fps_counter);

        app.add_systems(Startup, setup_camera);

        let render_app = app.sub_app_mut(RenderApp);

        render_app.add_systems(
            Render,
            (
                setup_quad_buffer
                    .in_set(RenderSet::PrepareResources)
                    .run_if(not(resource_exists::<QuadVertexBuffer>)),
                prepare_particle_render_bind_group
                    .in_set(RenderSet::PrepareBindGroups)
                    .run_if(not(resource_exists::<ParticleRenderBindGroups>)),
            ),
        );

        // Use the modern ViewNodeRunner approach
        render_app.add_render_graph_node::<ViewNodeRunner<ParticleRenderNode>>(
            Core2d,
            ParticleRenderLabel,
        );

        // Add it to the 2D render graph after the main pass
        render_app.add_render_graph_edges(
            Core2d,
            (
                Node2d::EndMainPass,
                ParticleLifeLabel,
                ParticleRenderLabel,
                Node2d::Bloom,
            ),
        );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<ParticleRenderPipeline>();
    }
}

#[derive(Debug, Clone, Copy, Component)]
struct FPSCounter;

fn setup_camera(mut commands: Commands) {
    // Add a 2D camera to render to
    commands.spawn((
        Camera2d,
        Camera {
            // clear_color: ClearColorConfig::Custom(Color::srgb(0.0, 1.0, 0.0)),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));

    commands.spawn((
        Node {
            padding: UiRect::all(Val::Px(16.0)),
            ..default()
        },
        children![(FPSCounter, Text::from("0.0"),)],
    ));
}

fn update_fps_counter(
    mut text: Single<&mut Text, With<FPSCounter>>,
    time: Res<Time>,
    mut moving_average: Local<Option<f32>>,
) {
    let Some(moving_average) = moving_average.as_mut() else {
        *moving_average = Some(time.delta_secs());
        return;
    };

    *moving_average = *moving_average * 0.99 + time.delta_secs() * 0.01;
    ***text = format!("{:.0}", 1.0 / *moving_average);
}

#[derive(Resource)]
pub struct ParticleRenderPipeline {
    bind_group_layout: BindGroupLayout,
    render_pipeline: CachedRenderPipelineId,
}

impl FromWorld for ParticleRenderPipeline {
    fn from_world(world: &mut World) -> Self {
        tracing::error!("extracting particle render pipeline from world");

        let render_device = world.resource::<RenderDevice>();

        // Bind group layout for graphics pipeline
        let bind_group_layout = render_device.create_bind_group_layout(
            "ParticleRender",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::VERTEX,
                (
                    uniform_buffer::<ViewUniform>(true),
                    storage_buffer_read_only_sized(false, BUFFER_SIZE),
                    storage_buffer_read_only_sized(false, BUFFER_SIZE),
                ),
            ),
        );

        let shader = world.load_asset(PARTICLE_RENDER_SHADER_PATH);
        let pipeline_cache = world.resource::<PipelineCache>();

        let render_pipeline = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: Some("particle_render_pipeline".into()),
            layout: vec![bind_group_layout.clone()],
            vertex: VertexState {
                shader: shader.clone(),
                shader_defs: vec![],
                entry_point: "vertex".into(),
                buffers: vec![
                    // Quad vertex buffer layout
                    VertexBufferLayout {
                        array_stride: 12, // 3 * f32
                        step_mode: VertexStepMode::Vertex,
                        attributes: vec![VertexAttribute {
                            format: VertexFormat::Float32x3,
                            offset: 0,
                            shader_location: 0,
                        }],
                    },
                ],
            },
            fragment: Some(FragmentState {
                shader,
                shader_defs: vec![],
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: bevy::render::render_resource::TextureFormat::bevy_default(),
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
            zero_initialize_workgroup_memory: true,
        });

        ParticleRenderPipeline {
            bind_group_layout,
            render_pipeline,
        }
    }
}

#[derive(Resource)]
pub struct ParticleRenderBindGroups([BindGroup; 2]);

fn prepare_particle_render_bind_group(
    mut commands: Commands,
    render_pipeline: Res<ParticleRenderPipeline>,
    gpu_buffers: Res<RenderAssets<GpuShaderStorageBuffer>>,
    particle_life_buffers: Res<ParticleLifeBuffers>,
    view_uniforms: Res<ViewUniforms>,
    render_device: Res<RenderDevice>,
) {
    tracing::info!("preparing particle render bind groups");

    let positions_a = gpu_buffers.get(&particle_life_buffers.positions_a).unwrap();
    let positions_b = gpu_buffers.get(&particle_life_buffers.positions_b).unwrap();
    let colours = gpu_buffers.get(&particle_life_buffers.colours).unwrap();

    let bind_group_0 = render_device.create_bind_group(
        None,
        &render_pipeline.bind_group_layout,
        &BindGroupEntries::sequential((
            view_uniforms.uniforms.binding().unwrap(),
            positions_a.buffer.as_entire_buffer_binding(),
            colours.buffer.as_entire_buffer_binding(),
        )),
    );

    let bind_group_1 = render_device.create_bind_group(
        None,
        &render_pipeline.bind_group_layout,
        &BindGroupEntries::sequential((
            view_uniforms.uniforms.binding().unwrap(),
            positions_b.buffer.as_entire_buffer_binding(),
            colours.buffer.as_entire_buffer_binding(),
        )),
    );

    commands.insert_resource(ParticleRenderBindGroups([bind_group_0, bind_group_1]));
}

#[derive(Resource)]
pub struct QuadVertexBuffer {
    buffer: Buffer,
}

fn setup_quad_buffer(mut commands: Commands, render_device: Res<RenderDevice>) {
    tracing::info!("preparing quad vertex buffer");

    // Simple quad vertices (2 triangles)
    let vertices: &[f32] = &[
        // Triangle 1
        -1.0, -1.0, 0.0, // Bottom left
        1.0, -1.0, 0.0, // Bottom right
        -1.0, 1.0, 0.0, // Top left
        // Triangle 2
        1.0, -1.0, 0.0, // Bottom right
        1.0, 1.0, 0.0, // Top right
        -1.0, 1.0, 0.0, // Top left
    ];

    let vertices = unsafe {
        let inner = vertices.align_to::<u8>();
        debug_assert_eq!(inner.0.len(), 0);
        debug_assert_eq!(inner.2.len(), 0);
        debug_assert_eq!(inner.1.len(), 72);
        inner.1
    };

    let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("quad_vertex_buffer"),
        contents: vertices,
        usage: BufferUsages::VERTEX,
    });

    commands.insert_resource(QuadVertexBuffer { buffer });
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ParticleRenderNode;

impl ViewNode for ParticleRenderNode {
    type ViewQuery = (&'static ViewUniformOffset, &'static ViewTarget);

    fn run(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        (view_uniforms_offset, view_target): bevy::ecs::query::QueryItem<Self::ViewQuery>,
        world: &bevy::ecs::world::World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        // Check all resources exist
        let pipeline_cache = world.resource::<PipelineCache>();
        let render_pipeline_resource = world.resource::<ParticleRenderPipeline>();
        let bind_groups = world.resource::<ParticleRenderBindGroups>();
        let quad_buffer = world.resource::<QuadVertexBuffer>();
        let pipeline_state = world.resource::<ParticleLifeState>();

        let Some(render_pipeline) =
            pipeline_cache.get_render_pipeline(render_pipeline_resource.render_pipeline)
        else {
            return Ok(());
        };

        let current_bind_group = match pipeline_state {
            ParticleLifeState::Loading => &bind_groups.0[0],
            ParticleLifeState::Init => &bind_groups.0[0],
            &ParticleLifeState::Update(index) => &bind_groups.0[index],
        };

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("particle_render_pass"),
            color_attachments: &[Some(
                bevy::render::render_resource::RenderPassColorAttachment {
                    view: view_target.main_texture_view(),
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Load, // Load existing content from main pass
                        store: StoreOp::Store,
                    },
                },
            )],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(render_pipeline);
        render_pass.set_bind_group(0, current_bind_group, &[view_uniforms_offset.offset]);
        render_pass.set_vertex_buffer(0, quad_buffer.buffer.slice(..));

        render_pass.draw(
            0..6,                    // 6 vertices for quad (2 triangles)
            0..NUM_PARTICLES as u32, // All particles
        );

        Ok(())
    }
}
