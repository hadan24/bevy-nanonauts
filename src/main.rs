use bevy_nanonauts::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, || {println!("hello world!")})
        .add_systems(Update, close_on_esc)
        .run();
}