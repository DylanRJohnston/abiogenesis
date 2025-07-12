use crate::math::TorodialMath;
use std::{cmp::Ordering, time::Duration};

use bevy::prelude::*;

use crate::{
    camera::{FollowParticle, Viewport},
    particles::{
        particle::Particle, size::SimulationSize, spatial_index::SpatialIndex,
        spawner::SpawnParticle,
    },
    systems::AppSystems,
    ui::toolbar::Tool,
};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, touch_pan.in_set(AppSystems::RecordInput))
            .add_systems(
                Update,
                touch_registration_timeout
                    .run_if(resource_exists::<TouchRegistrationTimeout>)
                    .in_set(AppSystems::RecordInput)
                    .after(touch_pan),
            );
    }
}

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct TouchRegistrationTimeout(Timer);

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn touch_pan(
    touches: Res<Touches>,
    simulation_size: SimulationSize,
    touch_registration_timeout: Option<ResMut<TouchRegistrationTimeout>>,
    mut particles: Query<&mut Transform, With<Particle>>,
    mut camera: Single<&mut Projection, With<Camera>>,
    mut commands: Commands,
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

    match touch_registration_timeout {
        None => {
            commands.insert_resource(TouchRegistrationTimeout(Timer::new(
                Duration::from_secs_f32(0.250),
                TimerMode::Once,
            )));
        }
        Some(mut touch_registration_timeout) => {
            touch_registration_timeout.reset();
        }
    }

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

fn touch_registration_timeout(
    mut touch_registration_timeout: ResMut<TouchRegistrationTimeout>,
    time: Res<Time>,
    mut commands: Commands,
) {
    if touch_registration_timeout
        .tick(time.delta())
        .just_finished()
    {
        commands.remove_resource::<TouchRegistrationTimeout>();
    }
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn select_follow_particle(
    trigger: Trigger<Pointer<Click>>,
    spatial_index: Res<SpatialIndex>,
    viewport: Viewport,
    touch_registration_timeout: Option<ResMut<TouchRegistrationTimeout>>,
    tool: Res<Tool>,
    mut commands: Commands,
) {
    if *tool != Tool::Camera {
        return;
    }

    if !matches!(trigger.button, PointerButton::Primary) {
        return;
    }

    if touch_registration_timeout.is_some() {
        tracing::info!("Skipping follow due to touch registration timeout");
        return;
    }

    let pointer_location = viewport.to_world(trigger.pointer_location.position);

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

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn drag_screen(
    trigger: Trigger<Pointer<Drag>>,
    mut camera_transform: Single<&mut Transform, With<Camera>>,
    projection: Single<&Projection>,
    bounds: SimulationSize,
    mut commands: Commands,
) {
    if !matches!(trigger.button, PointerButton::Secondary) {
        return;
    }

    commands.remove_resource::<FollowParticle>();

    let Projection::Orthographic(ref project) = **projection else {
        return;
    };

    let mut delta = trigger.delta;
    delta.y *= -1.0;
    delta *= project.scale;

    // Move the camera instead of all particles
    camera_transform.translation = bounds
        .as_rect()
        .toroidal_wrap(camera_transform.translation.truncate() - delta)
        .extend(0.0);
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn scroll_wheel_zoom(
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

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn particle_brush_start(
    trigger: Trigger<Pointer<Pressed>>,
    tool: Res<Tool>,
    viewport: Viewport,
    mut spawn_particles: EventWriter<SpawnParticle>,
) {
    let Tool::Particle(colour) = *tool else {
        return;
    };

    if !matches!(trigger.button, PointerButton::Primary) {
        return;
    }

    let position = viewport.to_world(trigger.pointer_location.position);

    spawn_particles.write(SpawnParticle { position, colour });
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn particle_brush_drag(
    trigger: Trigger<Pointer<Drag>>,
    tool: Res<Tool>,
    viewport: Viewport,
    mut spawn_particles: EventWriter<SpawnParticle>,
) {
    let Tool::Particle(colour) = *tool else {
        return;
    };

    if !matches!(trigger.button, PointerButton::Primary) {
        return;
    }

    let position = viewport.to_world(trigger.pointer_location.position);

    spawn_particles.write(SpawnParticle { position, colour });
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn eraser_brush_start(
    trigger: Trigger<Pointer<Pressed>>,
    tool: Res<Tool>,
    viewport: Viewport,
    spatial_index: Res<SpatialIndex>,
    mut commands: Commands,
) {
    let Tool::Smite = *tool else {
        return;
    };

    if !matches!(trigger.button, PointerButton::Primary) {
        return;
    }

    let position = viewport.to_world(trigger.pointer_location.position);

    for (_, &(entity, _)) in spatial_index.query(position, 30.0) {
        commands.entity(entity).try_despawn();
    }
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn eraser_brush_drag(
    trigger: Trigger<Pointer<Drag>>,
    tool: Res<Tool>,
    viewport: Viewport,
    spatial_index: Res<SpatialIndex>,
    mut commands: Commands,
) {
    let Tool::Smite = *tool else {
        return;
    };

    if !matches!(trigger.button, PointerButton::Primary) {
        return;
    }

    let position = viewport.to_world(trigger.pointer_location.position);

    for (_, &(entity, _)) in spatial_index.query(position, 30.0) {
        commands.entity(entity).try_despawn();
    }
}
