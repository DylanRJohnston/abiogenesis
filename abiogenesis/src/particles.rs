use bevy::prelude::*;

#[cfg(feature = "hot_reload")]
use bevy_simple_subsecond_system::hot;

use crate::math::remap;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Model {
            weights: [[-0.5, -0.5, 0.5], [-0.5, -0.9, 0.6], [-0.6, -0.9, 0.8]],
        })
        .register_type::<SimulationParams>()
        .insert_resource(SimulationParams::DEFAULT)
        .add_systems(Startup, spawn_particles)
        .add_observer(respawn_particles)
        .add_systems(
            Update,
            (zero_forces, compute_forces, move_particles).chain(),
        );
    }
}

#[derive(Debug, Reflect, Component, Default, Clone, Copy)]
pub enum ParticleColour {
    #[default]
    Red,
    Green,
    Blue,
}

pub const RED: Color = Color::srgb_from_array([172.0 / 255.0, 40.0 / 255.0, 71.0 / 255.0]);
pub const GREEN: Color = Color::srgb_from_array([90.0 / 255.0, 181.0 / 255.0, 82.0 / 255.0]);
pub const BLUE: Color = Color::srgb_from_array([51.0 / 255.0, 136.0 / 255.0, 222.0 / 255.0]);

impl Into<Color> for ParticleColour {
    fn into(self) -> Color {
        match self {
            ParticleColour::Red => RED.into(),
            ParticleColour::Green => GREEN.into(),
            ParticleColour::Blue => BLUE.into(),
        }
    }
}

impl ParticleColour {
    fn index(&self) -> usize {
        match self {
            ParticleColour::Red => 0,
            ParticleColour::Green => 1,
            ParticleColour::Blue => 2,
        }
    }
}

#[derive(Debug, Reflect, Resource, Clone, Copy, Deref, DerefMut)]
pub struct Model {
    #[deref]
    pub weights: [[f32; 3]; 3],
}

#[derive(Debug, Reflect, Component, Default, Clone, Copy, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Debug, Reflect, Component, Default, Clone, Copy, Deref, DerefMut)]
struct Force(Vec2);

#[derive(Debug, Reflect, Component)]
#[require(Transform, ParticleColour, Velocity, Force)]
pub struct Particle;

#[derive(Debug, Reflect, Resource, Clone, Copy)]
#[reflect(Resource)]
pub struct SimulationParams {
    pub friction: f32,
    pub force_factor: f32,
    pub peak_attraction_radius: f32,
    pub repulsion_radius: f32,
    pub max_distance: f32,
}

impl SimulationParams {
    const DEFAULT: Self = Self {
        friction: 1.0,
        force_factor: 60.0,
        peak_attraction_radius: 2.0 * 150.0 / 3.0,
        repulsion_radius: 150.0 / 3.0,
        max_distance: 150.0,
    };
}

#[derive(Debug, Event, Clone, Copy, Reflect)]
pub struct Respawn;

#[cfg_attr(feature = "hot_reload", hot)]
fn spawn_particles(mut commands: Commands) {
    commands.trigger(Respawn);
}

fn respawn_particles(
    _trigger: Trigger<Respawn>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
    particles: Query<Entity, With<Particle>>,
) -> Result {
    particles
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());

    let window = windows.single()?;
    let (width, height) = (window.width(), window.height());

    let mesh = meshes.add(Circle::new(2.0));
    let red = materials.add(Color::from(RED));
    let green = materials.add(Color::from(GREEN));
    let blue = materials.add(Color::from(BLUE));

    (0..500).for_each(|_| {
        commands.spawn((
            Particle,
            Transform::from_xyz(
                rand::random::<f32>() * width - width / 2.0,
                rand::random::<f32>() * height - height / 2.0,
                0.0,
            ),
            ParticleColour::Red,
            Mesh2d(mesh.clone()),
            MeshMaterial2d(red.clone()),
        ));
    });

    (0..500).for_each(|_| {
        commands.spawn((
            Particle,
            Transform::from_xyz(
                rand::random::<f32>() * width - width / 2.0,
                rand::random::<f32>() * height - height / 2.0,
                0.0,
            ),
            ParticleColour::Green,
            Mesh2d(mesh.clone()),
            MeshMaterial2d(green.clone()),
        ));
    });

    (0..500).for_each(|_| {
        commands.spawn((
            Particle,
            Transform::from_xyz(
                rand::random::<f32>() * width - width / 2.0,
                rand::random::<f32>() * height - height / 2.0,
                0.0,
            ),
            ParticleColour::Blue,
            Mesh2d(mesh.clone()),
            MeshMaterial2d(blue.clone()),
        ));
    });

    Ok(())
}

fn toroidal_displacement(a: Vec2, b: Vec2, width: f32, height: f32) -> Vec2 {
    let mut dx = b.x - a.x;
    let mut dy = b.y - a.y;

    // Check if wrapping horizontally gives a shorter path
    if dx.abs() > width / 2.0 {
        dx = if dx > 0.0 { dx - width } else { dx + width };
    }

    // Check if wrapping vertically gives a shorter path
    if dy.abs() > height / 2.0 {
        dy = if dy > 0.0 { dy - height } else { dy + height };
    }

    Vec2::new(dx, dy)
}

