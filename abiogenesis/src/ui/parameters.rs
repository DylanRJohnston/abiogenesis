use bevy::prelude::*;

use crate::{
    particles::simulation::{
        ATTRACTION_RADIUS_RANGE, DECAY_RATE_RANGE, FORCE_STRENGTH_RANGE, FRICTION_RANGE,
        PEAK_ATTRACTION_RADIUS_RANGE, REPULSION_RADIUS_RANGE, SimulationParams,
    },
    ui::{
        dropdown::dropdown,
        icon::Icon,
        model_matrix::{MODEL_MATRIX_SIZE, model_matrix},
        slider::{self, Slider},
    },
};

const NUM_SLIDERS: f32 = 6.0;
const ROW_GAP: f32 = 8.0;
const HEIGHT: f32 =
    NUM_SLIDERS * slider::COMPONENT_SIZE + ROW_GAP * (NUM_SLIDERS + 1.0) + MODEL_MATRIX_SIZE;

pub fn parameters() -> impl Bundle {
    (dropdown(
        Icon("icons/balance.png"),
        "Laws of Creation",
        HEIGHT,
        content(),
    ),)
}

fn content() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            row_gap: Val::Px(8.0),
            column_gap: Val::Px(8.0),
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        children![
            model_matrix(),
            Slider {
                name: "Friction",
                range: FRICTION_RANGE,
                lens: |resource: &mut SimulationParams| { &mut resource.friction },
            }
            .into_bundle(),
            Slider {
                name: "Force",
                range: FORCE_STRENGTH_RANGE,
                lens: |resource: &mut SimulationParams| { &mut resource.force_strength },
            }
            .into_bundle(),
            Slider {
                name: "Attraction Radius",
                range: ATTRACTION_RADIUS_RANGE,
                lens: |resource: &mut SimulationParams| { &mut resource.attraction_radius },
            }
            .into_bundle(),
            Slider {
                name: "Peak Attraction Radius",
                range: PEAK_ATTRACTION_RADIUS_RANGE,
                lens: |resource: &mut SimulationParams| { &mut resource.peak_attraction_radius },
            }
            .into_bundle(),
            Slider {
                name: "Repulsion Radius",
                range: REPULSION_RADIUS_RANGE,
                lens: |resource: &mut SimulationParams| { &mut resource.repulsion_radius },
            }
            .into_bundle(),
            Slider {
                name: "Entropy",
                range: DECAY_RATE_RANGE,
                lens: |resource: &mut SimulationParams| { &mut resource.decay_rate },
            }
            .into_bundle(),
        ],
    )
}
