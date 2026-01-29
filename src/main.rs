use bevy::{
    asset::AssetMetaCheck,
    window::{WindowResized, WindowResolution}
};
use bevy_nanonauts::*;

fn main() {
    let window_settings = WindowPlugin {
        primary_window: Some(Window {
            title: "bevy Nanonauts".into(),
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            resize_constraints: WindowResizeConstraints { 
                min_width: WINDOW_WIDTH as f32 / 4.0,
                min_height: WINDOW_HEIGHT as f32 / 4.0,
                max_width: WINDOW_WIDTH as f32,
                max_height: WINDOW_HEIGHT as f32
            },
            ..default()
        }),
        ..default()
    };

    let mut app = App::new();
    #[cfg(debug_assertions)]    // debug-only systems
    {
        app.add_systems(Update, bevy_nanonauts::close_on_esc);
    }
    app.add_plugins(DefaultPlugins
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
        .add_systems(Update, |mut resize_reader: MessageReader<WindowResized>, mut window: Single<&mut Window>| {
            for e in resize_reader.read() {
                window.resolution.set_scale_factor_override(Some(e.width / WINDOW_WIDTH as f32));
            }
        })
        .run();
}