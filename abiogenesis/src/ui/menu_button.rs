use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Animator, Delay, Sequence, Tween};

use crate::{
    observe::observe,
    ui::{
        Layout, Sidebar, UIRoot,
        icon::Icon,
        lenses::{BottomLens, LeftLens, TopLens},
        mixins,
        toolbar::ToolBar,
    },
};

pub fn hide_ui() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        mixins::tooltip("Hide UI"),
        children![Icon("icons/hide.png")],
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
        mixins::tooltip("Show UI"),
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
                    Sequence::from_single(Delay::new(Duration::from_secs_f32(0.750))).then(
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
                    Sequence::from_single(Delay::new(Duration::from_secs_f32(0.750))).then(
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
                    Duration::from_secs_f32(1.),
                    TopLens {
                        start: 0.,
                        end: -50.,
                    },
                )));
            },
        ),
    )
}
