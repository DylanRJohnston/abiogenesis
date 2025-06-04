use crate::particles::simulation::Particle;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

#[cfg_attr(
    feature = "hot_reload",
    bevy_simple_subsecond_system::hot(rerun_on_hot_patch = true)
)]
fn spawn_camera(
    mut commands: Commands,
    #[cfg(feature = "hot_reload")] cameras: Query<Entity, With<Camera>>,
) {
    #[cfg(feature = "hot_reload")]
    cameras
        .iter()
        .for_each(|camera| commands.entity(camera).despawn());

    commands.spawn((
        Name::from("Camera"),
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 1.0,
            ..OrthographicProjection::default_2d()
        }),
        Camera { ..default() },
        // Tonemapping::AcesFitted,
        // Bloom::default(),
        // DebandDither::Enabled,
    ));
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn drag_screen(
    trigger: Trigger<Pointer<Drag>>,
    mut particles: Query<&mut Transform, With<Particle>>,
    projection: Single<&Projection>,
) {
    let Projection::Orthographic(ref project) = **projection else {
        return;
    };

    let mut delta = trigger.delta;
    delta.y *= -1.0;
    delta *= project.scale;

    for mut particle in &mut particles {
        particle.translation += delta.extend(0.0);
    }
}

const MAX_ZOOM: f32 = 1.0;
const MIN_ZOOM: f32 = 0.1;

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn zoom(trigger: Trigger<Pointer<Scroll>>, mut projection: Single<&mut Projection>) {
    let Projection::Orthographic(ref mut project) = **projection else {
        return;
    };

    project.scale = (project.scale - trigger.y * 0.01).clamp(MIN_ZOOM, MAX_ZOOM);
}
