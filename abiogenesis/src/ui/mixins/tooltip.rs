use bevy::prelude::*;

use crate::observe::observe;

pub fn tooltip(text: impl Into<String>) -> impl Bundle {
    (
        observe(tooltip_hover_start(text.into())),
        observe(tooltip_hover),
        observe(tooltip_hover_end),
        observe(tooltip_touch_end),
    )
}

#[derive(Debug, Clone, Copy, Component)]
pub struct Tooltip;

fn tooltip_bundle(text: String, position: Vec2) -> impl Bundle {
    (
        Tooltip,
        Node {
            padding: UiRect::all(Val::Px(8.0)),
            top: Val::Px(position.y),
            left: Val::Px(position.x),
            ..default()
        },
        GlobalZIndex(100),
        Pickable::IGNORE,
        BackgroundColor(Color::BLACK.with_alpha(0.8)),
        BorderRadius::all(Val::Px(8.0)),
        children![(
            Text::new(text),
            TextFont::from_font_size(18.0),
            Pickable::IGNORE
        )],
    )
}

fn tooltip_hover_start(text: String) -> impl Fn(Trigger<Pointer<Over>>, Commands, Res<UiScale>) {
    move |trigger, mut commands, ui_scale| {
        commands.spawn(tooltip_bundle(
            text.clone(),
            trigger.pointer_location.position / **ui_scale,
        ));
    }
}

fn tooltip_hover(
    trigger: Trigger<Pointer<Move>>,
    mut tooltips: Query<&mut Node, With<Tooltip>>,
    ui_scale: Res<UiScale>,
) {
    tooltips.iter_mut().for_each(|mut tooltip| {
        tooltip.top = Val::Px(trigger.pointer_location.position.y / **ui_scale);
        tooltip.left = Val::Px(trigger.pointer_location.position.x / **ui_scale);
    });
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

fn tooltip_touch_end(
    _: Trigger<Pointer<Click>>,
    tooltips: Query<Entity, With<Tooltip>>,
    mut commands: Commands,
) {
    tooltips
        .iter()
        .for_each(|tooltip| commands.entity(tooltip).despawn());
}
