use bevy::{prelude::*, window::WindowResized};

use crate::{
    controls,
    observe::observe,
    particles::{
        model::{ClearParticles, Randomise},
        spawner::Respawn,
    },
    systems::AppSystems,
    ui::{
        button::control_button,
        examples::examples,
        model_matrix::{model_matrix, update_model_matrix},
        parameters::parameters,
    },
};

mod button;
mod dropdown;
mod examples;
mod icon;
mod mixins;
mod model_matrix;
mod parameters;
mod slider;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (detect_layout, respawn_ui).chain())
            .add_systems(Update, update_model_matrix.in_set(AppSystems::Update))
            .add_systems(Update, on_window_resized);
    }
}

#[derive(Debug, Event, Resource, Reflect, PartialEq, Eq, Copy, Clone)]
enum Layout {
    Horizontal,
    Vertical,
}

impl Layout {
    fn flex_direction(&self) -> FlexDirection {
        match self {
            Layout::Horizontal => FlexDirection::Row,
            Layout::Vertical => FlexDirection::Column,
        }
    }
}

fn decide_layout(width: f32, height: f32) -> Layout {
    if width > height {
        Layout::Vertical
    } else {
        Layout::Horizontal
    }
}

fn detect_layout(window: Single<&Window>, mut commands: Commands) {
    commands.insert_resource(decide_layout(window.width(), window.height()));
}

fn on_window_resized(
    window: Single<&Window>,
    mut resize_reader: EventReader<WindowResized>,
    mut layout: ResMut<Layout>,
    mut commands: Commands,
) {
    if resize_reader.read().count() == 0 {
        return;
    }

    let new_layout = decide_layout(window.width(), window.height());
    if new_layout == *layout {
        return;
    }

    *layout = new_layout;
    commands.run_system_cached(respawn_ui);
}

#[derive(Debug, Component, Reflect)]
struct UIRoot;

#[cfg_attr(
    feature = "hot_reload",
    bevy_simple_subsecond_system::hot(rerun_on_hot_patch = true)
)]
fn respawn_ui(
    mut commands: Commands,
    icons: Res<AssetServer>,
    layout: Res<Layout>,
    roots: Query<Entity, With<UIRoot>>,
) {
    roots
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());

    commands.spawn((
        full_screen_container(),
        children![(
            sidebar(*layout),
            prevent_event_propagation(),
            children![
                model_matrix(),
                (
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::End,
                        row_gap: Val::Px(8.0),
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                    children![
                        (
                            Node {
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::SpaceBetween,
                                column_gap: Val::Px(8.0),
                                ..default()
                            },
                            children![
                                control_button(
                                    "Clear all cells",
                                    ClearParticles,
                                    icons.load("icons/clear.png")
                                ),
                                control_button(
                                    "Reset cell positions",
                                    Respawn,
                                    icons.load("icons/reload.png")
                                ),
                                control_button(
                                    "Randomise inter-cell forces",
                                    Randomise,
                                    icons.load("icons/shuffle.png")
                                ),
                            ],
                        ),
                        parameters(),
                        examples()
                    ]
                )
            ]
        )],
    ));
}

fn prevent_event_propagation() -> impl Bundle {
    observe(|mut trigger: Trigger<Pointer<Click>>| trigger.propagate(false))
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
        observe(controls::drag_screen),
        observe(controls::scroll_wheel_zoom),
        observe(controls::select_follow_particle),
    )
}

fn sidebar(direction: Layout) -> impl Bundle {
    Node {
        padding: UiRect::all(Val::Px(8.0)),
        display: Display::Flex,
        flex_direction: direction.flex_direction(),
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Start,
        row_gap: Val::Px(16.0),
        column_gap: Val::Px(16.0),
        ..default()
    }
}
