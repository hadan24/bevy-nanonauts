pub use bevy::prelude::*;

pub mod animation;
pub mod nanonaut;

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

//141 139