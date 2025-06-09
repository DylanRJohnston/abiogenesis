use bevy::prelude::*;
use rand_distr::uniform;

use crate::particles::{
    colour::*,
    particle::{MAX_PARTICLES, Particle, ParticleIndex, Velocity},
    simulation::SimulationParams,
    size::SimulationSize,
};

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnParticle>()
            .insert_resource(SpawnerConfig::Uniform)
            .insert_resource(OldestParticle::default())
            .add_systems(Startup, (init_assets, spawn_particles_on_startup).chain())
            .add_systems(Update, spawn_particle)
            .add_systems(Update, update_colours_on_num_change)
            .add_observer(respawn_particles);
    }
}

#[derive(Debug, Event, Clone)]
pub struct Respawn;

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
pub fn spawn_particles_on_startup(mut commands: Commands) {
    commands.trigger(Respawn);
}

#[derive(Debug, Resource)]
pub struct ParticleAssets {
    mesh: Handle<Mesh>,
    red: Handle<ColorMaterial>,
    green: Handle<ColorMaterial>,
    blue: Handle<ColorMaterial>,
    orange: Handle<ColorMaterial>,
    pink: Handle<ColorMaterial>,
    aqua: Handle<ColorMaterial>,
}

impl ParticleAssets {
    pub fn material(&self, color: ParticleColour) -> Handle<ColorMaterial> {
        match color {
            ParticleColour::Red => self.red.clone(),
            ParticleColour::Green => self.green.clone(),
            ParticleColour::Blue => self.blue.clone(),
            ParticleColour::Orange => self.orange.clone(),
            ParticleColour::Pink => self.pink.clone(),
            ParticleColour::Aqua => self.aqua.clone(),
        }
    }
}

fn init_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle::new(1.0));
    let red = materials.add(Color::from(RED));
    let green = materials.add(Color::from(GREEN));
    let blue = materials.add(Color::from(BLUE));
    let orange = materials.add(Color::from(ORANGE));
    let pink = materials.add(Color::from(PINK));
    let aqua = materials.add(Color::from(AQUA));

    commands.insert_resource(ParticleAssets {
        mesh,
        red,
        green,
        blue,
        orange,
        pink,
        aqua,
    });
}

#[derive(Debug, Clone, Resource, Default, PartialEq)]
pub enum SpawnerConfig {
    None,
    #[default]
    Uniform,
    Custom(Vec<(ParticleColour, SpawnShape)>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpawnShape {
    Rect(Rect),
    Circle {
        position: Vec2,
        radius: f32,
    },
    HollowCircle {
        position: Vec2,
        inner_radius: f32,
        outer_radius: f32,
    },
}

impl SpawnShape {
    pub fn transform(&self) -> Transform {
        match self {
            SpawnShape::Rect(rect) => Transform::from_xyz(
                rect.min.x + (rect.max.x - rect.min.x) * rand::random::<f32>(),
                rect.min.y + (rect.max.y - rect.min.y) * rand::random::<f32>(),
                0.0,
            ),
            SpawnShape::Circle { position, radius } => {
                let angle = 2.0 * std::f32::consts::PI * rand::random::<f32>();
                let x = position.x + radius * angle.cos();
                let y = position.y + radius * angle.sin();

                Transform::from_xyz(x, y, 0.0)
            }
            SpawnShape::HollowCircle {
                position,
                inner_radius,
                outer_radius,
            } => {
                let angle = 2.0 * std::f32::consts::PI * rand::random::<f32>();
                let radius = inner_radius + (outer_radius - inner_radius) * rand::random::<f32>();

                let x = position.x + radius * angle.cos();
                let y = position.y + radius * angle.sin();

                Transform::from_xyz(x, y, 0.0)
            }
        }
    }
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn respawn_particles(
    _trigger: Trigger<Respawn>,
    mut commands: Commands,
    mut particle_indexes: ResMut<ParticleIndex>,
    simulation_size: SimulationSize,
    particles: Query<Entity, With<Particle>>,
    particle_assets: Res<ParticleAssets>,
    mut params: ResMut<SimulationParams>,
    spawner_config: Res<SpawnerConfig>,
) -> Result<()> {
    params.decay_rate = 80.0;

    particles
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());

    let Vec2 {
        x: width,
        y: height,
    } = simulation_size.dimensions();

    particle_indexes.clear();

    let transform = move |color: ParticleColour| match &*spawner_config {
        SpawnerConfig::None | SpawnerConfig::Uniform => Transform::from_xyz(
            width * (rand::random::<f32>() - 0.5),
            height * (rand::random::<f32>() - 0.5),
            0.0,
        ),
        SpawnerConfig::Custom(items) => {
            for (inner_colour, shape) in items {
                if color == *inner_colour {
                    return shape.transform();
                }
            }

            Transform::from_xyz(
                width * (rand::random::<f32>() - 0.5),
                height * (rand::random::<f32>() - 0.5),
                0.0,
            )
        }
    };

    (0..MAX_PARTICLES).for_each(|i| {
        let color = match i % params.num_colours {
            0 => ParticleColour::Red,
            1 => ParticleColour::Green,
            2 => ParticleColour::Blue,
            3 => ParticleColour::Orange,
            4 => ParticleColour::Pink,
            5 => ParticleColour::Aqua,
            _ => unreachable!(),
        };

        commands.spawn((
            Particle,
            transform(color),
            color,
            Mesh2d(particle_assets.mesh.clone()),
        ));
    });

    Ok(())
}

#[derive(Debug, Resource, Deref, DerefMut, Default)]
pub struct OldestParticle(usize);

#[derive(Debug, Event, Clone, Copy)]
pub struct SpawnParticle {
    pub position: Vec2,
    pub colour: ParticleColour,
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn spawn_particle(
    particle_assets: Res<ParticleAssets>,
    mut commands: Commands,
    mut spawn_particles: EventReader<SpawnParticle>,
    particle_index: Res<ParticleIndex>,
    mut oldest_particle: ResMut<OldestParticle>,
) -> Result<()> {
    for SpawnParticle {
        position,
        colour: color,
    } in spawn_particles.read()
    {
        if particle_index.len() >= MAX_PARTICLES {
            commands.entity(particle_index[**oldest_particle]).insert((
                Transform::from_translation(position.extend(0.0)),
                Velocity::default(),
                *color,
            ));

            **oldest_particle = (**oldest_particle + 1) % particle_index.len();
        } else {
            commands.spawn((
                Particle,
                Transform::from_translation(position.extend(0.0)),
                *color,
                Mesh2d(particle_assets.mesh.clone()),
            ));
        }
    }

    Ok(())
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn update_colours_on_num_change(
    params: Res<SimulationParams>,
    mut prev_num: Local<usize>,
    particles: Query<Entity, With<Particle>>,
    mut commands: Commands,
) {
    if !params.is_changed() {
        return;
    }

    if params.num_colours == *prev_num {
        return;
    }

    *prev_num = params.num_colours;

    commands.insert_batch(
        particles
            .iter()
            .map(|particle| (particle, ParticleColour::random(params.num_colours)))
            // insert_batch requires Send + Send + 'static, so we can't hold onto the particles query
            .collect::<Vec<_>>(),
    );
}
