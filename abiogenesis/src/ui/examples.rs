use bevy::{ecs::spawn::SpawnIter, prelude::*};

use crate::{
    observe::observe,
    particles::{
        model::{Model, PRESETS},
        simulation::SimulationParams,
    },
    ui::{
        dropdown::{ToggleState, dropdown},
        mixins,
    },
};

const NUM_PRESETS: usize = PRESETS.len();
const VERTICAL_PADDING: f32 = 4.0;
const FONT_SIZE: f32 = 24.0;
const TOTAL_HEIGHT: f32 =
    (NUM_PRESETS as f32) * FONT_SIZE + 2.0 * VERTICAL_PADDING * NUM_PRESETS as f32;

pub fn examples() -> impl Bundle {
    dropdown("Examples", "Example Presets", TOTAL_HEIGHT, contents())
}

const DROPDOWN_PADDING: f32 = 8.0;

fn contents() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            width: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(DROPDOWN_PADDING)),
            ..default()
        },
        Children::spawn(SpawnIter(PRESETS.iter().map(|(name, model, params)| {
            (
                Node {
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(VERTICAL_PADDING)),
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    ..default()
                },
                BorderRadius::all(Val::Px(4.0)),
                mixins::hover_colour(Color::NONE, Color::WHITE.with_alpha(0.1)),
                children![(Text::from(*name), Pickable::IGNORE)],
                observe(
                    |mut trigger: Trigger<Pointer<Click>>,
                     mut commands: Commands,
                     mut res_model: ResMut<Model>,
                     mut res_params: ResMut<SimulationParams>| {
                        trigger.propagate(false);

                        // This will bubble up to the dropdown, which will close itself
                        commands.trigger_targets(ToggleState, trigger.target());

                        *res_model = *model;
                        *res_params = *params;
                    },
                ),
            )
        }))),
    )
}
