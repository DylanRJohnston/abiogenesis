use bevy::prelude::*;

use bevy_tweening::component_animator_system;

use crate::{
    camera::{drag_screen, select_follow_particle, zoom},
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
            prevent_event_propagation(),
            children![
                model_matrix(),
                (
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    children![
                        (
                            control_button("RANDOMISE", Randomise),
                            BorderRadius::top(Val::Px(8.0))
                        ),
                        control_button("RESPAWN", Respawn),
                        (
                            control_button("CLEAR", ClearParticles),
                            BorderRadius::bottom(Val::Px(8.0)),
                        ),
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
        Observe::event(drag_screen),
        Observe::event(zoom),
        Observe::event(select_follow_particle),
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
