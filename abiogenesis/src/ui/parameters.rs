use bevy::prelude::*;

use crate::{
    particles::simulation::SimulationParams,
    ui::{
        dropdown::dropdown,
        slider::{self, Slider},
    },
};

const NUM_SLIDERS: f32 = 6.0;
const ROW_GAP: f32 = 8.0;
const HEIGHT: f32 = NUM_SLIDERS * slider::COMPONENT_SIZE + ROW_GAP * (NUM_SLIDERS);

pub fn parameters() -> impl Bundle {
    (dropdown(
        "Parameters",
        "Advanced Model Parameters",
        HEIGHT,
        content(),
    ),)
}

fn content() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::End,
            row_gap: Val::Px(8.0),
            column_gap: Val::Px(8.0),
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        children![
            Slider {
                name: "Friction",
                lower: 0.0,
                upper: 5.0,
                lens: |resource: &mut SimulationParams| { &mut resource.friction },
            }
            .into_bundle(),
            Slider {
                name: "Force",
                lower: 0.0,
                upper: 200.0,
                lens: |resource: &mut SimulationParams| { &mut resource.force_strength },
            }
            .into_bundle(),
            Slider {
                name: "Attraction Radius",
                lower: 0.0,
                upper: 200.0,
                lens: |resource: &mut SimulationParams| { &mut resource.attraction_radius },
            }
            .into_bundle(),
            Slider {
                name: "Peak Attraction Radius",
                lower: 0.0,
                upper: 200.0,
                lens: |resource: &mut SimulationParams| { &mut resource.peak_attraction_radius },
            }
            .into_bundle(),
            Slider {
                name: "Repulsion Radius",
                lower: 0.0,
                upper: 200.0,
                lens: |resource: &mut SimulationParams| { &mut resource.repulsion_radius },
            }
            .into_bundle(),
            Slider {
                name: "Decay Rate",
                lower: 0.0,
                upper: 200.0,
                lens: |resource: &mut SimulationParams| { &mut resource.decay_rate },
            }
            .into_bundle(),
        ],
    )
}
