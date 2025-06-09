use std::time::Duration;

use bevy::{prelude::*, window::WindowResized};
use bevy_tweening::{Animator, Delay, Sequence, Tween, TweenCompleted};

use crate::{
    math::remap,
    observe::observe,
    ui::{lenses::TextColourLens, respawn_ui},
};

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_title_screen)
            .add_systems(Update, calculate_ui_scale);
    }
}

#[cfg_attr(
    feature = "hot_reload",
    bevy_simple_subsecond_system::hot(rerun_on_hot_patch = false)
)]
fn spawn_title_screen(
    mut commands: Commands,
    title_screens: Query<Entity, With<TitleScreen>>,
    window: Single<&Window>,
    ui_scale: Res<UiScale>,
) {
    title_screens
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());

    tracing::warn!(width = ?window.width(), ?ui_scale);

    commands.spawn(title_screen(
        remap(window.width(), 362.0, 1280.0, 40., 100.).min(100.),
        remap(window.width(), 362.0, 1280.0, 16., 20.).min(20.),
    ));
}

#[derive(Component)]
struct Title;

#[derive(Component)]
struct SubTitle;

#[cfg_attr(
    feature = "hot_reload",
    bevy_simple_subsecond_system::hot(rerun_on_hot_patch = true)
)]
fn calculate_ui_scale(
    mut resize_reader: EventReader<WindowResized>,
    window: Single<&Window>,
    mut title: Single<&mut TextFont, (With<Title>, Without<SubTitle>)>,
    mut subtitle: Single<&mut TextFont, (With<SubTitle>, Without<Title>)>,
) {
    if let Some(_) = resize_reader.read().last() {
        title.font_size = remap(window.width(), 362.0, 1280.0, 40., 100.).min(100.);
        subtitle.font_size = remap(window.width(), 362.0, 1280.0, 16., 20.).min(20.);
    }
}

#[derive(Component)]
struct TitleScreen;

fn title_screen(title_size: f32, subtitle_size: f32) -> impl Bundle {
    (
        TitleScreen,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::axes(Val::Px(24.0), Val::default()),
            ..default()
        },
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            children![
                (
                    Title,
                    Text::from("A B I O G E N E S I S"),
                    TextFont::from_font_size(title_size),
                    TextLayout::new_with_linebreak(LineBreak::NoWrap),
                    TextColor(Color::WHITE.with_alpha(0.0)),
                    Animator::new(
                        Sequence::from_single(Tween::new(
                            EaseFunction::SmoothStepIn,
                            Duration::from_secs_f32(5.0),
                            TextColourLens {
                                start: Color::WHITE.with_alpha(0.0),
                                end: Color::WHITE,
                            }
                        ))
                        .then(Delay::new(Duration::from_secs_f32(5.0)))
                        .then(
                            Tween::new(
                                EaseFunction::SmoothStepOut,
                                Duration::from_secs_f32(1.0),
                                TextColourLens {
                                    start: Color::WHITE,
                                    end: Color::WHITE.with_alpha(0.0),
                                }
                            )
                            .with_completed_event(0)
                        )
                    ),
                    observe(
                        |trigger: Trigger<TweenCompleted>,
                         title_screen: Single<Entity, With<TitleScreen>>,
                         mut commands: Commands| {
                            match trigger.user_data {
                                0 => {
                                    commands.run_system_cached(respawn_ui);
                                    commands.entity(*title_screen).despawn();
                                }
                                other => {
                                    tracing::warn!(?other, "unrecognized tween completed event");
                                }
                            }
                        }
                    )
                ),
                (
                    SubTitle,
                    Node {
                        justify_self: JustifySelf::Start,
                        ..default()
                    },
                    Text::from(
                        "abiogenesis. noun. abio·​gen·​e·​sis : the origin of life from nonliving matter"
                    ),
                    TextFont::from_font_size(subtitle_size),
                    TextColor(Color::WHITE.with_alpha(0.0)),
                    Animator::new(
                        Sequence::from_single(Delay::new(Duration::from_secs_f32(2.0)))
                            .then(Tween::new(
                                EaseFunction::SmoothStepIn,
                                Duration::from_secs_f32(5.0),
                                TextColourLens {
                                    start: Color::WHITE.with_alpha(0.0),
                                    end: Color::WHITE,
                                }
                            ))
                            .then(Delay::new(Duration::from_secs_f32(3.0)))
                            .then(Tween::new(
                                EaseFunction::SmoothStepOut,
                                Duration::from_secs_f32(1.0),
                                TextColourLens {
                                    start: Color::WHITE,
                                    end: Color::WHITE.with_alpha(0.0),
                                }
                            ))
                    )
                )
            ]
        )],
    )
}
