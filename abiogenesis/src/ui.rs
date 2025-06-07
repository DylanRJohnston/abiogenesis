use std::time::Duration;

use bevy::{prelude::*, window::WindowResized};
use bevy_tweening::{Animator, Tween};

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
        lenses::LeftLens,
        menu_button::{hide_ui, show_ui_button},
        model_matrix::{model_matrix, update_model_matrix},
        parameters::parameters,
        toolbar::{ToolBarPlugin, toolbar},
    },
};

mod button;
mod colours;
mod dropdown;
mod examples;
mod icon;
mod lenses;
mod menu_button;
mod mixins;
mod model_matrix;
mod parameters;
mod slider;
pub mod toolbar;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ToolBarPlugin)
            .add_systems(Startup, (detect_layout, respawn_ui).chain())
            .add_systems(Update, update_model_matrix.in_set(AppSystems::Update))
            .add_systems(Update, on_window_resized);
    }
}

#[derive(Debug, Event, Resource, Reflect, PartialEq, Eq, Copy, Clone)]
pub enum Layout {
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
    // if width > 630.0 {
    //     if height < 590.0 {
    //         Layout::Horizontal
    //     } else {
    //         Layout::Vertical
    //     }
    // } else {
    //     if height < 710.0 {
    //         Layout::Horizontal
    //     } else {
    //         Layout::Vertical
    //     }
    // }
    Layout::Vertical
}

fn detect_layout(window: Single<&Window>, mut commands: Commands) {
    commands.insert_resource(decide_layout(window.width(), window.height()));
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn on_window_resized(
    window: Single<&Window>,
    mut resize_reader: EventReader<WindowResized>,
    mut layout: ResMut<Layout>,
    mut commands: Commands,
) {
    if resize_reader.read().count() == 0 {
        return;
    }

    tracing::info!(width = ?window.width(), height = ?window.height());

    let new_layout = decide_layout(window.width(), window.height());
    if new_layout == *layout {
        return;
    }

    *layout = new_layout;
    commands.run_system_cached(respawn_ui);
}

#[derive(Debug, Component, Reflect)]
pub struct UIRoot;

#[cfg_attr(
    feature = "hot_reload",
    bevy_simple_subsecond_system::hot(rerun_on_hot_patch = true)
)]
pub fn respawn_ui(
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
        children![
            show_ui_button(),
            (
                sidebar(*layout),
                mixins::block_all_interactions(),
                Animator::new(Tween::new(
                    EaseFunction::SmootherStepOut,
                    Duration::from_secs_f32(1.),
                    LeftLens {
                        start: if *layout == Layout::Horizontal {
                            -500.0
                        } else {
                            -250.0
                        },
                        end: 0.0,
                    }
                )),
                children![(
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::End,
                        row_gap: Val::Px(8.0),
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                    children![
                        examples(),
                        parameters(),
                        (
                            Node {
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::SpaceBetween,
                                column_gap: Val::Px(8.0),
                                ..default()
                            },
                            children![
                                hide_ui(),
                                control_button(
                                    "Annihilate",
                                    ClearParticles,
                                    icons.load("icons/nuclear-explosion.png")
                                ),
                                control_button("Revive", Respawn, icons.load("icons/plant.png")),
                                control_button("Reshape", Randomise, icons.load("icons/dice.png")),
                            ],
                        ),
                    ]
                )]
            ),
            toolbar()
        ],
    ));
}

fn full_screen_container() -> impl Bundle {
    (
        UIRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            ..default()
        },
        observe(controls::drag_screen),
        observe(controls::scroll_wheel_zoom),
        observe(controls::select_follow_particle),
        observe(controls::particle_brush_start),
        observe(controls::particle_brush_drag),
        observe(controls::eraser_brush_start),
        observe(controls::eraser_brush_drag),
    )
}

#[derive(Debug, Component, Deref, Clone, Copy)]
pub struct Sidebar(Layout);

fn sidebar(direction: Layout) -> impl Bundle {
    (
        Sidebar(direction),
        Node {
            padding: UiRect::all(Val::Px(8.0)),
            display: Display::Flex,
            flex_direction: direction.flex_direction(),
            row_gap: Val::Px(4.0),
            column_gap: Val::Px(16.0),
            ..default()
        },
    )
}
