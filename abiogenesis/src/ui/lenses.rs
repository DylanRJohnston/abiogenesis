use bevy::prelude::*;
use bevy_tweening::*;

use crate::math::lerp;

pub struct LeftLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<Node> for LeftLens {
    fn lerp(&mut self, target: &mut dyn bevy_tweening::Targetable<Node>, ratio: f32) {
        target.left = Val::Px(lerp(self.start, self.end, ratio));
    }
}
pub struct HeightLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<Node> for HeightLens {
    fn lerp(&mut self, target: &mut dyn bevy_tweening::Targetable<Node>, ratio: f32) {
        target.height = Val::Px(lerp(self.start, self.end, ratio));
    }
}

pub struct BottomLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<Node> for BottomLens {
    fn lerp(&mut self, target: &mut dyn bevy_tweening::Targetable<Node>, ratio: f32) {
        target.bottom = Val::Px(lerp(self.start, self.end, ratio));
    }
}

pub struct TopLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<Node> for TopLens {
    fn lerp(&mut self, target: &mut dyn bevy_tweening::Targetable<Node>, ratio: f32) {
        target.top = Val::Px(lerp(self.start, self.end, ratio));
    }
}
