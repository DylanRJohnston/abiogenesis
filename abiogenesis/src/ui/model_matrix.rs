use bevy::{color::palettes::css::GREY, ecs::spawn::SpawnWith, prelude::*};

mod circle;
mod model_box;

use circle::*;
use model_box::*;

use crate::particles::{colour::*, model::Model};

#[cfg_attr(
    feature = "hot_reload",
    bevy_simple_subsecond_system::hot(rerun_on_hot_patch = true)
)]
pub fn update_model_matrix(
    mut elements: Query<(&ModelIndex, &mut BackgroundColor, &Children)>,
    mut text: Query<&mut Text>,
    model: Res<Model>,
) {
    if !model.is_changed() {
        return;
    }

    for (value, mut color, children) in elements.iter_mut() {
        let value = model[value.source][value.target];

        **(text.get_mut(children[0]).unwrap()) = format!("{value:.0}", value = value * 10.0);

        if value <= -0.0 {
            *color = Color::from(GREY)
                .mix(&RED, (-value).powf(0.5))
                .with_alpha(color.0.alpha())
                .into()
        } else {
            *color = Color::from(GREY)
                .mix(&GREEN, value.powf(0.5))
                .with_alpha(color.0.alpha())
                .into()
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

pub fn model_matrix() -> impl Bundle {
    (
        Name::from("Model Matrix"),
        Node {
            display: Display::Grid,
            grid_template_columns: vec![RepeatedGridTrack::px(4, 40.)],
            grid_template_rows: vec![RepeatedGridTrack::px(4, 40.)],
            justify_content: JustifyContent::Start,
            align_content: AlignContent::Start,
            justify_items: JustifyItems::Stretch,
            align_items: AlignItems::Stretch,
            row_gap: Val::Px(4.0),
            column_gap: Val::Px(4.0),
            ..default()
        },
        // children![] has a maximum limit of children
        many_children!(
            Node::default(),
            circle(RED),
            circle(GREEN),
            circle(BLUE),
            // circle(ORANGE),
            circle(RED),
            (model_box(0, 0), BorderRadius::top_left(Val::Px(8.0))),
            model_box(0, 1),
            (model_box(0, 2), BorderRadius::top_right(Val::Px(8.0))),
            // model_box(3, 0),
            circle(GREEN),
            model_box(1, 0),
            model_box(1, 1),
            model_box(1, 2),
            // model_box(3, 1),
            circle(BLUE),
            (model_box(2, 0), BorderRadius::bottom_left(Val::Px(8.0))),
            model_box(2, 1),
            (model_box(2, 2), BorderRadius::bottom_right(Val::Px(8.0))),
            // model_box(3, 2),
            // circle(ORANGE),
            // model_box(0, 3),
            // model_box(1, 3),
            // model_box(2, 3),
            // model_box(3, 3),
        ),
    )
}
