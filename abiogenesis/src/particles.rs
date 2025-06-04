use bevy::prelude::*;

use crate::particles::{
    decay::DecayPlugin, model::*, simulation::SimulationPlugin, size::SimulationSizePlugin,
    spatial_index::SpatialIndexPlugin, spawner::SpawnerPlugin,
};

pub mod colour;
pub mod decay;
pub mod model;
pub mod simulation;
pub mod size;
pub mod spatial_index;
pub mod spawner;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ModelPlugin,
            DecayPlugin,
            SimulationPlugin,
            SimulationSizePlugin,
            SpatialIndexPlugin,
            SpawnerPlugin,
        ));
    }
}
