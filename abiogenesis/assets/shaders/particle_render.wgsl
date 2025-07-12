#import bevy_render::view::View
#import utils::colours::COLOURS
#import utils::math::{Rect, toroidal_displacement}

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

override size = 1.0;
override sharpness = 1.0;

const BOUNDS: Rect = Rect(
    vec2<f32>(-4.0 * 1920.0 / 2.0, -4.0 * 1080.0 / 2.0),
    vec2<f32>(4.0 * 1920.0 / 2.0, 4.0 * 1080.0 / 2.0)
);

fn wrap_particle_position(particle_pos: vec2<f32>, camera_pos: vec2<f32>) -> vec2<f32> {
    let width = BOUNDS.max.x - BOUNDS.min.x;
    let height = BOUNDS.max.y - BOUNDS.min.y;

    var wrapped_pos = particle_pos;

    // Calculate the displacement from camera to particle
    let displacement = toroidal_displacement(BOUNDS, camera_pos, particle_pos);
    let direct_displacement = particle_pos - camera_pos;

    // If the shortest path crosses a boundary, render at the wrapped position
    if abs(displacement.x) < abs(direct_displacement.x) {
        if direct_displacement.x > 0.0 && displacement.x < 0.0 {
            wrapped_pos.x -= width;
        } else if direct_displacement.x < 0.0 && displacement.x > 0.0 {
            wrapped_pos.x += width;
        }
    }

    if abs(displacement.y) < abs(direct_displacement.y) {
        if direct_displacement.y > 0.0 && displacement.y < 0.0 {
            wrapped_pos.y -= height;
        } else if direct_displacement.y < 0.0 && displacement.y > 0.0 {
            wrapped_pos.y += height;
        }
    }

    return wrapped_pos;
}

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let raw_particle_pos = particle_positions[input.instance_index];

    // Extract camera position from the existing View uniform
    let camera_pos = view.world_position.xy;

    // Wrap the particle position for seamless toroidal rendering
    let wrapped_particle_pos = wrap_particle_position(raw_particle_pos, camera_pos);

    let world_position = vec4<f32>(input.position.xy * size + wrapped_particle_pos, 0.0, 1.0);
    out.clip_position = view.clip_from_world * world_position;

    out.color = COLOURS[particle_colours[input.instance_index]];
    out.fragUV = input.position.xy;

    return out;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let alpha = clamp(smoothstep(1.0, 0.0, sharpness * (dot(input.fragUV, input.fragUV) * 2.0 - 1.0)), 0.0, 1.0);

    return vec4<f32>(input.color, alpha);
}
