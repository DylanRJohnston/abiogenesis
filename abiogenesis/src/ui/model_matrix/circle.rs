use bevy::prelude::*;

use crate::particles::colour::ParticleColour;

pub fn circle(color: ParticleColour) -> impl Bundle {
    (
        Node {
            // width: Val::Percent(100.0),
            // height: Val::Percent(100.0),
            // aspect_ratio: Some(1.0),
            ..default()
        },
        BorderRadius::all(Val::Percent(50.0)),
        BackgroundColor(Color::from(color).with_alpha(0.5)),
    )
}
