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
    Smite,
}

impl Tool {
    fn index(&self) -> usize {
        match self {
            Tool::Camera => 0,
            Tool::Smite => 1,
            Tool::Particle(ParticleColour::Red) => 2,
            Tool::Particle(ParticleColour::Green) => 3,
            Tool::Particle(ParticleColour::Blue) => 4,
            Tool::Particle(ParticleColour::Orange) => 5,
            Tool::Particle(ParticleColour::Pink) => 6,
            Tool::Particle(ParticleColour::Aqua) => 7,
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
            bottom: Val::Px(-100.),
            ..default()
        },
        BackgroundColor(UI_BACKGROUND),
        BorderRadius::all(Val::Percent(50.0)),
        mixins::block_all_interactions(),
        observe(update_toolbar_select),
        Animator::new(Tween::new(
            EaseFunction::SmootherStepOut,
            Duration::from_secs_f32(1.5),
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
        Tool::Smite,
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

#[derive(Debug, Component)]
pub struct HoverRegion;

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn smite_start_hover(trigger: Trigger<Pointer<Over>>, tool: Res<Tool>, mut commands: Commands) {
    if Tool::Smite != *tool {
        return;
    };

    let position = trigger.pointer_location.position - Vec2::new(30., 30.);

    commands.spawn((
        HoverRegion,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(60.),
            height: Val::Px(60.),
            left: Val::Px(position.x),
            top: Val::Px(position.y),
            ..default()
        },
        GlobalZIndex(-100),
        BackgroundColor(Color::from(WHITE).with_alpha(0.05)),
        BorderRadius::all(Val::Percent(50.)),
    ));
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn smite_hover(
    trigger: Trigger<Pointer<Move>>,
    mut hover_region: Single<&mut Node, With<HoverRegion>>,
) {
    let position = trigger.pointer_location.position - Vec2::new(30., 30.);

    hover_region.top = Val::Px(position.y);
    hover_region.left = Val::Px(position.x);
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn smite_end_hover(
    _trigger: Trigger<Pointer<Out>>,
    hover_region: Query<Entity, With<HoverRegion>>,
    mut commands: Commands,
) {
    for entity in hover_region.iter() {
        commands.entity(entity).despawn();
    }
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn smite_end_click(
    _trigger: Trigger<Pointer<Pressed>>,
    hover_region: Query<Entity, With<HoverRegion>>,
    mut commands: Commands,
) {
    for entity in hover_region.iter() {
        commands.entity(entity).despawn();
    }
}
