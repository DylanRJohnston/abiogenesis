use std::ops::RangeInclusive;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    math::{TorodialMath, remap},
    particles::{
        colour::ParticleColour,
        model::{Model, PRESETS},
        particle::{Particle, Velocity},
        size::SimulationSize,
        spatial_index::SpatialIndex,
    },
    systems::AppSystems,
};

pub struct SimulationPlugin;
impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SimulationParams>()
            .insert_resource(PRESETS[0].3)
            .add_systems(Update, compute_forces.in_set(AppSystems::Update));
    }
}

pub const FRICTION_RANGE: RangeInclusive<f32> = 0.0..=5.0;
pub const FORCE_STRENGTH_RANGE: RangeInclusive<f32> = 0.0..=200.0;
pub const ATTRACTION_RADIUS_RANGE: RangeInclusive<f32> = 0.0..=200.0;
pub const PEAK_ATTRACTION_RADIUS_RANGE: RangeInclusive<f32> = 0.0..=200.0;
pub const REPULSION_RADIUS_RANGE: RangeInclusive<f32> = 0.0..=200.0;
pub const DECAY_RATE_RANGE: RangeInclusive<f32> = 0.0..=200.0;

#[derive(Debug, Reflect, Resource, Clone, Copy, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct SimulationParams {
    pub friction: f32,
    pub force_strength: f32,
    pub peak_attraction_radius: f32,
    pub repulsion_radius: f32,
    pub attraction_radius: f32,
    pub decay_rate: f32,
    pub num_colours: usize,
}

const INTERACTION_RADIUS: f32 = 75.0;

impl SimulationParams {
    pub const DEFAULT: Self = Self {
        friction: 2.0,
        force_strength: 100.0,
        peak_attraction_radius: 2.0 * INTERACTION_RADIUS / 3.0,
        repulsion_radius: INTERACTION_RADIUS / 3.0,
        attraction_radius: INTERACTION_RADIUS,
        decay_rate: 100.0,
        num_colours: 6,
    };
}

impl Default for SimulationParams {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn compute_forces(
    mut particles: Query<(Entity, &mut Transform, &mut Velocity, &ParticleColour), With<Particle>>,
    mut spatial_index: ResMut<SpatialIndex>,
    model: Res<Model>,
    params: Res<SimulationParams>,
    simulation_size: SimulationSize,
    time: Res<Time>,
) -> Result<()> {
    let bounds = Rect::from_center_size(Vec2::ZERO, simulation_size.dimensions());

    spatial_index.clear();
    particles.iter().for_each(|(entity, transform, _, color)| {
        spatial_index.insert(transform.translation.truncate(), (entity, *color));
    });

    let dt = time.delta_secs();
    let friction_factor = (-params.friction * dt).exp();

    // https://github.com/TheBevyFlock/bevy_simple_subsecond_system/issues/26
    #[cfg(feature = "hot_reload")]
    let iter = particles.iter_mut();

    #[cfg(not(feature = "hot_reload"))]
    let iter = particles.par_iter_mut();

    iter.for_each(|(entity, mut transform, mut velocity, a_color)| {
        // Too many particles being in the same place is bad for performance
        // It is the degenerate case for the spatial hash
        // let too_crowded = spatial_index
        //     .query(
        //         transform.translation.truncate(),
        //         params.attraction_radius * 2.0,
        //     )
        //     .nth(500)
        //     .is_some();

        let too_crowded = false;

        let force = spatial_index
            .query(transform.translation.truncate(), params.attraction_radius)
            .filter(|(_, (it, _))| *it != entity)
            .map(|(b_position, (_, b_color))| {
                let displacement =
                    bounds.toroidal_displacement(transform.translation.truncate(), b_position);

                let magnitude = magnitude(
                    &params,
                    model.weight(*a_color, *b_color),
                    displacement.length(),
                );

                // Too crowded, turn off attractive forces
                if too_crowded && magnitude > 0.0 {
                    return Vec2::ZERO;
                }

                magnitude * params.force_strength * displacement.normalize()
            })
            .sum::<Vec2>();

        **velocity += force * dt;
        **velocity *= friction_factor;
        **velocity = velocity.clamp_length(0.0, 200.0);

        transform.translation = bounds
            .toroidal_wrap(transform.translation.truncate() + **velocity * dt)
            .extend(0.0);
    });

    Ok(())
}

fn magnitude(params: &SimulationParams, factor: f32, distance: f32) -> f32 {
    if distance <= params.repulsion_radius {
        remap(distance, 0.0, params.repulsion_radius, -1.0, 0.0)
    } else if distance <= params.peak_attraction_radius {
        remap(
            distance,
            params.repulsion_radius,
            params.peak_attraction_radius,
            0.0,
            factor,
        )
    } else {
        remap(
            distance,
            params.peak_attraction_radius,
            params.attraction_radius,
            factor,
            0.0,
        )
    }
}

#[cfg(test)]
mod test {

