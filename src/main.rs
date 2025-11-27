use bevy::window::WindowResolution;
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
        )
        .add_plugins(bevy_nanonauts::CameraPlugin)
        .add_plugins(bevy_nanonauts::AnimsPlugin)
        .add_plugins(bevy_nanonauts::BgPlugin)
        .add_plugins(bevy_nanonauts::GameplayPlugin)
        .add_systems(Update, bevy_nanonauts::close_on_esc)
        .run();
}