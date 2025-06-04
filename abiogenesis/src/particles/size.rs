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

        Vec2::new(width.max(1920.0), height.max(1080.0))
    }

    pub fn scale_bounds(&self) -> (f32, f32) {
        scale_bounds(self.window.width(), self.window.height())
    }
}

fn resize_simulation(mut spatial_index: ResMut<SpatialIndex>, simulation_size: SimulationSize) {
    spatial_index.update_bounds(Rect::from_center_size(
        Vec2::ZERO,
        simulation_size.dimensions(),
    ));
}

// Allows zooming out until you hit the screen edges
fn scale_bounds(screen_width: f32, screen_height: f32) -> (f32, f32) {
    let (simulation_width, simulation_height) =
        (screen_width.max(1920.0), screen_height.max(1080.0));

    (
        0.1,
        (simulation_width / screen_width).min(simulation_height / screen_height),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_normal() {
        assert_eq!(scale_bounds(1920.0, 1080.0), (0.1, 1.0));
    }

    #[test]
    fn test_big_screen() {
        assert_eq!(scale_bounds(3840.0, 2160.0), (0.1, 1.0));
    }

    #[test]
    fn test_narrow_screen() {
        assert_eq!(scale_bounds(1920.0, 540.0), (0.1, 1.0));
    }

    #[test]
    fn test_wide_screen() {
        assert_eq!(scale_bounds(960.0, 1080.0), (0.1, 1.0));
    }

    #[test]
    fn test_small_landscape() {
        assert_eq!(scale_bounds(1280.0, 720.0), (0.1, 1.5));
    }

    #[test]
    fn test_small_portrait() {
        assert_eq!(scale_bounds(720.0, 960.0), (0.1, 1.125));
    }
}