    mod influence {
        use crate::{
            math::lerp,
            particles::simulation::{SimulationParams, magnitude},
        };

        fn approx_eq(a: f32, b: f32) -> bool {
            (a - b).abs() < 0.00001
        }

        #[test]
        fn repulsed_self() {
            assert!(
                approx_eq(magnitude(&SimulationParams::DEFAULT, 1.0, 0.0), -1.0),
                "Expected influence to be -1.0, but got {}",
                magnitude(&SimulationParams::DEFAULT, 1.0, 0.0)
            );
        }

        #[test]
        fn repulsed_other() {
            assert!(
                approx_eq(
                    magnitude(
                        &SimulationParams::DEFAULT,
                        -1.0,
                        SimulationParams::DEFAULT.peak_attraction_radius
                    ),
                    -1.0
                ),
                "Expected influence to be -1.0, but got {}",
                magnitude(
                    &SimulationParams::DEFAULT,
                    -1.0,
                    SimulationParams::DEFAULT.peak_attraction_radius
                )
            );
        }

        #[test]
        fn balanced() {
            assert!(
                approx_eq(
                    magnitude(
                        &SimulationParams::DEFAULT,
                        1.0,
                        SimulationParams::DEFAULT.repulsion_radius
                    ),
                    0.0
                ),
                "Expected influence to be 0.0, but got {}",
                magnitude(
                    &SimulationParams::DEFAULT,
                    1.0,
                    SimulationParams::DEFAULT.repulsion_radius
                )
            );
        }

        #[test]
        fn attracted() {
            assert!(
                approx_eq(
                    magnitude(
                        &SimulationParams::DEFAULT,
                        1.0,
                        SimulationParams::DEFAULT.peak_attraction_radius
                    ),
                    1.0
                ),
                "Expected influence to be 1.0, but got {}",
                magnitude(
                    &SimulationParams::DEFAULT,
                    1.0,
                    SimulationParams::DEFAULT.peak_attraction_radius
                )
            );
        }

        #[test]
        fn halfway_repulsed() {
            assert!(
                approx_eq(
                    magnitude(
                        &SimulationParams::DEFAULT,
                        1.0,
                        SimulationParams::DEFAULT.repulsion_radius / 2.0
                    ),
                    -0.5
                ),
                "Expected influence to be -0.5, but got {}",
                magnitude(
                    &SimulationParams::DEFAULT,
                    1.0,
                    SimulationParams::DEFAULT.repulsion_radius / 2.0
                )
            );
        }

        #[test]
        fn halfway_attracted() {
            assert!(
                approx_eq(
                    magnitude(
                        &SimulationParams::DEFAULT,
                        1.0,
                        lerp(
                            SimulationParams::DEFAULT.repulsion_radius,
                            SimulationParams::DEFAULT.peak_attraction_radius,
                            0.5
                        )
                    ),
                    0.5
                ),
                "Expected influence to be 0.5, but got {}",
                magnitude(
                    &SimulationParams::DEFAULT,
                    1.0,
                    lerp(
                        SimulationParams::DEFAULT.repulsion_radius,
                        SimulationParams::DEFAULT.peak_attraction_radius,
                        0.5
                    )
                )
            );
        }

        #[test]
        fn halfway_attracted_otherside() {
            assert!(
                approx_eq(
                    magnitude(
                        &SimulationParams::DEFAULT,
                        1.0,
                        lerp(
                            SimulationParams::DEFAULT.peak_attraction_radius,
                            SimulationParams::DEFAULT.attraction_radius,
                            0.5
                        )
                    ),
                    0.5
                ),
                "Expected influence to be 0.5, but got {}",
                magnitude(
                    &SimulationParams::DEFAULT,
                    1.0,
                    lerp(
                        SimulationParams::DEFAULT.peak_attraction_radius,
                        SimulationParams::DEFAULT.attraction_radius,
                        0.5
                    )
                )
            );
        }
    }
}
