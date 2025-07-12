#import utils::colours::NUM_COLOURS
#import utils::math::{
    lcg, random_float, remap,
    toroidal_displacement, toroidal_wrap,
    Rect, clamp_length, safe_normalize
}
#import bevy_render::view::View
#import bevy_render::globals::Globals

@group(0) @binding(0)
var<uniform> view: View;

@group(0) @binding(1)
var<uniform> globals: Globals;

@group(0) @binding(2)
var<storage, read_write> particle_colours: array<u32>;

@group(0) @binding(3)
var<storage, read> in_position: array<vec2<f32>>;

@group(0) @binding(4)
var<storage, read> in_velocity: array<vec2<f32>>;

@group(0) @binding(5)
var<storage, read_write> out_position: array<vec2<f32>>;

@group(0) @binding(6)
var<storage, read_write> out_velocity: array<vec2<f32>>;

override NUM_PARTICLES = 400u * 64u;

override x_dim = 4.0 * 1920.0;
override y_dim = 4.0 * 1080.0;


@compute @workgroup_size(64)
fn init(@builtin(global_invocation_id) id: vec3<u32>) {
    let index = id.x;

    particle_colours[index] = lcg(index) % 3;

    out_position[index] = vec2<f32>(
        mix(-x_dim / 2.0, x_dim / 2.0, random_float(index + 1283718)),
        mix(-y_dim / 2.0, y_dim / 2.0, random_float(index + 3879349)),
    );

    out_velocity[index] = vec2<f32>(0.0, 0.0);
}

override friction = 2.0;
const model = array<f32, 9>(
    0.3, 0.4, 0.5,
    0.7, -0.4, 0.3,
    -0.5, 0.5, 0.0
);

fn get_model_value(a: u32, b: u32) -> f32 {
    return model[a * 3 + b];
}

const BOUNDS: Rect = Rect(
    vec2<f32>(-4.0 * 1920.0 / 2.0, -4.0 * 1080.0 / 2.0),
    vec2<f32>(4.0 * 1920.0 / 2.0, 4.0 * 1080.0 / 2.0)
);

@compute @workgroup_size(64)
fn update(@builtin(global_invocation_id) id: vec3<u32>) {
    let index = id.x;

    var position = in_position[index];

    let lower_index = u32(((globals.time - globals.delta_time) % 60.0 / 60.0) * f32(NUM_PARTICLES));
    let upper_index = u32((globals.time % 60.0 / 60.0) * f32(NUM_PARTICLES));

    if lower_index <= index && index < upper_index {
        particle_colours[index] = lcg(index + u32(globals.time)) % 3;
        position = vec2<f32>(
            mix(-x_dim / 2.0, x_dim / 2.0, random_float(index + 1283718)),
            mix(-y_dim / 2.0, y_dim / 2.0, random_float(index + 3879349)),
        );
    }

    let friction = exp(-friction * globals.delta_time);

    var force = vec2<f32>(0.0, 0.0);

    for (var other = 0u; other < NUM_PARTICLES; other++) {
        if other == index { continue; }

        let displacement = toroidal_displacement(BOUNDS, position, in_position[other]);
        // let displacement = in_position[other] - position;

        let magnitude = influence(get_model_value(particle_colours[index], particle_colours[other]), length(displacement));

        force += 100.0 * magnitude * safe_normalize(displacement);
    }


    let velocity = in_velocity[index] + clamp_length(force, 0.0, 1000.0) * globals.delta_time;
    out_velocity[index] = clamp_length(velocity * friction, 0.0, 400.0);
    out_position[index] = toroidal_wrap(BOUNDS, position + velocity * globals.delta_time);
}

override replusion_radius = 25.0;
override peak_attraction_radius = 50.0;
override attraction_radius = 75.0;

fn influence(factor: f32, distance: f32) -> f32 {
    if distance <= replusion_radius {
        return remap(distance, 0.0, replusion_radius, -1.0, 0.0);
    } else if distance <= peak_attraction_radius {
        return remap(
            distance,
            replusion_radius,
            peak_attraction_radius,
            0.0,
            factor,
        );
    } else if distance <= attraction_radius {
        return remap(
            distance,
            66.6,
            attraction_radius,
            factor,
            0.0,
        );
    } else {
        return 0.0;
    }
}
