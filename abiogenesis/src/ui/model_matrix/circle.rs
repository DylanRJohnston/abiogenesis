use bevy::prelude::*;

pub fn circle(color: impl Into<Color>) -> impl Bundle {
    (
        Node {
            // width: Val::Percent(100.0),
            // height: Val::Percent(100.0),
            // aspect_ratio: Some(1.0),
            ..default()
        },
        BorderRadius::all(Val::Percent(50.0)),
        BackgroundColor(color.into().with_alpha(0.5)),
    )
}
