pub use bevy::prelude::*;

mod animation;
mod nanonaut;
mod robot;
mod bg;

pub struct AnimsPlugin;
impl Plugin for AnimsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut cmds: Commands| { cmds.spawn(Camera2d); })
            .add_systems(Startup, nanonaut::spawn_nanonaut)
            .add_systems(Startup, robot::spawn_robot)
            .add_systems(Update, animation::animate_sprites);
    }
}

pub struct BgPlugin;
impl Plugin for BgPlugin {
    fn build (&self, app: &mut App) {
        app.add_systems(Startup, bg::spawn_ground)
            .add_systems(Startup, bg::spawn_bg)
            .add_systems(Update, bg::scroll_bgs);
    }
}

// for faster iteration, from https://taintedcoders.com/bevy/windows
pub fn close_on_esc(
    mut commands: Commands,
    windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>
) {
    // Query to get window Entity + its Window component
    // only close on esc if focused
    for (window, properties) in windows {
        if properties.focused && input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}