fn toroidal_wrap(pos: Vec2, width: f32, height: f32) -> Vec2 {
    let mut pos = pos;

    if pos.x > width / 2.0 {
        pos.x -= width;
    } else if pos.x < -width / 2.0 {
        pos.x += width;
    }

    if pos.y > height / 2.0 {
        pos.y -= height;
    } else if pos.y < -height / 2.0 {
        pos.y += height;
    }

    pos
}

fn zero_forces(mut forces: Query<&mut Force>) {
    for mut force in forces.iter_mut() {
        **force = Vec2::ZERO;
    }
}

// #[cfg_attr(feature = "hot_reload", hot)]
fn compute_forces(
    mut particles: Query<(&Transform, &ParticleColour, &mut Force), With<Particle>>,
    model: Res<Model>,
    window: Query<&Window>,
    params: Res<SimulationParams>,
) -> Result<()> {
    let model = *model;
    let window = window.single()?;

    let mut iter = particles.iter_combinations_mut();

    while let Some(
        [
            (a_transform, a_color, mut a_force),
            (b_transform, b_color, mut b_force),
        ],
    ) = iter.fetch_next()
    {
        let displacement = toroidal_displacement(
            a_transform.translation.truncate(),
            b_transform.translation.truncate(),
            window.width(),
            window.height(),
        );

        let distance = displacement.length();
        let direction = displacement.normalize();

        if distance > params.max_distance {
            continue;
        }

        let a_influence = influence(
            &params,
            model.weights[a_color.index()][b_color.index()],
            distance,
        );

        let b_influence = influence(
            &params,
            model.weights[b_color.index()][a_color.index()],
            distance,
        );

        **a_force += direction * a_influence * params.force_factor;
        **b_force -= direction * b_influence * params.force_factor;
    }

    Ok(())
}

#[cfg_attr(feature = "hot_reload", hot)]
fn move_particles(
    mut particles: Query<(&mut Transform, &mut Velocity, &Force), With<Particle>>,
    time: Res<Time>,
    window: Query<&Window>,
    params: Res<SimulationParams>,
) -> Result<()> {
    let window = window.single()?;
    let width = window.width();
    let height = window.height();

    let dt = time.delta_secs();
    let friction_factor = (-params.friction * dt).exp();

    particles
        .iter_mut()
        .for_each(|(mut transform, mut velocity, force)| {
            let mut new_velocity = **velocity + force.0 * dt;
            new_velocity *= friction_factor;
            new_velocity = new_velocity.clamp_length(0.0, 200.0);

            let mut new_position =
                // transform.translation.truncate() + (new_velocity + previous_velocity) * 0.5 * dt;
                transform.translation.truncate() + new_velocity * dt;

            new_position = toroidal_wrap(new_position, width, height);

            **velocity = new_velocity;
            transform.translation = new_position.extend(0.0);
        });

    Ok(())
}

fn influence(params: &SimulationParams, factor: f32, distance: f32) -> f32 {
    if distance <= params.repulsion_radius {
        remap(distance, 0.0, params.repulsion_radius, -1.0, 0.0)
    } else if distance <= params.peak_attraction_radius {
        remap(
            distance,
            params.repulsion_radius,
            params.peak_attraction_radius,
            0.0,
            factor,
        )
    } else {
        remap(
            distance,
            params.peak_attraction_radius,
            params.max_distance,
            factor,
            0.0,
        )
    }
}

#[cfg(test)]
mod test {
    mod toroidal_displacement {

        use bevy::math::Vec2;

        use crate::particles::toroidal_displacement;

        #[test]
        fn zero() {
            assert_eq!(
                toroidal_displacement(Vec2::ZERO, Vec2::ZERO, 100.0, 100.0),
                Vec2::ZERO
            );
        }

        #[test]
        fn diagonal() {
            assert_eq!(
                toroidal_displacement(Vec2::ZERO, Vec2::splat(25.0), 100.0, 100.0),
                Vec2::splat(25.0)
            );
        }

        #[test]
        fn wrap_x() {
            assert_eq!(
                toroidal_displacement(Vec2::ZERO, Vec2::new(75.0, 0.0), 100.0, 100.0),
                Vec2::new(-25.0, 0.0)
            );
        }

        #[test]
        fn wrap_y() {
            assert_eq!(
                toroidal_displacement(Vec2::ZERO, Vec2::new(0.0, 75.0), 100.0, 100.0),
                Vec2::new(0.0, -25.0)
            );
        }

        #[test]
        fn wrap_xy() {
            assert_eq!(
                toroidal_displacement(Vec2::ZERO, Vec2::new(75.0, 75.0), 100.0, 100.0),
                Vec2::new(-25.0, -25.0)
            );
        }
    }

    mod toroidal_wrap {
        use bevy::math::Vec2;

        use crate::particles::toroidal_wrap;

