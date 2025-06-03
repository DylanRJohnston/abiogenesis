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
        BackgroundColor(Color::WHITE.with_alpha(0.05)),
    )
}
