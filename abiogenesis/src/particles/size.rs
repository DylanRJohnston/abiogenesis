use bevy::{ecs::system::SystemParam, prelude::*};

use crate::particles::spatial_index::SpatialIndex;

pub struct SimulationSizePlugin;
impl Plugin for SimulationSizePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, resize_simulation);
    }
}

#[derive(SystemParam)]
pub struct SimulationSize<'w> {
    window: Single<'w, &'static Window>,
    // projection: Single<'w, &'static Projection>,
}

impl SimulationSize<'_> {
    pub fn dimensions(&self) -> Vec2 {
        let (width, height) = (self.window.width(), self.window.height());
        // let Projection::Orthographic(OrthographicProjection { scale, ..}) = *self.projection else {
        //     panic!("Projection is not orthographic");
        // };

        Vec2::new(width, height)
    }
}

fn resize_simulation(mut spatial_index: ResMut<SpatialIndex>, simulation_size: SimulationSize) {
    spatial_index.update_bounds(Rect::from_center_size(
        Vec2::ZERO,
        simulation_size.dimensions(),
    ));
}
