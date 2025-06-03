use bevy::prelude::*;

pub fn circle(color: impl Into<Color>) -> impl Bundle {
    (
        Node {
            width: Val::Px(50.0),
            height: Val::Px(50.0),

            ..default()
        },
        BorderRadius::all(Val::Px(25.0)),
        BackgroundColor(color.into().with_alpha(0.5)),
    )
}
