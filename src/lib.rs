pub use bevy::prelude::*;

mod animation;
mod camera;
mod collision;
mod nanonaut;
mod robot;
mod bg;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;
const GROUND_LEVEL: f32 = -250.0;   // center of "ground plane" rect, 100px tall

#[derive(Component, Deref, DerefMut)]
pub struct Dimensions(UVec2);

pub struct AnimsPlugin;
impl Plugin for AnimsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(camera::CameraPlugin)
            .add_systems(Startup, nanonaut::spawn_nanonaut)
            .add_systems(Startup, robot::spawn_robot)
            .add_systems(Update, robot::move_robot)
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

pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build (&self, app: &mut App) {
        let systems = (
            nanonaut::nanonaut_gravity,
            nanonaut::nanonaut_jump,
            collision::detect_collision
        ).chain();
        app.add_systems(FixedUpdate, systems);
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