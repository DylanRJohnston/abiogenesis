use bevy::prelude::*;

use crate::{observe::observe, ui::mixins};

pub fn control_button(
    text: &'static str,
    event: impl Event + Copy,
    icon: Handle<Image>,
) -> impl Bundle {
    (
        Node {
            padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
            // width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        Button,
        BorderRadius::all(Val::Px(16.0)),
        Pickable::default(),
        children![
            // (Text::new(text), Pickable::IGNORE),
            (
                Node {
                    width: Val::Px(25.0),
                    height: Val::Px(25.0),
                    ..default()
                },
                ImageNode {
                    image: icon,

                    ..default()
                },
                Pickable::IGNORE
            ),
        ],
        mixins::hover_colour(Color::WHITE.with_alpha(0.1), Color::WHITE.with_alpha(0.2)),
        mixins::tooltip(text),
        observe(move |_: Trigger<Pointer<Click>>, mut commands: Commands| {
            commands.trigger(event);
        }),
    )
}
