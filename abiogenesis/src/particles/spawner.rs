use bevy::prelude::*;

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
            .insert_resource(OldestParticle::default())
            .add_systems(Startup, (init_assets, spawn_particles_on_startup).chain())
            .add_systems(Update, spawn_particle)
            .add_observer(respawn_particles);
    }
}

#[derive(Debug, Event, Clone, Copy, Reflect)]
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

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn respawn_particles(
    _trigger: Trigger<Respawn>,
    mut commands: Commands,
    mut particle_indexes: ResMut<ParticleIndex>,
    simulation_size: SimulationSize,
    particles: Query<Entity, With<Particle>>,
    particle_assets: Res<ParticleAssets>,
    mut params: ResMut<SimulationParams>,
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

    let transform = move || {
        Transform::from_xyz(
            width * (rand::random::<f32>() - 0.5),
            height * (rand::random::<f32>() - 0.5),
            0.0,
        )
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
            transform(),
            color,
            Mesh2d(particle_assets.mesh.clone()),
            MeshMaterial2d(particle_assets.material(color)),
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
                MeshMaterial2d(particle_assets.material(*color)),
            ));

            **oldest_particle = (**oldest_particle + 1) % particle_index.len();
        } else {
            commands.spawn((
                Particle,
                Transform::from_translation(position.extend(0.0)),
                *color,
                Mesh2d(particle_assets.mesh.clone()),
                MeshMaterial2d(particle_assets.material(*color)),
            ));
        }
    }

    Ok(())
}
