use bevy::prelude::*;

use crate::{
    camera::FollowParticle,
    particles::{
        simulation::{Particle, Velocity},
        size::SimulationSize,
        spawner::ParticleIndexes,
    },
};

const DECAY_PER_SECOND: f32 = 100.0;
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
    mut particles: Query<(&mut Transform, &mut Velocity), With<Particle>>,
    particle_indexes: Res<ParticleIndexes>,
    simulation_size: SimulationSize,
    mut index: Local<usize>,
    follow_particle: Option<Res<FollowParticle>>,
) -> Result<()> {
    let Vec2 {
        x: width,
        y: height,
    } = simulation_size.dimensions();

    let mut count = 0;
    while count < (DECAY_PER_SECOND * SCHEDULE_INTERVAL) as i32 {
        let particle_index = particle_indexes.get(*index);

        match particle_indexes.len() {
            0 => {
                *index = 0;
                return Ok(());
            }
            other => *index = (*index + 1) % other,
        }

        let Some(particle_index) = particle_index else {
            return Ok(());
        };

        if let Some(ref follow_particle) = follow_particle {
            if *particle_index == ***follow_particle {
                continue;
            }
        }

        let (mut transform, mut velocity) = particles.get_mut(*particle_index)?;
        transform.translation = Vec2::new(
            rand::random::<f32>() * width - width / 2.0,
            rand::random::<f32>() * height - height / 2.0,
        )
        .extend(0.0);

        **velocity = Vec2::ZERO;

        count += 1;
    }

    Ok(())
}
