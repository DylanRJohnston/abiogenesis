use bevy::prelude::*;

use crate::{observe::Observe, ui::tooltip::tooltip};

pub fn button_hover_states() -> impl Bundle {
    (
        Observe::event(
            |event: Trigger<Pointer<Over>>,
             mut background_color: Query<&mut BackgroundColor, With<Button>>| {
                let Ok(mut color) = background_color.get_mut(event.target) else {
                    return;
                };

                *color = BackgroundColor(Color::WHITE.with_alpha(0.2));
            },
        ),
        Observe::event(
            |event: Trigger<Pointer<Out>>,
             mut background_color: Query<&mut BackgroundColor, With<Button>>| {
                let Ok(mut color) = background_color.get_mut(event.target) else {
                    return;
                };

                *color = BackgroundColor(Color::WHITE.with_alpha(0.1));
            },
        ),
        BackgroundColor(Color::WHITE.with_alpha(0.1)),
    )
}

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
        button_hover_states(),
        tooltip(text),
        Observe::event(move |_: Trigger<Pointer<Click>>, mut commands: Commands| {
            commands.trigger(event);
        }),
    )
}
