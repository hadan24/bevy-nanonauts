use bevy_nanonauts::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, || {println!("hello world!")})
        .add_systems(Startup, nanonaut::spawn_nanonaut)
        .add_systems(Update, animation::animate_sprites)
        .add_systems(Update, close_on_esc)
        .run();
}