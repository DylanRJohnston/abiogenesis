use std::cmp::Ordering;

use crate::particles::{
    simulation::Particle,
    size::SimulationSize,
    spatial_index::{self, SpatialIndex},
};
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, touch_pan)
            .add_systems(Update, rescale_zoom)
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
fn touch_pan(
    touches: Res<Touches>,
    mut particles: Query<&mut Transform, With<Particle>>,
    mut camera: Single<&mut Projection, With<Camera>>,
    simulation_size: SimulationSize,
) {
    if !touches.is_changed() {
        return;
    }

    let Projection::Orthographic(ref mut project) = **camera else {
        return;
    };

    let touches = touches.iter().collect::<Vec<_>>();
    let [first, second] = touches.as_slice() else {
        return;
    };

    // Compute the translation zoom and rotation from the delta of the two touches
    let prev_center = (first.previous_position() + second.previous_position()) / 2.0;
    let curr_center = (first.position() + second.position()) / 2.0;
    let mut translation = curr_center - prev_center;
    translation.y *= -1.0;

    let prev_diff = second.previous_position() - first.previous_position();
    let curr_diff = second.position() - first.position();

    let scale = prev_diff.length() / curr_diff.length();

    let prev_angle = prev_diff.y.atan2(prev_diff.x);
    let curr_angle = curr_diff.y.atan2(curr_diff.x);
    let rotation = curr_angle - prev_angle;

    let transform = Transform {
        translation: translation.extend(0.0) * project.scale,
        rotation: Quat::from_rotation_z(-rotation),
        scale: Vec3::splat(1.0),
    };

    let (min_zoom, max_zoom) = simulation_size.scale_bounds();

    project.scale = (project.scale * scale).clamp(min_zoom, max_zoom);

    particles.iter_mut().for_each(|mut particle| {
        particle.translation = transform.transform_point(particle.translation);
    });
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn drag_screen(
    trigger: Trigger<Pointer<Drag>>,
    mut particles: Query<&mut Transform, With<Particle>>,
    projection: Single<&Projection>,
) {
    if !matches!(trigger.button, PointerButton::Secondary) {
        return;
    }

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

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn zoom(
    trigger: Trigger<Pointer<Scroll>>,
    mut projection: Single<&mut Projection>,
    simulation_size: SimulationSize,
) {
    let Projection::Orthographic(ref mut project) = **projection else {
        return;
    };

    let (min_zoom, max_zoom) = simulation_size.scale_bounds();
    project.scale = (project.scale - trigger.y.clamp(-0.05, 0.05)).clamp(min_zoom, max_zoom);
}

fn rescale_zoom(mut projection: Single<&mut Projection>, simulation_size: SimulationSize) {
    let Projection::Orthographic(ref mut project) = **projection else {
        return;
    };

    let (min_zoom, max_zoom) = simulation_size.scale_bounds();
    project.scale = project.scale.clamp(min_zoom, max_zoom);
}

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct FollowParticle(Entity);

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn select_follow_particle(
    trigger: Trigger<Pointer<Click>>,
    spatial_index: Res<SpatialIndex>,
    camera: Single<(&Camera, &GlobalTransform)>,
    follow_particle: Option<Res<FollowParticle>>,
    mut commands: Commands,
) {
    if !matches!(trigger.button, PointerButton::Primary) {
        return;
    }

    if follow_particle.is_some() {
        commands.remove_resource::<FollowParticle>();
        return;
    }

    let (camera, camera_transform) = *camera;

    let Ok(pointer_location) =
        camera.viewport_to_world_2d(camera_transform, trigger.pointer_location.position)
    else {
        return;
    };

    let Some((_, (entity, _))) =
        spatial_index
            .query(pointer_location, 25.0)
            .min_by(|(a, _), (b, _)| {
                a.distance_squared(pointer_location)
                    .partial_cmp(&b.distance_squared(pointer_location))
                    .unwrap_or(Ordering::Equal)
            })
    else {
        return;
    };

    commands.insert_resource(FollowParticle(*entity));
}

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
