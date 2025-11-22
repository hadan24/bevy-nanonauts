use bevy::prelude::*;
use crate::animation;

/*
- collider
*/
const ROBOT_WIDTH: u32 = 142;
const ROBOT_HEIGHT: u32 = 141;
const ROBOT_GROUND_LEVEL: f32 = crate::GROUND_LEVEL + ((ROBOT_HEIGHT/2) as f32);

#[derive(Component)]
pub struct Robot;

pub fn spawn_robot(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let texture = assets.load("animatedRobot.png");
    let layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(
            UVec2 { x: ROBOT_WIDTH, y: ROBOT_HEIGHT },
            3, 3,
            None, Some(UVec2::Y)
        )
    );
    let anim_frames = animation::AnimationFrames { first: 0, last: 7 };

    commands.spawn((
        Robot,
        Sprite::from_atlas_image(
            texture.clone(),
            TextureAtlas { layout, index: anim_frames.first }
        ),
        anim_frames,
        animation::AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Transform::from_xyz(crate::WINDOW_WIDTH as f32, ROBOT_GROUND_LEVEL, 1.0),
    ));
}

pub fn move_robot(
    transforms: Query<&mut Transform, With<Robot>>,
    time: Res<Time>
) {
    let spd = 650.0;
    let screen_left = -((crate::WINDOW_WIDTH / 2) as f32);

    for mut t in transforms {
        t.translation.x = if t.translation.x < (screen_left - ROBOT_WIDTH as f32) {
            t.translation.x + ((crate::WINDOW_WIDTH * 2) as f32)
        }
        else {
            t.translation.x - (time.delta_secs() * spd)
        };
    }
}