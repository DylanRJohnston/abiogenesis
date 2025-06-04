use std::cell::RefCell;

use bevy::prelude::*;
use itertools::Itertools;
use rand_distr::{Distribution, Normal};

use crate::particles::{colour::*, simulation::Particle, size::SimulationSize};

pub const PARTICLES_PER_COLOR: usize = 1000;

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ParticleIndexes(Vec::with_capacity(4 * PARTICLES_PER_COLOR)))
            .add_systems(Startup, spawn_particles_on_startup)
            .add_observer(respawn_particles);
    }
}

#[derive(Debug, Event, Clone, Copy, Reflect)]
pub struct Respawn;

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn spawn_particles_on_startup(mut commands: Commands) {
    commands.trigger(Respawn);
}

#[derive(Debug, Reflect, Resource, Deref, DerefMut)]
pub struct ParticleIndexes(Vec<Entity>);

fn respawn_particles(
    _trigger: Trigger<Respawn>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut particle_indexes: ResMut<ParticleIndexes>,
    simulation_size: SimulationSize,
    particles: Query<Entity, With<Particle>>,
) -> Result {
    particles
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());

    let mesh = meshes.add(Circle::new(1.0));
    let red = materials.add(Color::from(RED));
    let green = materials.add(Color::from(GREEN));
    let blue = materials.add(Color::from(BLUE));
    let orange = materials.add(Color::from(ORANGE));

    let material = |color: ParticleColour| match color {
        ParticleColour::Red => red.clone(),
        ParticleColour::Green => green.clone(),
        ParticleColour::Blue => blue.clone(),
        ParticleColour::Orange => orange.clone(),
    };

    let commands = RefCell::new(commands);

    let Vec2 {
        x: width,
        y: height,
    } = simulation_size.dimensions();

    particle_indexes.clear();

    type ParticleIter<'a> = Box<dyn Iterator<Item = Entity> + 'a>;
    let empty = || Box::new([].into_iter()) as ParticleIter<'_>;

    let normal = Normal::<f32>::new(0.0, 1.0).unwrap();

    let transform = move || {
        let mut rng = rand::thread_rng();

        Transform::from_xyz(
            width * (normal.sample(&mut rng) - 0.5),
            height * (normal.sample(&mut rng) - 0.5),
            0.0,
        )
    };

    [
        ParticleColour::Red,
        ParticleColour::Green,
        ParticleColour::Blue,
    ]
    .iter()
    .map(|color| {
        (0..PARTICLES_PER_COLOR).map(|_| {
            commands
                .borrow_mut()
                .spawn((
                    Particle,
                    transform(),
                    *color,
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(material(*color)),
                ))
                .id()
        })
    })
    .map(|iter| Box::new(iter) as ParticleIter<'_>)
    .fold(empty(), |a, b| Box::new(a.interleave(b)))
    .collect_into(&mut **particle_indexes);

    Ok(())
}
