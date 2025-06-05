use bevy::prelude::*;

use bevy_tweening::component_animator_system;

use crate::{
    camera, controls,
    observe::Observe,
    particles::{
        model::{ClearParticles, Randomise},
        spawner::Respawn,
    },
    ui::{
        button::control_button,
        model_matrix::{model_matrix, update_model_matrix},
    },
};

mod button;
mod model_matrix;

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

#[cfg_attr(
    feature = "hot_reload",
    bevy_simple_subsecond_system::hot(rerun_on_hot_patch = true)
)]
fn spawn_ui(
    mut commands: Commands,
    icons: Res<AssetServer>,
    window: Single<&Window>,
    #[cfg(feature = "hot_reload")] roots: Query<Entity, With<UIRoot>>,
) {
    #[cfg(feature = "hot_reload")]
    roots
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());

    let direction = if window.width() > window.height() {
        FlexDirection::Column
    } else {
        FlexDirection::Row
    };

    commands.spawn((
        full_screen_container(),
        children![(
            sidebar(direction),
            prevent_event_propagation(),
            children![
                model_matrix(),
                (
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        row_gap: Val::Px(8.0),
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                    children![
                        control_button("RANDOMISE", Randomise, icons.load("icons/shuffle.png")),
                        control_button("RESPAWN", Respawn, icons.load("icons/reload.png")),
                        control_button("CLEAR", ClearParticles, icons.load("icons/clear.png")),
                    ]
                )
            ]
        )],
    ));
}

fn prevent_event_propagation() -> impl Bundle {
    Observe::event(|mut trigger: Trigger<Pointer<Click>>| trigger.propagate(false))
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
        Observe::event(controls::drag_screen),
        Observe::event(controls::scroll_wheel_zoom),
        Observe::event(controls::select_follow_particle),
    )
}

fn sidebar(direction: FlexDirection) -> impl Bundle {
    Node {
        padding: UiRect::all(Val::Px(8.0)),
        display: Display::Flex,
        flex_direction: direction,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::End,
        row_gap: Val::Px(16.0),
        column_gap: Val::Px(16.0),
        ..default()
    }
}
