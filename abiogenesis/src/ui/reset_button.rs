use bevy::prelude::*;

use crate::{observe::Observe, particles::spawner::Respawn, ui::button::button_hover_states};

pub fn reset_button() -> impl Bundle {
    (
        Node {
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        Button,
        children![(Text::new("Reset"), Pickable::IGNORE)],
        button_hover_states(),
        Observe::event(reset_model),
    )
}

fn reset_model(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(Respawn);
}
