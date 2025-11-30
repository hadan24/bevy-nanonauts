pub use bevy::prelude::*;

mod animation;
mod camera;
mod collision;
mod hud;
mod nanonaut;
mod robot;
mod bg;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;
const GROUND_LEVEL: f32 = -250.0;   // center of "ground plane" rect, 100px tall
pub(crate) use collision::NanonautCollidedEvent; // re-export for easier use across mods

#[derive(Component, Deref, DerefMut)]
pub struct Dimensions(UVec2);
#[derive(Resource, Default)]
struct Score(u32);

pub use hud::hud_plugin;

pub fn animations_plugin(app: &mut App) {
    app.add_systems(Startup, nanonaut::spawn_nanonaut)
        .add_systems(Startup, robot::spawn_robot)
        .add_systems(Update, robot::move_robot)
        .add_systems(Update, animation::animate_sprites)
        .add_plugins((bg::backgrounds_plugin, camera::camera_plugin));
}
pub fn gameplay_plugin(app: &mut App) {
    app.init_resource::<Score>()
        .add_systems(FixedUpdate, (
            nanonaut::nanonaut_gravity,
            nanonaut::nanonaut_jump,
            collision::detect_collisions
        ).chain());
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