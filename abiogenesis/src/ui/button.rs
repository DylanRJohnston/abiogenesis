use bevy::prelude::*;

use crate::observe::Observe;

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

pub fn control_button(text: &str, event: impl Event + Copy) -> impl Bundle {
    (
        Node {
            padding: UiRect::all(Val::Px(8.0)),
            width: Val::Percent(100.0),
            ..default()
        },
        Button,
        children![(Text::new(text), Pickable::IGNORE)],
        button_hover_states(),
        Observe::event(move |_: Trigger<Pointer<Click>>, mut commands: Commands| {
            commands.trigger(event);
        }),
    )
}
