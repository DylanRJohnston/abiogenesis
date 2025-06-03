use bevy::prelude::*;

#[cfg(feature = "hot_reload")]
use bevy_simple_subsecond_system::prelude::*;
use bevy_tweening::component_animator_system;

use crate::{
    observe::Observe,
    particles::{Particle, SimulationSize, SpatialIndex, Velocity},
    ui::{
        model_matrix::{model_matrix, update_model_matrix},
        randomise_button::randomise_button,
        reset_button::reset_button,
    },
};

mod button;
mod model_matrix;
mod randomise_button;
mod reset_button;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui)
            .add_systems(Update, update_model_matrix)
            .add_systems(Update, component_animator_system::<Node>);
    }
}

#[derive(Debug, Component, Reflect)]
struct UIRoot;

#[cfg_attr(feature = "hot_reload", hot(rerun_on_hot_patch = true))]
fn spawn_ui(
    mut commands: Commands,
    #[cfg(feature = "hot_reload")] roots: Query<Entity, With<UIRoot>>,
) {
    #[cfg(feature = "hot_reload")]
    roots
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());

    commands.spawn((
        full_screen_container(),
        children![(
            sidebar(),
            children![
                model_matrix(),
                (
                    Node {
                        width: Val::Percent(100.0),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    children![reset_button(), randomise_button()]
                )
            ]
        )],
    ));
}

fn full_screen_container() -> impl Bundle {
    (
        UIRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            ..default()
        },
        Observe::event(drag_screen),
        Observe::event(zoom),
        // Observe::event(explode),
    )
}

fn sidebar() -> impl Bundle {
    Node {
        padding: UiRect::all(Val::Px(16.0)),
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Start,
        row_gap: Val::Px(8.0),
        ..default()
    }
}

#[cfg_attr(feature = "hot_reload", hot)]
fn drag_screen(
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

#[cfg_attr(feature = "hot_reload", hot)]
fn zoom(trigger: Trigger<Pointer<Scroll>>, mut projection: Single<&mut Projection>) {
    let Projection::Orthographic(ref mut project) = **projection else {
        return;
    };

    project.scale = (project.scale - trigger.y * 0.01).clamp(MIN_ZOOM, MAX_ZOOM);
}

#[cfg_attr(feature = "hot_reload", hot)]
fn explode(
    trigger: Trigger<Pointer<Click>>,
    mut particles: Query<(&mut Transform, &mut Velocity), With<Particle>>,
    simulation_size: SimulationSize,
    index: Res<SpatialIndex>,
) {
    let Vec2 {
        x: width,
        y: height,
    } = simulation_size.dimensions();

    // Pointer coordinates start in the top right corner, with positive x to the left and positive y downard.
    let pointer_location = Vec2::new(
        trigger.pointer_location.position.x - width / 2.0,
        height - trigger.pointer_location.position.y - height / 2.0,
    );

    for (pos, (entity, _)) in index.query(pointer_location, 150.0) {
        let Ok((mut transform, mut velocity)) = particles.get_mut(*entity) else {
            continue;
        };

        let direction = (pos - pointer_location).normalize();

        **velocity = direction * 4000.0;
        transform.translation += (direction * 100.0).extend(0.0);
    }
}
