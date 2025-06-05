use bevy::prelude::*;

use crate::observe::Observe;

pub fn tooltip(text: impl Into<String>) -> impl Bundle {
    (
        Observe::event(tooltip_hover_start(text.into())),
        Observe::event(tooltip_hover),
        Observe::event(tooltip_hover_end),
    )
}

#[derive(Debug, Clone, Copy, Component)]
pub struct Tooltip;

fn tooltip_bundle(text: String) -> impl Bundle {
    (
        Tooltip,
        Node {
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::BLACK.with_alpha(0.8)),
        BorderRadius::all(Val::Px(8.0)),
        children![(Text::new(text), Pickable::IGNORE)],
    )
}

fn tooltip_hover_start(text: String) -> impl Fn(Trigger<Pointer<Over>>, Commands) {
    move |_, mut commands| {
        commands.spawn(tooltip_bundle(text.clone()));
    }
}

fn tooltip_hover(trigger: Trigger<Pointer<Move>>, mut tooltip: Single<&mut Node, With<Tooltip>>) {
    tooltip.top = Val::Px(trigger.pointer_location.position.y);
    tooltip.left = Val::Px(trigger.pointer_location.position.x);
}

fn tooltip_hover_end(
    _: Trigger<Pointer<Out>>,
    tooltips: Query<Entity, With<Tooltip>>,
    mut commands: Commands,
) {
    tooltips
        .iter()
        .for_each(|tooltip| commands.entity(tooltip).despawn());
}
