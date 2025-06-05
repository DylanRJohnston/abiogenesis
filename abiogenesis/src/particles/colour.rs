use bevy::prelude::*;

#[derive(Debug, Reflect, Component, Default, Clone, Copy, PartialEq, Eq)]
pub enum ParticleColour {
    #[default]
    Red,
    Green,
    Blue,
    Orange,
}

pub const RED: Color = Color::srgb_from_array([172.0 / 255.0, 40.0 / 255.0, 71.0 / 255.0]);
pub const GREEN: Color = Color::srgb_from_array([90.0 / 255.0, 181.0 / 255.0, 82.0 / 255.0]);
pub const BLUE: Color = Color::srgb_from_array([51.0 / 255.0, 136.0 / 255.0, 222.0 / 255.0]);
// pub const ORANGE: Color = Color::srgb_from_array([233.0 / 255.0, 133.0 / 255.0, 55.0 / 255.0]);
pub const ORANGE: Color = Color::srgb_from_array([233.0 / 255.0, 133.0 / 255.0, 55.0 / 255.0]);

impl Into<Color> for ParticleColour {
    fn into(self) -> Color {
        match self {
            ParticleColour::Red => RED.into(),
            ParticleColour::Green => GREEN.into(),
            ParticleColour::Blue => BLUE.into(),
            ParticleColour::Orange => ORANGE.into(),
        }
    }
}

impl std::fmt::Display for ParticleColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParticleColour::Red => write!(f, "Red"),
            ParticleColour::Green => write!(f, "Green"),
            ParticleColour::Blue => write!(f, "Blue"),
            ParticleColour::Orange => write!(f, "Orange"),
        }
    }
}

impl ParticleColour {
    pub fn index(&self) -> usize {
        match self {
            ParticleColour::Red => 0,
            ParticleColour::Green => 1,
            ParticleColour::Blue => 2,
            ParticleColour::Orange => 3,
        }
    }
}
