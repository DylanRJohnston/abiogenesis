use bevy::prelude::*;

use crate::{
    particles::{colour::ParticleColour, size::SimulationSize},
    spatial_hash::SpatialHashGrid,
};

pub struct SpatialIndexPlugin;
impl Plugin for SpatialIndexPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialise_spatial_index);
    }
}

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct SpatialIndex(SpatialHashGrid<(Entity, ParticleColour)>);

fn initialise_spatial_index(mut commands: Commands, simulation: SimulationSize) {
    commands.insert_resource(SpatialIndex(SpatialHashGrid::new(
        Rect::from_center_size(Vec2::ZERO, simulation.dimensions()),
        (10, 10),
    )));
}
