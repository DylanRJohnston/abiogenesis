use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Animator, Delay, Sequence, Tween};

use crate::{
    observe::observe,
    ui::{
        Layout, Sidebar,
        colours::{UI_BACKGROUND, UI_BACKGROUND_FOCUSED},
        icon::Icon,
        lenses::{BottomLens, LeftLens, TopLens},
        mixins,
        toolbar::ToolBar,
    },
};

pub fn hide_ui() -> impl Bundle {
    (
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
            ..default()
        },
        BorderRadius::all(Val::Px(16.0)),
        mixins::hover_colour(UI_BACKGROUND, UI_BACKGROUND_FOCUSED),
        mixins::tooltip("Behold"),
        children![(
            Node {
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..default()
            },
            Icon("icons/hide.png"),
            Pickable::IGNORE
        )],
        observe(
            |mut trigger: Trigger<Pointer<Click>>,
             mut commands: Commands,
             sidebar: Single<(Entity, &Sidebar)>,
             toolbar: Single<Entity, With<ToolBar>>,
             show_ui: Single<Entity, With<ShowUIButton>>| {
                trigger.propagate(false);

                let (sidebar, &Sidebar(layout)) = *sidebar;

                commands.entity(sidebar).insert(Animator::new(Tween::new(
                    EaseFunction::SmootherStepIn,
                    Duration::from_secs_f32(1.),
                    LeftLens {
                        start: 0.,
                        end: if layout == Layout::Horizontal {
                            -500.0
                        } else {
                            -250.0
                        },
                    },
                )));

                commands.entity(*toolbar).insert(Animator::new(Tween::new(
                    EaseFunction::SmootherStepIn,
                    Duration::from_secs_f32(1.),
                    BottomLens {
                        start: 0.0,
                        end: -100.,
                    },
                )));

                commands.entity(*show_ui).insert(Animator::new(
                    Sequence::from_single(Delay::new(Duration::from_secs_f32(0.750))).then(
                        Tween::new(
                            EaseFunction::SmootherStepOut,
                            Duration::from_secs_f32(1.),
                            TopLens {
                                start: -50.,
                                end: 0.,
                            },
                        ),
                    ),
                ));
            },
        ),
    )
}

#[derive(Component)]
struct ShowUIButton;

pub fn show_ui_button() -> impl Bundle {
    (
        ShowUIButton,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(24.),
            height: Val::Px(24.),
            top: Val::Px(-50.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::new(Val::Px(16.0), Val::default(), Val::Px(16.0), Val::default()),
            ..default()
        },
        mixins::tooltip("Intervene"),
        children![(
            Pickable::IGNORE,
            Node {
                width: Val::Px(24.),
                height: Val::Px(24.),
                ..default()
            },
            Icon("icons/more.png")
        )],
        observe(
            |mut trigger: Trigger<Pointer<Click>>,
             mut commands: Commands,
             sidebar: Single<(Entity, &Sidebar)>,
             toolbar: Single<Entity, With<ToolBar>>,
             show_ui: Single<Entity, With<ShowUIButton>>| {
                trigger.propagate(false);

                let (sidebar, &Sidebar(layout)) = *sidebar;

                commands.entity(sidebar).insert(Animator::new(
                    Sequence::from_single(Delay::new(Duration::from_secs_f32(0.4))).then(
                        Tween::new(
                            EaseFunction::SmootherStepOut,
                            Duration::from_secs_f32(1.),
                            LeftLens {
                                start: if layout == Layout::Horizontal {
                                    -500.0
                                } else {
                                    -250.0
                                },
                                end: 0.,
                            },
                        ),
                    ),
                ));

                commands.entity(*toolbar).insert(Animator::new(
                    Sequence::from_single(Delay::new(Duration::from_secs_f32(0.4))).then(
                        Tween::new(
                            EaseFunction::SmootherStepOut,
                            Duration::from_secs_f32(1.),
                            BottomLens {
                                start: -100.,
                                end: 0.0,
                            },
                        ),
                    ),
                ));

                commands.entity(*show_ui).insert(Animator::new(Tween::new(
                    EaseFunction::SmootherStepIn,
                    Duration::from_secs_f32(0.5),
                    TopLens {
                        start: 0.,
                        end: -50.,
                    },
                )));
            },
        ),
    )
}
