use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_tweening::TweeningPlugin;
use particles::ParticlePlugin;
use ui::UIPlugin;

#[cfg(feature = "hot_reload")]
use bevy_simple_subsecond_system::SimpleSubsecondPlugin;

mod math;
mod observe;
mod particles;
mod ui;

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    RecordInput,
    Update,
}

const CLEAR_COLOUR: Color = Color::srgb_from_array([44.0 / 255.0, 30.0 / 255.0, 49.0 / 255.0]);

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "Abiogenesis".into(),
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
    )
    .add_plugins(TweeningPlugin)
    .insert_resource(ClearColor(CLEAR_COLOUR))
    .configure_sets(
        Update,
        (AppSystems::RecordInput, AppSystems::Update).chain(),
    )
    .add_plugins((ParticlePlugin, UIPlugin))
    .add_systems(Startup, spawn_camera);

    #[cfg(feature = "hot_reload")]
    app.add_plugins(SimpleSubsecondPlugin::default());

    app.run()
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::from("Camera"),
        Camera2d,
        Camera {
            // hdr: true,
            ..default()
        },
        // Tonemapping::AcesFitted,
        // Bloom::default(),
        // DebandDither::Enabled,
    ));
}
