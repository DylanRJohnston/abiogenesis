use std::sync::LazyLock;

use bevy::prelude::*;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeSeq};

use crate::particles::{
    colour::{NUM_COLOURS, ParticleColour},
    particle::{Particle, ParticleIndex},
    simulation::{
        ATTRACTION_RADIUS_RANGE, FORCE_STRENGTH_RANGE, FRICTION_RANGE,
        PEAK_ATTRACTION_RADIUS_RANGE, REPULSION_RADIUS_RANGE, SimulationParams,
    },
};

pub const NUM_PRESETS: usize = 8;
pub const PRESETS: LazyLock<[(&str, Model, SimulationParams); NUM_PRESETS]> = LazyLock::new(|| {
    [
        (
            "The First Garden",
            Model::from_3x3(
                [[0.3, 0.4, 0.5], [0.7, -0.4, 0.3], [-0.5, 0.5, 0.0]],
                NUM_COLOURS,
            ),
            SimulationParams {
                num_colours: 3,
                ..SimulationParams::DEFAULT
            },
        ),
        (
            "Circle of Life",
            Model::from_3x3(
                [[-0.2, 0.2, 0.8], [0.0, 0.7, 0.3], [0.6, 0.3, -0.5]],
                NUM_COLOURS,
            ),
            SimulationParams {
                num_colours: 3,
                ..SimulationParams::DEFAULT
            },
        ),
        (
            "Jörmungandr",
            Model::from_3x3(
                [[-0.8, 0.7, 0.7], [0.7, -0.8, 0.7], [0.3, 0.7, -0.8]],
                NUM_COLOURS,
            ),
            SimulationParams {
                friction: 2.5,
                force_strength: 100.0,
                attraction_radius: 120.0,
                peak_attraction_radius: 80.0,
                repulsion_radius: 20.0,
                decay_rate: 80.0,
                num_colours: 3,
                ..SimulationParams::DEFAULT
            },
        ),
        (
            "Predation",
            Model::from_3x3(
                [[0.9, -0.8, -0.9], [-0.1, 0.9, -0.4], [0.6, 0.8, -0.5]],
                NUM_COLOURS,
            ),
            SimulationParams {
                friction: 2.5,
                force_strength: 80.0,
                attraction_radius: 100.0,
                peak_attraction_radius: 20.0,
                repulsion_radius: 40.0,
                decay_rate: 80.0,
                num_colours: 3,
                ..SimulationParams::DEFAULT
            },
        ),
        (
            "Endless Chase",
            Model::from_3x3(
                [[1.0, 0.2, 0.0], [0.0, 1.0, 0.2], [0.2, 0.0, 1.0]],
                NUM_COLOURS,
            ),
            SimulationParams {
                num_colours: 3,
                ..SimulationParams::DEFAULT
            },
        ),
        (
            "The Trinity",
            Model::from_3x3(
                [[0.3, 0.4, 0.5], [0.7, -0.4, 0.3], [-0.5, 0.5, 0.0]],
                NUM_COLOURS,
            ),
            SimulationParams {
                friction: 5.0,
                force_strength: 180.0,
                attraction_radius: 200.0,
                peak_attraction_radius: 120.0,
                repulsion_radius: 110.0,
                decay_rate: 60.0,
                num_colours: 3,
                ..SimulationParams::DEFAULT
            },
        ),
        (
            "Divine Engine",
            Model::from_3x3(
                [[-0.1, 0.7, 0.0], [0.0, -0.1, 0.7], [0.7, 0.0, -0.1]],
                NUM_COLOURS,
            ),
            SimulationParams {
                friction: 5.0,
                force_strength: 120.,
                attraction_radius: 200.0,
                peak_attraction_radius: 120.0,
                repulsion_radius: 40.0,
                decay_rate: 100.0,
                num_colours: 3,
                ..SimulationParams::DEFAULT
            },
        ),
        (
            "Heath Death",
            Model::from_3x3(
                [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
                NUM_COLOURS,
            ),
            SimulationParams {
                num_colours: 3,
                ..SimulationParams::DEFAULT
            },
        ),
    ]
});

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PRESETS[0].1.clone())
            .add_observer(randomise_model)
            .add_observer(clear_particles);
    }
}

#[derive(Debug, Reflect, Resource, Clone, Deref, DerefMut, Serialize, Deserialize)]
pub struct Model {
    #[deref]
    #[serde(
        serialize_with = "model_serializer",
        deserialize_with = "model_deserializer"
    )]
    weights: Vec<f32>,
    num_colours: usize,
}

impl Model {
    pub fn from_3x3(weights: [[f32; 3]; 3], num_colours: usize) -> Self {
        Self {
            weights: weights
                .into_iter()
                .flat_map(|row| row.into_iter().chain((3..num_colours).map(|_| 0.0)))
                .chain((3..num_colours).flat_map(|_| (0..num_colours).map(|_| 0.0)))
                .collect(),
            num_colours: num_colours,
        }
    }

    pub fn weight(&self, source: ParticleColour, target: ParticleColour) -> f32 {
        debug_assert!(
            source.index() < self.num_colours && target.index() < self.num_colours,
            "Invalid particle colour index: source: {}, target: {}",
            source,
            target
        );

        self.weights[source.index() * self.num_colours + target.index()]
    }

    pub fn set_weight(&mut self, source: ParticleColour, target: ParticleColour, value: f32) {
        debug_assert!(
            source.index() < self.num_colours && target.index() < self.num_colours,
            "Invalid particle colour index: source: {}, target: {}",
            source,
            target
        );

        self.weights[source.index() * self.num_colours + target.index()] = value;
    }
}

fn model_serializer<S>(weights: &Vec<f32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(weights.len()))?;
    for weight in weights {
        seq.serialize_element(&((weight * 100.0).round() as i32))?;
    }
    seq.end()
}

fn model_deserializer<'de, D>(deserializer: D) -> Result<Vec<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let integer_weights: Vec<i32> = Deserialize::deserialize(deserializer)?;

    Ok(integer_weights.iter().map(|&i| i as f32 / 100.0).collect())
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

    params.attraction_radius = Normal::<f32>::new(75.0, 25.0)
        .unwrap()
        .sample(&mut rng)
        .clamp(20.0, *ATTRACTION_RADIUS_RANGE.end());

    params.peak_attraction_radius = Normal::<f32>::new(
        params.attraction_radius * 0.6,
        params.attraction_radius * 0.5,
    )
    .unwrap()
    .sample(&mut rng)
    .clamp(0.0, *PEAK_ATTRACTION_RADIUS_RANGE.end());

    params.repulsion_radius = Normal::<f32>::new(
        params.attraction_radius * 0.4,
        params.attraction_radius * 0.3,
    )
    .unwrap()
    .sample(&mut rng)
    .clamp(
        20.0_f32.min(params.attraction_radius),
        REPULSION_RADIUS_RANGE.end().min(params.attraction_radius),
    );

    model.weights.iter_mut().for_each(|value| {
        *value = rand::random::<f32>() * 2.0 - 1.0;
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