        #[test]
        fn zero() {
            assert_eq!(toroidal_wrap(Vec2::ZERO, 100.0, 100.0), Vec2::ZERO);
        }

        #[test]
        fn within_bounds() {
            assert_eq!(
                toroidal_wrap(Vec2::new(25.0, 25.0), 100.0, 100.0),
                Vec2::new(25.0, 25.0)
            );
        }

        #[test]
        fn within_bounds_negative() {
            assert_eq!(
                toroidal_wrap(Vec2::new(-25.0, -25.0), 100.0, 100.0),
                Vec2::new(-25.0, -25.0)
            );
        }

        #[test]
        fn wrap_x() {
            assert_eq!(
                toroidal_wrap(Vec2::new(75.0, 0.0), 100.0, 100.0),
                Vec2::new(-25.0, 0.0)
            );
        }

        #[test]
        fn wrap_y() {
            assert_eq!(
                toroidal_wrap(Vec2::new(0.0, 75.0), 100.0, 100.0),
                Vec2::new(0.0, -25.0)
            );
        }

        #[test]
        fn wrap_xy() {
            assert_eq!(
                toroidal_wrap(Vec2::new(75.0, 75.0), 100.0, 100.0),
                Vec2::new(-25.0, -25.0)
            );
        }
    }

    mod influence {
        use crate::{
            math::lerp,
            particles::{SimulationParams, influence},
        };

        fn approx_eq(a: f32, b: f32) -> bool {
            (a - b).abs() < 0.00001
        }

        #[test]
        fn repulsed_self() {
            assert!(
                approx_eq(influence(&SimulationParams::DEFAULT, 1.0, 0.0), -1.0),
                "Expected influence to be -1.0, but got {}",
                influence(&SimulationParams::DEFAULT, 1.0, 0.0)
            );
        }

        #[test]
        fn repulsed_other() {
            assert!(
                approx_eq(
                    influence(
                        &SimulationParams::DEFAULT,
                        -1.0,
                        SimulationParams::DEFAULT.peak_attraction_radius
                    ),
                    -1.0
                ),
                "Expected influence to be -1.0, but got {}",
                influence(
                    &SimulationParams::DEFAULT,
                    -1.0,
                    SimulationParams::DEFAULT.peak_attraction_radius
                )
            );
        }

        #[test]
        fn balanced() {
            assert!(
                approx_eq(
                    influence(
                        &SimulationParams::DEFAULT,
                        1.0,
                        SimulationParams::DEFAULT.repulsion_radius
                    ),
                    0.0
                ),
                "Expected influence to be 0.0, but got {}",
                influence(
                    &SimulationParams::DEFAULT,
                    1.0,
                    SimulationParams::DEFAULT.repulsion_radius
                )
            );
        }

        #[test]
        fn attracted() {
            assert!(
                approx_eq(
                    influence(
                        &SimulationParams::DEFAULT,
                        1.0,
                        SimulationParams::DEFAULT.peak_attraction_radius
                    ),
                    1.0
                ),
                "Expected influence to be 1.0, but got {}",
                influence(
                    &SimulationParams::DEFAULT,
                    1.0,
                    SimulationParams::DEFAULT.peak_attraction_radius
                )
            );
        }

        #[test]
        fn halfway_repulsed() {
            assert!(
                approx_eq(
                    influence(
                        &SimulationParams::DEFAULT,
                        1.0,
                        SimulationParams::DEFAULT.repulsion_radius / 2.0
                    ),
                    -0.5
                ),
                "Expected influence to be -0.5, but got {}",
                influence(
                    &SimulationParams::DEFAULT,
                    1.0,
                    SimulationParams::DEFAULT.repulsion_radius / 2.0
                )
            );
        }

        #[test]
        fn halfway_attracted() {
            assert!(
                approx_eq(
                    influence(
                        &SimulationParams::DEFAULT,
                        1.0,
                        lerp(
                            SimulationParams::DEFAULT.repulsion_radius,
                            SimulationParams::DEFAULT.peak_attraction_radius,
                            0.5
                        )
                    ),
                    0.5
                ),
                "Expected influence to be 0.5, but got {}",
                influence(
                    &SimulationParams::DEFAULT,
                    1.0,
                    lerp(
                        SimulationParams::DEFAULT.repulsion_radius,
                        SimulationParams::DEFAULT.peak_attraction_radius,
                        0.5
                    )
                )
            );
        }

        #[test]
        fn halfway_attracted_otherside() {
            assert!(
                approx_eq(
                    influence(
                        &SimulationParams::DEFAULT,
                        1.0,
                        lerp(
                            SimulationParams::DEFAULT.peak_attraction_radius,
                            SimulationParams::DEFAULT.max_distance,
                            0.5
                        )
                    ),
                    0.5
                ),
                "Expected influence to be 0.5, but got {}",
                influence(
                    &SimulationParams::DEFAULT,
                    1.0,
                    lerp(
                        SimulationParams::DEFAULT.peak_attraction_radius,
                        SimulationParams::DEFAULT.max_distance,
                        0.5
                    )
                )
            );
        }
    }
}
