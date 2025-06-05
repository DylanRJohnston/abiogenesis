use crate::particles::{simulation::Particle, size::SimulationSize};
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, clamp_camera_zoom)
            .add_systems(
                Update,
                camera_follow_particle.run_if(resource_exists::<FollowParticle>),
            );
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

    commands.spawn((Name::from("Camera"), Camera2d));
}

fn clamp_camera_zoom(mut projection: Single<&mut Projection>, simulation_size: SimulationSize) {
    let Projection::Orthographic(ref mut project) = **projection else {
        return;
    };

    let (min_zoom, max_zoom) = simulation_size.scale_bounds();
    project.scale = project.scale.clamp(min_zoom, max_zoom);
}

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct FollowParticle(pub Entity);

fn camera_follow_particle(
    follow_particle: Res<FollowParticle>,
    mut particles: Query<&mut Transform, With<Particle>>,
    mut commands: Commands,
) {
    let Ok(&particle) = particles.get(**follow_particle) else {
        commands.remove_resource::<FollowParticle>();
        return;
    };

    particles.iter_mut().for_each(|mut transform| {
        transform.translation -= particle.translation;
    });
}
