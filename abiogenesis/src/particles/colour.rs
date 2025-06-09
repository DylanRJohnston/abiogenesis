use bevy::prelude::*;
use rand::Rng;

#[derive(Debug, Reflect, Component, Default, Clone, Copy, PartialEq, Eq)]
pub enum ParticleColour {
    #[default]
    Red,
    Green,
    Blue,
    Orange,
    Pink,
    Aqua,
}

pub const NUM_COLOURS: usize = 6;

pub const RED: Color = Color::srgb_from_array([172.0 / 255.0, 40.0 / 255.0, 71.0 / 255.0]);
pub const GREEN: Color = Color::srgb_from_array([90.0 / 255.0, 181.0 / 255.0, 82.0 / 255.0]);
pub const BLUE: Color = Color::srgb_from_array([51.0 / 255.0, 136.0 / 255.0, 222.0 / 255.0]);
pub const ORANGE: Color = Color::srgb(255.0 / 255.0, 155.0 / 255.0, 37.0 / 255.0);
pub const PINK: Color = Color::srgb_from_array([233.0 / 255.0, 75.0 / 255.0, 234.0 / 255.0]);
pub const AQUA: Color = Color::srgb(57.0 / 255.0, 247.0 / 255.0, 241.0 / 255.0);

impl From<ParticleColour> for Color {
    fn from(value: ParticleColour) -> Self {
        match value {
            ParticleColour::Red => RED,
            ParticleColour::Green => GREEN,
            ParticleColour::Blue => BLUE,
            ParticleColour::Orange => ORANGE,
            ParticleColour::Pink => PINK,
            ParticleColour::Aqua => AQUA,
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
            ParticleColour::Pink => write!(f, "Pink"),
            ParticleColour::Aqua => write!(f, "Aqua"),
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
            ParticleColour::Pink => 4,
            ParticleColour::Aqua => 5,
        }
    }

    pub fn random(particle_variety: usize) -> Self {
        match rand::thread_rng().gen_range(1..=particle_variety) {
            1 => ParticleColour::Red,
            2 => ParticleColour::Green,
            3 => ParticleColour::Blue,
            4 => ParticleColour::Orange,
            5 => ParticleColour::Pink,
            6 => ParticleColour::Aqua,
            _ => unreachable!(),
        }
    }
}
