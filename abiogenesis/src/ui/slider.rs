use std::marker::PhantomData;

use bevy::{prelude::*, ui::UiSystem};

use crate::{
    bundle_fn::BundleFn, math::remap, observe::observe, particles::simulation::SimulationParams,
    ui::mixins,
};

const SLIDER_SIZE: f32 = 20.0;
pub const COMPONENT_SIZE: f32 = SLIDER_SIZE + 16.0 * 1.2;

pub struct Slider<R: Resource> {
    pub name: &'static str,
    pub lower: f32,
    pub upper: f32,
    pub lens: fn(&mut R) -> &mut f32,
}

impl<R: Resource> Slider<R> {
    pub fn into_bundle(self) -> impl Bundle {
        (
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            children![
                (Text::new(self.name), TextFont::from_font_size(16.0)),
                (
                    Node {
                        position_type: PositionType::Relative,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        height: Val::Px(SLIDER_SIZE),
                        ..default()
                    },
                    children![
                        (
                            Node {
                                width: Val::Percent(100.),
                                height: Val::Px(10.0),
                                ..default()
                            },
                            BorderRadius::all(Val::Px(8.0)),
                            BackgroundColor(Color::WHITE.with_alpha(0.1))
                        ),
                        (
                            Node {
                                position_type: PositionType::Absolute,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                width: Val::Px(SLIDER_SIZE),
                                height: Val::Px(SLIDER_SIZE),
                                left: Val::Px(0.),
                                ..default()
                            },
                            BorderRadius::all(Val::Px(8.0)),
                            BackgroundColor(Color::WHITE.with_alpha(0.8)),
                            mixins::cursor_grab_icon(),
                            SliderComponent {
                                lower: self.lower,
                                upper: self.upper,
                                lens: self.lens,
                            },
                            BundleFn(register_slider_system_once::<SimulationParams>),
                            observe(drag::<SimulationParams>),
                            children![(
                                Text::new("0"),
                                TextFont::from_font_size(14.0),
                                TextColor::from(Color::BLACK),
                                Pickable::IGNORE
                            )]
                        )
                    ],
                )
            ],
        )
    }
}

#[derive(Debug, Component)]
struct SliderComponent<R: Resource> {
    lower: f32,
    upper: f32,
    lens: fn(&mut R) -> &mut f32,
}

#[derive(Debug, Resource)]
struct SystemMarker<R: Resource>(PhantomData<R>);

fn register_slider_system_once<R: Resource>(entity: &mut EntityWorldMut) {
    // SAFETY: We don't modify the entity's position so this is safe
    let world = unsafe { entity.world_mut() };

    // We just created the UI, marking it as changed ensures the update system runs
    world.get_resource_mut::<R>().unwrap().into_inner();

    if world.get_resource::<SystemMarker<R>>().is_some() {
        return;
    }

    world.insert_resource(SystemMarker::<R>(PhantomData::default()));
    world
        .get_resource_mut::<Schedules>()
        .unwrap()
        .get_mut(PostUpdate)
        .unwrap()
        .add_systems(update_slider_position::<R>.in_set(UiSystem::PostLayout));
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn update_slider_position<R: Resource>(
    mut resource: ResMut<R>,
    containers: Query<&ComputedNode>,
    mut node: Query<(&mut Node, &SliderComponent<R>, &ChildOf, &Children)>,
    mut text: Query<&mut Text>,
) {
    if !resource.is_changed() {
        return;
    }

    for (mut node, slider, child_of, children) in node.iter_mut() {
        let container = containers.get(child_of.0).unwrap();
        let size = container.size.x * container.inverse_scale_factor;

        let value = *(slider.lens)(&mut resource);

        node.left = Val::Px(remap(
            value,
            slider.lower,
            slider.upper,
            0.,
            size - SLIDER_SIZE,
        ));

        let mut text = text.get_mut(*children.get(0).unwrap()).unwrap();
        **text = format!(
            "{value:.0}",
            value = remap(value, slider.lower, slider.upper, 0., 10.)
        );
    }
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn drag<R: Resource>(
    trigger: Trigger<Pointer<Drag>>,
    mut params: ResMut<R>,
    containers: Query<&ComputedNode>,
    nodes: Query<(&SliderComponent<R>, &ChildOf)>,
) {
    let Ok((slider, child_of)) = nodes.get(trigger.target) else {
        return;
    };

    let container = containers.get(child_of.0).unwrap();
    let percentage_change =
        trigger.delta.x / (container.size.x * container.inverse_scale_factor - SLIDER_SIZE);

    let value = (slider.lens)(&mut params);

    *value = (*value + percentage_change * (slider.upper - slider.lower))
        .clamp(slider.lower, slider.upper);
}
