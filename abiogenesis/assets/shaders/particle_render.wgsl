#import bevy_render::view::View
#import utils::colours::COLOURS

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) fragUV: vec2<f32>,
    @location(1) color: vec3<f32>,
}

@group(0) @binding(0) var<uniform> view: View;
@group(0) @binding(1) var<storage, read> particle_positions: array<vec2<f32>>;
@group(0) @binding(2) var<storage, read> particle_colours: array<u32>;

override size = 2.0;
override sharpness = 1.0;

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Get the particle position for this instance
    let particle_pos = particle_positions[input.instance_index];

    // Transform the quad vertex by the particle position
    let world_position = vec4<f32>(input.position.xy * size + particle_pos, 0.0, 1.0); // Small quads at particle positions
    out.clip_position = view.clip_from_world * world_position;

    // Color based on position for visual variety
    let normalized_pos = (particle_pos + 2.0) / 4.0; // Normalize to 0-1 range

    out.color = COLOURS[ particle_colours[input.instance_index] ];
    out.fragUV = input.position.xy;

    return out;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let alpha = clamp(smoothstep(1.0, 0.0, sharpness * (dot(input.fragUV, input.fragUV) * 2.0 - 1.0)), 0.0, 1.0);

    return vec4<f32>(input.color, alpha);
}
