use std::time::Duration;

use bevy::{prelude::*, window::WindowResized};
use bevy_tweening::{Animator, Tween};

use crate::{
    browser_state::{Export, Import},
    controls,
    observe::observe,
    particles::{
        model::{ClearParticles, Randomise},
        simulation::SimulationParams,
        spawner::Respawn,
    },
    systems::AppSystems,
    ui::{
        button::control_button,
        examples::examples,
        lenses::{LeftLens, LensPlugin},
        menu_button::{hide_ui, show_ui_button},
        model_matrix::{update_matrix_size, update_model_matrix},
        parameters::parameters,
        title_screen::TitleScreenPlugin,
        toolbar::ToolBarPlugin,
    },
};

mod button;
mod challenges;
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
mod title_screen;
pub mod toolbar;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ToolBarPlugin)
            .add_plugins(LensPlugin)
            .add_plugins(TitleScreenPlugin)
            .add_systems(Update, update_model_matrix.in_set(AppSystems::Update))
            .add_systems(Update, update_matrix_size.in_set(AppSystems::Update))
            .add_systems(PreUpdate, calculate_ui_scale);
    }
}

#[cfg_attr(
    feature = "hot_reload",
    bevy_simple_subsecond_system::hot(rerun_on_hot_patch = true)
)]
fn calculate_ui_scale(
    mut resize_reader: EventReader<WindowResized>,
    mut ui_scale: ResMut<UiScale>,
) {
    if let Some(e) = resize_reader.read().last() {
        ui_scale.0 = e.height / 800.;
    }
}

#[derive(Debug, Component, Reflect)]
pub struct UIRoot;

#[cfg_attr(
    feature = "hot_reload",
    bevy_simple_subsecond_system::hot(rerun_on_hot_patch = true)
)]
pub fn respawn_ui(
    mut commands: Commands,
    params: Res<SimulationParams>,
    icons: Res<AssetServer>,
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
                sidebar(),
                mixins::block_all_interactions(),
                Animator::new(Tween::new(
                    EaseFunction::SmootherStepOut,
                    Duration::from_secs_f32(1.5),
                    LeftLens {
                        start: -250.0,
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
                        parameters(params.num_colours),
                        (
                            Node {
                                width: Val::Percent(100.0),
                                display: Display::Grid,
                                grid_template_columns: vec![RepeatedGridTrack::flex(3, 1.0)],
                                column_gap: Val::Px(8.0),
                                row_gap: Val::Px(8.0),
                                ..default()
                            },
                            children![
                                hide_ui(),
                                control_button("Receive", Import, icons.load("icons/import.png")),
                                control_button("Bestow", Export, icons.load("icons/export.png")),
                                control_button(
                                    "Annihilate",
                                    ClearParticles,
                                    icons.load("icons/nuclear-explosion.png")
                                ),
                                control_button(
                                    "Regenerate",
                                    Respawn,
                                    icons.load("icons/plant.png")
                                ),
                                control_button("Reshape", Randomise, icons.load("icons/dice.png")),
                            ],
                        ),
                    ]
                )]
            ),
            toolbar::toolbar()
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
        observe(toolbar::smite_start_hover),
        observe(toolbar::smite_hover),
        observe(toolbar::smite_end_hover),
        observe(toolbar::smite_end_touch),
    )
}

#[derive(Debug, Component, Clone, Copy)]
pub struct Sidebar;

fn sidebar() -> impl Bundle {
    (
        Sidebar,
        Node {
            padding: UiRect::all(Val::Px(8.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            column_gap: Val::Px(16.0),
            left: Val::Px(-250.0),
            ..default()
        },
    )
}
