use bevy::{
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
    prelude::*,
};

mod circle;
mod model_box;
mod num_colours;

use circle::*;
use model_box::*;
use num_colours::*;

use crate::{
    math::remap,
    particles::{
        colour::{ParticleColour::*, *},
        model::Model,
        simulation::SimulationParams,
    },
    ui::parameters::Parameters,
};

const MID_COLOR: Color = Color::Srgba(Srgba::rgb(233.0 / 255.0, 133.0 / 255.0, 55.0 / 255.0));

#[cfg_attr(
    feature = "hot_reload",
    bevy_simple_subsecond_system::hot(rerun_on_hot_patch = true)
)]
pub fn update_model_matrix(
    mut elements: Query<(&ModelIndex, &mut BackgroundColor, &Children)>,
    mut text: Query<(&mut Text, &mut TextFont)>,
    params: Res<SimulationParams>,
    model: Res<Model>,
) {
    for (value, mut colour, children) in elements.iter_mut() {
        let value = model.weight(value.source, value.target);

        let (mut text, mut font) = text.get_mut(children[0]).unwrap();
        **text = format!("{value:.0}", value = value * 10.0);
        font.font_size = remap(
            params.num_colours as f32,
            1.0,
            NUM_COLOURS as f32,
            32.0,
            16.0,
        );

        if value <= -0.0 {
            *colour = MID_COLOR.mix(&RED, (-value).powf(0.5)).into();
        } else {
            *colour = MID_COLOR.mix(&GREEN, value.powf(0.5)).into();
        }
    }
}

macro_rules! many_children {
    ($($x:expr),* $(,)?) => {
        Children::spawn(SpawnWith(|spawner: &mut ChildSpawner| {
            $(
                spawner.spawn($x);
            )*
        }))
    };
}

pub const MODEL_MATRIX_SIZE: f32 = 200.0;

#[derive(Component)]
pub struct ModelMatrix;

pub fn model_matrix(num_colours: usize) -> impl Bundle {
    (
        ModelMatrix,
        Name::from("Model Matrix"),
        Node {
            display: Display::Grid,
            width: Val::Px(MODEL_MATRIX_SIZE),
            height: Val::Px(MODEL_MATRIX_SIZE),
            grid_template_columns: vec![RepeatedGridTrack::flex(num_colours as u16 + 1, 1.0)],
            grid_template_rows: vec![RepeatedGridTrack::flex(num_colours as u16 + 1, 1.0)],
            justify_content: JustifyContent::Stretch,
            align_content: AlignContent::Start,
            justify_items: JustifyItems::Stretch,
            align_items: AlignItems::Stretch,
            row_gap: Val::Px(4.0),
            column_gap: Val::Px(4.0),
            ..default()
        },
        // children![] has a maximum limit of children
        Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
            spawner.spawn(change_num_colours());

            (0..num_colours).for_each(|index| {
                spawner.spawn(circle(ParticleColour::from_index(index)));
            });

            (0..num_colours).for_each(|row| {
                spawner.spawn(circle(ParticleColour::from_index(row)));

                (0..num_colours).for_each(|col| {
                    let box_bundle = model_box(
                        ParticleColour::from_index(row),
                        ParticleColour::from_index(col),
                    );

                    if num_colours == 1 {
                        spawner.spawn((box_bundle, BorderRadius::all(Val::Px(8.0))));
                    } else if row == 0 && col == 0 {
                        spawner.spawn((box_bundle, BorderRadius::top_left(Val::Px(8.0))));
                    } else if row == 0 && col == (num_colours - 1) {
                        spawner.spawn((box_bundle, BorderRadius::top_right(Val::Px(8.0))));
                    } else if row == (num_colours - 1) && col == 0 {
                        spawner.spawn((box_bundle, BorderRadius::bottom_left(Val::Px(8.0))));
                    } else if row == (num_colours - 1) && col == (num_colours - 1) {
                        spawner.spawn((box_bundle, BorderRadius::bottom_right(Val::Px(8.0))));
                    } else {
                        spawner.spawn(box_bundle);
                    }
                });
            });
        })),
    )
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn update_matrix_size(
    params: Res<SimulationParams>,
    model_matrix_entity: Single<Entity, With<ModelMatrix>>,
    parameters_ui_entity: Single<Entity, With<Parameters>>,
    mut num_colours: Local<usize>,
    mut commands: Commands,
) {
    if !params.is_changed() {
        return;
    }

    tracing::info!("params changed");

    if params.num_colours == *num_colours {
        return;
    }

    tracing::info!("num_colours changed");

    *num_colours = params.num_colours;

    commands.entity(*model_matrix_entity).despawn();

    let new_matrix = commands.spawn(model_matrix(params.num_colours)).id();

    commands
        .entity(*parameters_ui_entity)
        .insert_children(0, &[new_matrix]);
}
