use bevy::{prelude::*, text::FontStyle};

use crate::{
    observe::observe,
    particles::simulation::SimulationParams,
    ui::{colours::UI_BACKGROUND_FOCUSED, mixins},
};

pub fn change_num_colours() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceAround,
            ..default()
        },
        children![
            (
                button("-", "Decrease the number of colours"),
                observe(
                    |mut trigger: Trigger<Pointer<Click>>, mut params: ResMut<SimulationParams>| {
                        trigger.propagate(false);

                        params.num_colours = (params.num_colours - 1).max(1);
                    }
                )
            ),
            (
                button("+", "Increase the number of colours"),
                observe(
                    |mut trigger: Trigger<Pointer<Click>>, mut params: ResMut<SimulationParams>| {
                        trigger.propagate(false);

                        params.num_colours = (params.num_colours + 1).min(6);
                    }
                )
            )
        ],
    )
}

fn button(text: &'static str, tooltip: &'static str) -> impl Bundle {
    (
        Node {
            width: Val::Percent(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderRadius::all(Val::Px(8.0)),
        mixins::hover_colour(Color::NONE, UI_BACKGROUND_FOCUSED),
        mixins::tooltip(tooltip),
        children![(
            Text::from(text),
            TextFont::from_font_size(24.0),
            Pickable::IGNORE
        )],
    )
}
