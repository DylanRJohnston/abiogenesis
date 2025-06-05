use bevy::{ecs::spawn::SpawnIter, prelude::*};

use crate::{
    observe::Observe,
    particles::model::{Model, PRESETS},
    ui::{
        button::button_hover_states,
        tooltip::{Tooltip, tooltip},
    },
};

#[derive(Debug, Component)]
struct Dropdown;

pub fn preset_dropdown(selected: usize, icon: Handle<Image>) -> impl Bundle {
    (
        Dropdown,
        Node {
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(Val::Px(8.0)),
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.0),
            row_gap: Val::Px(8.0),
            overflow: Overflow::clip(),
            ..default()
        },
        Button,
        BorderRadius::all(Val::Px(8.0)),
        Pickable::default(),
        button_hover_states(),
        tooltip("Select a preset"),
        Observe::event(on_dropdown),
        children![
            (Text::new(PRESETS[selected].0), Pickable::IGNORE),
            (
                Node {
                    width: Val::Px(24.0),
                    height: Val::Px(24.0),
                    ..default()
                },
                ImageNode::new(icon),
                Pickable::IGNORE
            )
        ],
    )
}

fn on_dropdown(
    trigger: Trigger<Pointer<Click>>,
    child_of: Query<&ChildOf>,
    tooltips: Query<Entity, With<Tooltip>>,
    mut commands: Commands,
) {
    let Ok(&ChildOf(parent)) = child_of.get(trigger.target) else {
        return;
    };

    commands.entity(trigger.target).despawn();
    tooltips
        .iter()
        .for_each(|tooltip| commands.entity(tooltip).despawn());

    let child = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                width: Val::Percent(100.0),
                row_gap: Val::Px(4.0),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::WHITE.with_alpha(0.1)),
            BorderRadius::all(Val::Px(16.0)),
            Children::spawn(SpawnIter(PRESETS.iter().enumerate().map(
                |(index, (name, _))| {
                    (
                        Node {
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(8.0)),
                            width: Val::Percent(100.0),
                            overflow: Overflow::clip(),
                            ..default()
                        },
                        Button,
                        Observe::event(on_select(index)),
                        match index {
                            0 => BorderRadius::top(Val::Px(8.0)),
                            bottom if bottom == PRESETS.len() - 1 => {
                                BorderRadius::bottom(Val::Px(8.0))
                            }
                            _ => BorderRadius::default(),
                        },
                        button_hover_states(),
                        children![(Text::new(*name), Pickable::IGNORE)],
                    )
                },
            ))),
        ))
        .id();

    commands.entity(parent).add_child(child);
}

fn on_select(
    index: usize,
) -> impl Fn(Trigger<'_, Pointer<Click>>, Query<&ChildOf>, ResMut<Model>, Res<AssetServer>, Commands)
{
    move |mut trigger, child_of, mut model, assets, mut commands| {
        trigger.propagate(false);

        *model = PRESETS[index].1;

        let Ok(&ChildOf(parent)) = child_of.get(trigger.target) else {
            return;
        };

        let Ok(&ChildOf(container)) = child_of.get(parent) else {
            return;
        };

        commands.entity(parent).despawn();
        let child = commands
            .spawn(preset_dropdown(index, assets.load("icons/dropdown.png")))
            .id();
        commands.entity(container).add_child(child);
    }
}
