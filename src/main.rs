use bevy::{
    asset::AssetMetaCheck,
    window::WindowResolution
};
use bevy_nanonauts::*;

fn main() {
    let window_settings = WindowPlugin {
        primary_window: Some(Window {
            title: "bevy Nanonauts".into(),
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            //resizable: false,
            ..default()
        }),
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins
            .set(window_settings)
            // nearest sampling, to prevent white outlines on sprites
            .set(ImagePlugin::default_nearest())
            // for WASM, thinks images are meta files for some reason
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
        )
        .add_plugins(bevy_nanonauts::animations_plugin)
        .add_plugins(bevy_nanonauts::hud_plugin)
        .add_plugins(bevy_nanonauts::gameplay_plugin)
        .add_systems(Update, bevy_nanonauts::close_on_esc)
        .run();
}