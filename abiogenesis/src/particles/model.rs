use bevy::prelude::*;

use crate::particles::{
    simulation::{Particle, SimulationParams},
    spawner::ParticleIndexes,
};

pub const PRESETS: [(&str, Model, SimulationParams); 7] = [
    (
        "Mantaray",
        Model {
            weights: [[0.3, 0.4, 0.5], [0.7, -0.4, 0.3], [-0.5, 0.5, 0.0]],
        },
        SimulationParams::DEFAULT,
    ),
    (
        "Life Cycle",
        Model {
            weights: [[-0.2, 0.2, 0.8], [0.0, 0.7, 0.3], [0.6, 0.3, -0.5]],
        },
        SimulationParams::DEFAULT,
    ),
    (
        "SNAKES!",
        Model {
            weights: [[-0.8, 0.7, 0.7], [0.7, -0.8, 0.7], [0.3, 0.7, -0.8]],
        },
        SimulationParams::DEFAULT,
    ),
    (
        "Spinors",
        Model {
            weights: [[1.0, 0.2, 0.0], [0.0, 1.0, 0.2], [0.2, 0.0, 1.0]],
        },
        SimulationParams::DEFAULT,
    ),
    (
        "Trisolaris",
        Model {
            weights: [[0.3, 0.4, 0.5], [0.7, -0.4, 0.3], [-0.5, 0.5, 0.0]],
        },
        SimulationParams {
            friction: 5.0,
            force_strength: 180.0,
            attraction_radius: 200.0,
            peak_attraction_radius: 120.0,
            repulsion_radius: 110.0,
            decay_rate: 60.0,
        },
    ),
    (
        "Hypersphere",
        Model {
            weights: [[-0.1, 0.7, 0.0], [0.0, -0.1, 0.7], [0.7, 0.0, -0.1]],
        },
        SimulationParams {
            friction: 5.0,
            force_strength: 120.,
            attraction_radius: 200.0,
            peak_attraction_radius: 120.0,
            repulsion_radius: 40.0,
            decay_rate: 100.0,
        },
    ),
    (
        "Blank",
        Model {
            weights: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
        },
        SimulationParams::DEFAULT,
    ),
];

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PRESETS[0].1)
            .add_observer(randomise_model)
            .add_observer(clear_particles);
    }
}

#[derive(Debug, Reflect, Resource, Clone, Copy, Deref, DerefMut)]
pub struct Model {
    #[deref]
    pub weights: [[f32; 3]; 3],
}

#[derive(Debug, Event, Clone, Copy, Reflect)]
pub struct Randomise;

fn randomise_model(_: Trigger<Randomise>, mut model: ResMut<Model>) {
    model.weights.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|value| {
            *value = rand::random::<f32>() * 2.0 - 1.0;
        })
    });
}

#[derive(Debug, Event, Clone, Copy, Reflect)]
pub struct ClearParticles;

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn clear_particles(
    _trigger: Trigger<ClearParticles>,
    particles: Query<Entity, With<Particle>>,
    mut commands: Commands,
    mut particle_index: ResMut<ParticleIndexes>,
) {
    particles.iter().for_each(|particle| {
        commands.entity(particle).despawn();
    });

    particle_index.clear();
}
