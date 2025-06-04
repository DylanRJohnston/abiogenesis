use bevy::prelude::*;

use crate::{observe::Observe, particles::model::Randomise, ui::button::button_hover_states};

pub fn randomise_button() -> impl Bundle {
    (
        Node {
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        Button,
        children![(Text::new("Randomise"), Pickable::IGNORE)],
        Observe::event(randomise_model),
        button_hover_states(),
    )
}

fn randomise_model(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(Randomise);
}
