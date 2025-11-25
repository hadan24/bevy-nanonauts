// screen shake code based on bevy/examples/camera/2d_screen_shake.rs

use bevy::prelude::*;
use crate::collision::NanonautCollided;

pub fn setup_camera(
    mut commands: Commands
) {
    commands.spawn((
        Camera2d,
        Observer::new(screen_shake)
    ));

}

fn screen_shake(
    _collided: On<NanonautCollided>,
    camera: Single<&mut Transform, With<Camera2d>>,
    _time: Res<Time>
) {
    println!("{:?}", camera.translation)
}