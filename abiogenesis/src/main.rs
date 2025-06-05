#![feature(iter_collect_into)]
#![feature(coroutines)]
#![feature(gen_blocks)]

use bevy::{
    asset::{AssetMetaCheck, load_internal_binary_asset},
    prelude::*,
};
use bevy_tweening::TweeningPlugin;
use particles::ParticlePlugin;
use ui::UIPlugin;

use crate::{camera::CameraPlugin, controls::ControlsPlugin, scenes::ScenePlugin};

mod camera;
mod controls;
mod math;
mod observe;
mod particles;
mod scenes;
mod spatial_hash;
mod ui;

const CLEAR_COLOUR: Color = Color::srgb_from_array([44.0 / 255.0, 30.0 / 255.0, 49.0 / 255.0]);

fn main() -> AppExit {
    let mut app = App::new();

    // Bevy Plugins;
    bevy_systems(&mut app);
    third_party_systems(&mut app);
    app_systems(&mut app);

    app.run()
}

fn bevy_systems(app: &mut App) {
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "Abiogenesis".into(),
                    // #[cfg(not(target_arch = "wasm32"))]
                    // resolution: WindowResolution::new(1920.0, 1080.0),
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
    )
    .insert_resource(ClearColor(CLEAR_COLOUR));
}

fn third_party_systems(app: &mut App) {
    app.add_plugins((
        TweeningPlugin,
        #[cfg(feature = "egui")]
        (
            bevy_inspector_egui::bevy_egui::EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
        ),
    ));

    #[cfg(feature = "hot_reload")]
    app.add_plugins(bevy_simple_subsecond_system::SimpleSubsecondPlugin::default());
}

fn app_systems(app: &mut App) {
    app.add_plugins((
        ParticlePlugin,
        UIPlugin,
        ScenePlugin,
        CameraPlugin,
        ControlsPlugin,
    ));

    load_internal_binary_asset!(
        app,
        TextFont::default().font,
        "../assets/fonts/CreatoDisplay-Regular.otf",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );
}
