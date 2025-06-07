use std::time::Duration;

use bevy::{color::palettes::css::WHITE, prelude::*};
use bevy_tweening::{Animator, Tween};

use crate::{
    camera::FollowParticle,
    observe::observe,
    particles::colour::ParticleColour,
    ui::{
        colours::{UI_BACKGROUND, UI_BACKGROUND_FOCUSED},
        icon::Icon,
        lenses::{BottomLens, LeftLens},
        mixins,
    },
};

#[derive(Debug, Clone, Copy, Resource, Component, PartialEq, Eq, Event)]
#[event(auto_propagate, traversal = &'static ChildOf)]
pub enum Tool {
    Camera,
    Particle(ParticleColour),
    Eraser,
}

impl Tool {
    fn index(&self) -> usize {
        match self {
            Tool::Camera => 0,
            Tool::Eraser => 1,
            Tool::Particle(ParticleColour::Red) => 2,
            Tool::Particle(ParticleColour::Green) => 3,
            Tool::Particle(ParticleColour::Blue) => 4,
            Tool::Particle(ParticleColour::Orange) => 5,
        }
    }
}

pub struct ToolBarPlugin;

impl Plugin for ToolBarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Tool::Camera).add_systems(
            Update,
            update_camera.run_if(resource_changed_or_removed::<FollowParticle>),
        );
    }
}

#[derive(Component)]
pub struct ToolBar;

pub fn toolbar() -> impl Bundle {
    (
        ToolBar,
        Node {
            position_type: PositionType::Absolute,
            height: Val::Px(50.0),
            margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Auto, Val::Px(16.0)),
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            ..default()
        },
        BackgroundColor(UI_BACKGROUND),
        BorderRadius::all(Val::Percent(50.0)),
        mixins::block_all_interactions(),
        observe(update_toolbar_select),
        Animator::new(Tween::new(
            EaseFunction::SmootherStepOut,
            Duration::from_secs_f32(1.),
            BottomLens {
                start: -100.,
                end: 0.0,
            },
        )),
        children![
            selection(),
            camera_tool(),
            eraser_tool(),
            particle_tool(ParticleColour::Red),
            particle_tool(ParticleColour::Green),
            particle_tool(ParticleColour::Blue),
        ],
    )
}

#[derive(Component)]
struct Selection;

fn selection() -> impl Bundle {
    (
        Selection,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(50.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderRadius::all(Val::Percent(50.0)),
        BackgroundColor(UI_BACKGROUND_FOCUSED),
    )
}

const TOOL_SIZE: f32 = 50.0;

fn tool() -> impl Bundle {
    (
        Node {
            width: Val::Px(TOOL_SIZE),
            height: Val::Px(TOOL_SIZE),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderRadius::all(Val::Percent(50.0)),
        observe(
            move |mut trigger: Trigger<Pointer<Click>>,
                  mut commands: Commands,
                  tools: Query<&Tool>|
                  -> Result<()> {
                trigger.propagate(false);

                commands.trigger_targets(*tools.get(trigger.target)?, trigger.target);

                Ok(())
            },
        ),
    )
}

fn camera_tool() -> impl Bundle {
    (
        Tool::Camera,
        tool(),
        mixins::tooltip("Witness"),
        observe(
            |mut trigger: Trigger<Pointer<Click>>,
             tool: Res<Tool>,
             mut commands: Commands|
             -> Result<()> {
                trigger.propagate(false);
                if *tool == Tool::Camera {
                    commands.remove_resource::<FollowParticle>();
                }

                Ok(())
            },
        ),
        children![(
            Node {
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..default()
            },
            Pickable::IGNORE,
            Icon("icons/eye_empty.png"),
        )],
    )
}

fn eraser_tool() -> impl Bundle {
    (
        Tool::Eraser,
        tool(),
        mixins::tooltip("Smite"),
        children![(
            Node {
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..default()
            },
            Pickable::IGNORE,
            Icon("icons/human-skull.png"),
        )],
    )
}

fn particle_tool(color: ParticleColour) -> impl Bundle {
    (
        Tool::Particle(color),
        tool(),
        mixins::tooltip(format!("Awaken {color:?} Life")),
        children![(
            Node {
                width: Val::Px(36.0),
                height: Val::Px(36.0),
                ..default()
            },
            Pickable::IGNORE,
            BorderRadius::all(Val::Percent(50.0)),
            BackgroundColor(Color::from(color).with_alpha(0.8)),
        )],
    )
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn update_toolbar_select(
    mut trigger: Trigger<Tool>,
    mut selected_tool: ResMut<Tool>,
    selection: Single<Entity, With<Selection>>,
    mut commands: Commands,
) -> Result<()> {
    trigger.propagate(false);

    let start = selected_tool.index() as f32 * TOOL_SIZE;
    let end = trigger.index() as f32 * TOOL_SIZE;

    *selected_tool = *trigger;

    let tween = Tween::new(
        EaseFunction::CubicInOut,
        Duration::from_secs_f32(0.2),
        LeftLens { start, end },
    );

    commands.entity(*selection).insert(Animator::new(tween));

    Ok(())
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn update_camera(
    follow_particle: Option<Res<FollowParticle>>,
    icons: Query<Entity, With<Icon>>,
    children: Query<&Children>,
    tools: Query<(Entity, &Tool)>,
    mut commands: Commands,
) -> Result<()> {
    let camera_tool = tools
        .iter()
        .find_map(|(entity, tool)| {
            if *tool == Tool::Camera {
                Some(entity)
            } else {
                None
            }
        })
        .ok_or("failed to find camera tool")?;

    let camera_icon = children
        .iter_descendants(camera_tool)
        .find_map(|child| icons.get(child).ok())
        .ok_or("failed to find icon for the camera tool")?;

    match follow_particle {
        Some(_) => commands
            .entity(camera_icon)
            .insert(Icon("icons/eye_full.png")),
        None => commands
            .entity(camera_icon)
            .insert(Icon("icons/eye_empty.png")),
    };

    Ok(())
}
