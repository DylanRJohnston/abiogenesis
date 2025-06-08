use bevy::prelude::*;
use bevy_tweening::*;

use crate::math::lerp;

pub struct LensPlugin;

impl Plugin for LensPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            component_animator_system::<TextColor>.in_set(AnimationSystem::AnimationUpdate),
        );
    }
}

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

pub struct TextColourLens {
    pub start: Color,
    pub end: Color,
}

impl Lens<TextColor> for TextColourLens {
    fn lerp(&mut self, target: &mut dyn Targetable<TextColor>, ratio: f32) {
        ***target = self.start.mix(&self.end, ratio);
    }
}
