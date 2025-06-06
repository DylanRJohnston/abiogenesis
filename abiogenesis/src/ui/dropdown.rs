use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Animator, EaseMethod, Lens, Tween, lens::TransformRotationLens};

use crate::{
    math::lerp,
    observe::observe,
    ui::{icon::Icon, mixins},
};

pub struct HeightLens {
    start: f32,
    end: f32,
}

impl Lens<Node> for HeightLens {
    fn lerp(&mut self, target: &mut dyn bevy_tweening::Targetable<Node>, ratio: f32) {
        target.height = Val::Px(lerp(self.start, self.end, ratio));
    }
}

#[derive(Debug, Component, PartialEq, Eq, Default)]
enum DropdownState {
    #[default]
    Closed,
    Open,
}

#[derive(Debug, Component)]
#[require(DropdownState)]
struct Dropdown {
    content_height: f32,
}

const HEADER_HEIGHT: f32 = 24.0;
const HEADER_PADDING: f32 = 8.0;
const ANIMATION_LENGTH: f32 = 0.5;

pub fn dropdown(
    title: &'static str,
    header_tooltip: &'static str,
    height: f32,
    contents: impl Bundle,
) -> impl Bundle {
    (
        Dropdown {
            content_height: height,
        },
        Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            overflow: Overflow::clip(),
            height: Val::Px(HEADER_HEIGHT + HEADER_PADDING * 2.0),
            ..default()
        },
        BorderRadius::all(Val::Px(8.0)),
        BackgroundColor(Color::WHITE.with_alpha(0.1)),
        children![header(title, header_tooltip), contents],
        observe(toggle_state),
    )
}

#[derive(Debug, Component)]
struct DropdownIcon;

#[derive(Debug, Component)]
pub struct HeaderText;

fn header(title: &'static str, tooltip: &'static str) -> impl Bundle {
    (
        Node {
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(HEADER_PADDING)),
            ..default()
        },
        BorderRadius::top(Val::Px(8.0)),
        Pickable {
            should_block_lower: true,
            is_hoverable: true,
        },
        mixins::hover_colour(Color::NONE, Color::WHITE.with_alpha(0.1)),
        mixins::tooltip(tooltip),
        observe(
            |mut trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                trigger.propagate(false);
                commands.trigger_targets(ToggleState, trigger.target)
            },
        ),
        children![
            (
                HeaderText,
                Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Text::from(title),
                Pickable::IGNORE,
            ),
            (
                DropdownIcon,
                Node {
                    width: Val::Px(24.0),
                    height: Val::Px(24.0),
                    ..default()
                },
                Icon("icons/dropdown.png"),
                Transform::default()
                    .with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2)),
                Pickable::IGNORE,
            )
        ],
    )
}

#[derive(Debug, Event, Clone, Copy)]
#[event(auto_propagate, traversal = &'static ChildOf)]
pub struct ToggleState;

fn toggle_state(
    trigger: Trigger<ToggleState>,
    mut commands: Commands,
    dropdowns: Query<(Entity, &DropdownState, &Dropdown)>,
    icons: Query<(Entity, &Transform), With<DropdownIcon>>,
    children: Query<&Children>,
) -> Result<()> {
    let Ok((dropdown_entity, dropdown_state, dropdown)) = dropdowns.get(trigger.target()) else {
        return Ok(());
    };

    let new_state = match dropdown_state {
        DropdownState::Closed => DropdownState::Open,
        DropdownState::Open => DropdownState::Closed,
    };

    let closed_height = HEADER_HEIGHT + HEADER_PADDING * 2.0;
    let open_height = HEADER_HEIGHT + HEADER_PADDING * 4.0 + dropdown.content_height;

    let lens = match dropdown_state {
        DropdownState::Closed => HeightLens {
            start: closed_height,
            end: open_height,
        },
        DropdownState::Open => HeightLens {
            start: open_height,
            end: closed_height,
        },
    };

    let tween = Tween::new(
        EaseMethod::EaseFunction(EaseFunction::SmootherStepOut),
        Duration::from_secs_f32(ANIMATION_LENGTH),
        lens,
    );

    commands
        .entity(trigger.target())
        .insert((new_state, Animator::new(tween)));

    let (icon_entity, icon_transform) = children
        .iter_descendants(dropdown_entity)
        .filter_map(|child| icons.get(child).ok())
        .next()
        .ok_or("Failed to find icon for dropdown")?;

    let next_rotation = match dropdown_state {
        DropdownState::Closed => Quat::IDENTITY,
        DropdownState::Open => Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
    };

    let tween = Tween::new(
        EaseMethod::EaseFunction(EaseFunction::SmootherStepOut),
        Duration::from_secs_f32(ANIMATION_LENGTH),
        TransformRotationLens {
            start: icon_transform.rotation,
            end: next_rotation,
        },
    );

    commands.entity(icon_entity).insert(Animator::new(tween));

    Ok(())
}
