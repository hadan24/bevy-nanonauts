pub use bevy::prelude::*;
use systems::{
    animation,
    camera,
    game_modes::GameMode
};

mod collision;
mod hud;
mod nanonaut;
mod robot;
mod bg;

mod resources;
use resources::*;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;
const GROUND_LEVEL: f32 = -250.0;   // center of "ground plane" rect, 100px tall
use collision::NanonautCollidedEvent; // re-export for easier use across mods


pub use hud::hud_plugin;

fn in_play_mode(mode: Res<GameMode>) -> bool {
    *mode == GameMode::Playing
}
pub fn animations_plugin(app: &mut App) {
    app.add_systems(Startup, (nanonaut::spawn_nanonaut, robot::spawn_robot))
        .add_systems(Update, (
            robot::move_robot,
            animation::animate_sprites
        ).run_if(in_play_mode))
        .add_plugins((bg::backgrounds_plugin, camera::camera_plugin::<NanonautCollidedEvent>));
}
pub fn gameplay_plugin(app: &mut App) {
    app.init_resource::<Score>()
        .init_resource::<ScoreRequirements>()
        .init_resource::<GameMode>()
        .add_systems(FixedUpdate, (
            nanonaut::nanonaut_gravity,
            nanonaut::nanonaut_jump,
            collision::detect_collisions,
            collision::over_robots
        ).chain().run_if(in_play_mode));
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