use bevy::prelude::*;

use crate::{
    camera::FollowParticle,
    particles::{
        colour::ParticleColour,
        particle::{ParticleIndex, Velocity},
        simulation::SimulationParams,
        size::SimulationSize,
        spawner::{OldestParticle, ParticleAssets},
    },
};

const SCHEDULE_INTERVAL: f32 = 0.1;

pub struct DecayPlugin;
impl Plugin for DecayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_seconds(SCHEDULE_INTERVAL as f64))
            .add_systems(FixedUpdate, particle_decay);
    }
}

#[cfg_attr(feature = "hot_reload", bevy_simple_subsecond_system::hot)]
fn particle_decay(
    particle_indexes: Res<ParticleIndex>,
    simulation_size: SimulationSize,
    mut oldest_particle: ResMut<OldestParticle>,
    follow_particle: Option<Res<FollowParticle>>,
    params: Res<SimulationParams>,
    mut commands: Commands,
    particle_assets: Res<ParticleAssets>,
) -> Result<()> {
    let Vec2 {
        x: width,
        y: height,
    } = simulation_size.dimensions();

    let mut count = 0;
    while count < (params.decay_rate * SCHEDULE_INTERVAL) as i32 {
        let particle_index = particle_indexes.get(**oldest_particle);

        match particle_indexes.len() {
            0 => {
                **oldest_particle = 0;
                return Ok(());
            }
            num_particles => **oldest_particle = (**oldest_particle + 1) % num_particles,
        }

        let Some(&particle_index) = particle_index else {
            return Ok(());
        };

        if let Some(ref follow_particle) = follow_particle {
            if particle_index == ***follow_particle {
                continue;
            }
        }

        let colour = ParticleColour::random(params.num_colours);

        commands.entity(particle_index).insert((
            Transform::from_translation(
                Vec2::new(
                    rand::random::<f32>() * width - width / 2.0,
                    rand::random::<f32>() * height - height / 2.0,
                )
                .extend(0.0),
            ),
            Velocity::default(),
            colour,
            MeshMaterial2d(particle_assets.material(colour)),
        ));

        count += 1;
    }

    Ok(())
}
