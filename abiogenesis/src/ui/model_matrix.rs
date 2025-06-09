use bevy::{ecs::spawn::SpawnWith, prelude::*};

mod circle;
mod model_box;
mod num_colours;

use circle::*;
use model_box::*;
use num_colours::*;

use crate::particles::{
    colour::{ParticleColour::*, *},
    model::Model,
};

const MID_COLOR: Color = Color::Srgba(Srgba::rgb(233.0 / 255.0, 133.0 / 255.0, 55.0 / 255.0));

#[cfg_attr(
    feature = "hot_reload",
    bevy_simple_subsecond_system::hot(rerun_on_hot_patch = true)
)]
pub fn update_model_matrix(
    mut elements: Query<(&ModelIndex, &mut BackgroundColor, &Children)>,
    mut text: Query<&mut Text>,
    model: Res<Model>,
) {
    for (value, mut colour, children) in elements.iter_mut() {
        let value = model.weight(value.source, value.target);

        **(text.get_mut(children[0]).unwrap()) = format!("{value:.0}", value = value * 10.0);

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
struct ModelMatrix;

pub fn model_matrix() -> impl Bundle {
    (
        ModelMatrix,
        Name::from("Model Matrix"),
        Node {
            display: Display::Grid,
            width: Val::Px(MODEL_MATRIX_SIZE),
            height: Val::Px(MODEL_MATRIX_SIZE),
            grid_template_columns: vec![RepeatedGridTrack::flex(4, 1.0)],
            grid_template_rows: vec![RepeatedGridTrack::flex(4, 1.0)],
            justify_content: JustifyContent::Stretch,
            align_content: AlignContent::Start,
            justify_items: JustifyItems::Stretch,
            align_items: AlignItems::Stretch,
            row_gap: Val::Px(4.0),
            column_gap: Val::Px(4.0),
            ..default()
        },
        // children![] has a maximum limit of children
        many_children!(
            num_colours(),
            circle(Red),
            circle(Green),
            circle(Blue),
            circle(Red),
            (model_box(Red, Red), BorderRadius::top_left(Val::Px(8.0))),
            model_box(Red, Green),
            (model_box(Red, Blue), BorderRadius::top_right(Val::Px(8.0))),
            circle(Green),
            model_box(Green, Red),
            model_box(Green, Green),
            model_box(Green, Blue),
            circle(Blue),
            (
                model_box(Blue, Red),
                BorderRadius::bottom_left(Val::Px(8.0))
            ),
            model_box(Blue, Green),
            (
                model_box(Blue, Blue),
                BorderRadius::bottom_right(Val::Px(8.0))
            ),
        ),
    )
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn update_matrix_size() {}
