use bevy::prelude::*;

#[cfg(feature = "hot_reload")]
use bevy_simple_subsecond_system::prelude::*;
use bevy_tweening::component_animator_system;

use crate::{
    observe::Observe,
    particles::Particle,
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
) {
    let mut delta = trigger.delta;
    delta.y *= -1.0;

    for mut particle in &mut particles {
        particle.translation += delta.extend(0.0);
    }
}
