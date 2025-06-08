use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeSeq};

use crate::particles::{
    particle::{Particle, ParticleIndex},
    simulation::{
        ATTRACTION_RADIUS_RANGE, FORCE_STRENGTH_RANGE, FRICTION_RANGE,
        PEAK_ATTRACTION_RADIUS_RANGE, SimulationParams,
    },
};

pub const PRESETS: [(&str, Model, SimulationParams); 8] = [
    (
        "The First Garden",
        Model {
            weights: [[0.3, 0.4, 0.5], [0.7, -0.4, 0.3], [-0.5, 0.5, 0.0]],
        },
        SimulationParams::DEFAULT,
    ),
    (
        "Circle of Life",
        Model {
            weights: [[-0.2, 0.2, 0.8], [0.0, 0.7, 0.3], [0.6, 0.3, -0.5]],
        },
        SimulationParams::DEFAULT,
    ),
    (
        "Jörmungandr",
        Model {
            weights: [[-0.8, 0.7, 0.7], [0.7, -0.8, 0.7], [0.3, 0.7, -0.8]],
        },
        SimulationParams {
            friction: 2.5,
            force_strength: 100.0,
            attraction_radius: 120.0,
            peak_attraction_radius: 80.0,
            repulsion_radius: 20.0,
            decay_rate: 80.0,
            ..SimulationParams::DEFAULT
        },
    ),
    (
        "Predation",
        Model {
            weights: [[0.9, -0.8, -0.9], [-0.1, 0.9, -0.4], [0.6, 0.8, -0.5]],
        },
        SimulationParams {
            friction: 2.5,
            force_strength: 80.0,
            attraction_radius: 100.0,
            peak_attraction_radius: 20.0,
            repulsion_radius: 40.0,
            decay_rate: 80.0,
            ..SimulationParams::DEFAULT
        },
    ),
    (
        "Endless Chase",
        Model {
            weights: [[1.0, 0.2, 0.0], [0.0, 1.0, 0.2], [0.2, 0.0, 1.0]],
        },
        SimulationParams::DEFAULT,
    ),
    (
        "The Trinity",
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
            ..SimulationParams::DEFAULT
        },
    ),
    (
        "Divine Engine",
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
            ..SimulationParams::DEFAULT
        },
    ),
    (
        "Heath Death",
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

#[derive(Debug, Reflect, Resource, Clone, Copy, Deref, DerefMut, Serialize, Deserialize)]
pub struct Model {
    #[deref]
    #[serde(
        serialize_with = "model_serializer",
        deserialize_with = "model_deserializer"
    )]
    pub weights: [[f32; 3]; 3],
}

fn model_serializer<S>(weights: &[[f32; 3]; 3], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(3))?;
    for row in weights {
        let formatted_row: [i32; 3] = [
            (row[0] * 100.0).round() as i32,
            (row[1] * 100.0).round() as i32,
            (row[2] * 100.0).round() as i32,
        ];
        seq.serialize_element(&formatted_row)?;
    }
    seq.end()
}

fn model_deserializer<'de, D>(deserializer: D) -> Result<[[f32; 3]; 3], D::Error>
where
    D: Deserializer<'de>,
{
    let string_weights: [[i32; 3]; 3] = Deserialize::deserialize(deserializer)?;

    let mut result = [[0.0f32; 3]; 3];
    for (i, row) in string_weights.iter().enumerate() {
        for (j, s) in row.iter().enumerate() {
            result[i][j] = *s as f32 / 100.0;
        }
    }

    Ok(result)
}

#[derive(Debug, Event, Clone, Copy, Reflect)]
pub struct Randomise;

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn randomise_model(
    _trigger: Trigger<Randomise>,
    mut model: ResMut<Model>,
    mut params: ResMut<SimulationParams>,
) {
    let mut rng = rand::thread_rng();

    // We're not truly random across the parameter range to try and encourage interesting results
    params.force_strength = rng.gen_range(40.0..=*FORCE_STRENGTH_RANGE.end());
    params.friction = rng.gen_range(
        1.0..=(params.force_strength / *FORCE_STRENGTH_RANGE.end()) * FRICTION_RANGE.end(),
    );

    params.attraction_radius = rng.gen_range(60.0..=*ATTRACTION_RADIUS_RANGE.end());
    params.peak_attraction_radius = rng.gen_range(0.0..=*PEAK_ATTRACTION_RADIUS_RANGE.end());
    params.repulsion_radius =
        rng.gen_range((params.attraction_radius / 10.0)..=(params.attraction_radius * 0.6));

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
    mut particle_index: ResMut<ParticleIndex>,
    mut params: ResMut<SimulationParams>,
) {
    // If someone clears all the particles, it's likely they want to design a creature, so disable decay.
    params.decay_rate = 0.0;

    particles.iter().for_each(|particle| {
        commands.entity(particle).despawn();
    });

    particle_index.clear();
}
