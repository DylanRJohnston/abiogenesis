use crate::{
    particles::{particle::Particle, size::SimulationSize},
    systems::AppSystems,
};
use bevy::{ecs::system::SystemParam, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, clamp_camera_zoom.in_set(AppSystems::Camera))
            .add_systems(
                Update,
                camera_follow_particle
                    .run_if(resource_exists::<FollowParticle>)
                    .in_set(AppSystems::Camera),
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

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn camera_follow_particle(
    follow_particle: Res<FollowParticle>,
    mut particles: Query<&mut Transform, With<Particle>>,
    projection: Single<&Projection>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let Ok(&particle) = particles.get(**follow_particle) else {
        commands.remove_resource::<FollowParticle>();
        return;
    };

    let Projection::Orthographic(ref projection) = **projection else {
        return;
    };

    // Dividing by projection scale makes the camera move at the same speed regardless of zoom level.
    let translation = (3.0 / projection.scale) * time.delta_secs() * particle.translation;

    particles.iter_mut().for_each(|mut transform| {
        transform.translation -= translation;
    });
}

#[derive(SystemParam)]
pub struct Viewport<'w> {
    camera: Single<'w, (&'static Camera, &'static GlobalTransform)>,
}

impl Viewport<'_> {
    pub fn to_world(&self, position: Vec2) -> Vec2 {
        let (camera, transform) = *self.camera;

        camera.viewport_to_world_2d(transform, position).unwrap()
    }
}
