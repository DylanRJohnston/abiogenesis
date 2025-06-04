use bevy::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state::<Scene>(Scene::Sandbox);
    }
}

#[derive(Debug, Clone, Copy, Reflect, States, PartialEq, Eq, Hash)]
pub enum Scene {
    Sandbox,
}
