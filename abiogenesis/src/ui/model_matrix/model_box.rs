use std::time::Duration;

use bevy::{prelude::*, window::SystemCursorIcon, winit::cursor::CursorIcon};

use bevy_tweening::{Animator, Tween, lens::UiPositionLens};

use crate::{
    observe::observe,
    particles::{colour::ParticleColour, model::Model},
    ui::mixins,
};

#[derive(Debug, Component, Clone, Copy, Reflect)]
pub struct ModelIndex {
    pub source: ParticleColour,
    pub target: ParticleColour,
}

pub fn model_box(source: ParticleColour, target: ParticleColour) -> impl Bundle {
    let text = if source == target {
        format!("{source}'s attraction to itself")
    } else {
        format!("{source}'s attraction to {target}")
    };

    (
        ModelIndex { source, target },
        Node {
            padding: UiRect::all(Val::Px(8.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Pickable {
            should_block_lower: true,
            is_hoverable: true,
        },
        children![(
            Text::new("0"),
            TextFont::from_font_size(24.0),
            TextColor::default(),
            Pickable::IGNORE,
        )],
        mixins::tooltip(text),
        observe(drag_start),
        observe(drag),
        observe(drag_end),
        mixins::cursor_grab_icon(),
        BackgroundColor(Color::NONE),
    )
}

const SLIDER_SCALAR: f32 = 50.0;

#[derive(Component, Reflect, Default, Clone, Copy, Deref, DerefMut)]
struct DragStartValue(f32);

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn drag_start(
    trigger: Trigger<Pointer<DragStart>>,
    model: Res<Model>,
    indexes: Query<&ModelIndex>,
    mut commands: Commands,
) {
    let Ok(index) = indexes.get(trigger.target) else {
        return;
    };

    commands.entity(trigger.target).insert((
        DragStartValue(SLIDER_SCALAR * model.weight(index.source, index.target)),
        GlobalZIndex(50),
    ));
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn drag(
    trigger: Trigger<Pointer<Drag>>,
    mut model: ResMut<Model>,
    mut nodes: Query<(
        &ModelIndex,
        &DragStartValue,
        &mut Node,
        &mut BackgroundColor,
    )>,
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
) {
    let Ok((index, start_value, mut node, mut color)) = nodes.get_mut(trigger.target) else {
        return;
    };

    let lower_bound = -SLIDER_SCALAR - **start_value;
    let upper_bound = SLIDER_SCALAR - **start_value;

    color.0 = color.0.with_alpha(1.0);
    node.left = Val::Px(trigger.distance.x.clamp(lower_bound, upper_bound));
    model.set_weight(
        index.source,
        index.target,
        ((**start_value + trigger.distance.x) / SLIDER_SCALAR).clamp(-1.0, 1.0),
    );

    commands
        .entity(*window)
        .insert(CursorIcon::from(SystemCursorIcon::Grab));
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn drag_end(
    trigger: Trigger<Pointer<DragEnd>>,
    window: Single<Entity, With<Window>>,
    mut commands: Commands,
    mut nodes: Query<(&Node, &mut BackgroundColor)>,
) {
    commands
        .entity(*window)
        .insert(CursorIcon::from(SystemCursorIcon::Default));

    let Ok((node, mut color)) = nodes.get_mut(trigger.target) else {
        return;
    };

    color.0 = color.0.with_alpha(0.5);

    let tween = Tween::new(
        EaseFunction::BounceOut,
        Duration::from_secs_f32(0.75),
        UiPositionLens {
            start: UiRect::left(node.left),
            end: UiRect::all(Val::Px(0.0)),
        },
    );

    commands
        .entity(trigger.target)
        .insert(Animator::new(tween))
        .remove::<DragStartValue>()
        .remove::<GlobalZIndex>();
}
