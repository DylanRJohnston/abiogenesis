use bevy::prelude::*;

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Model {
            weights: [[0.3, 0.4, 0.5], [0.7, -0.4, 0.3], [-0.5, 0.5, 0.0]],
        })
        .add_observer(randomise_model);
    }
}

#[derive(Debug, Reflect, Resource, Clone, Copy, Deref, DerefMut)]
pub struct Model {
    #[deref]
    pub weights: [[f32; 3]; 3],
}

#[derive(Debug, Event, Clone, Copy, Reflect)]
pub struct Randomise;

fn randomise_model(_: Trigger<Randomise>, mut model: ResMut<Model>) {
    model.weights.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|value| {
            *value = rand::random::<f32>() * 2.0 - 1.0;
        })
    });
}
