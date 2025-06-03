use bevy::prelude::*;

use crate::{observe::Observe, particles::Model, ui::button::button_hover_states};

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

fn randomise_model(_: Trigger<Pointer<Click>>, mut model: ResMut<Model>) {
    model.weights.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|value| {
            *value = rand::random::<f32>() * 2.0 - 1.0;
        })
    });
}
