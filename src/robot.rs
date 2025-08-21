use bevy::prelude::*;
use crate::animation;

/*
- anim spd
- spd
- collider
*/

#[derive(Component)]
struct Robot;

pub fn spawn_robot(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let texture = assets.load("animatedRobot.png");
    let layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(
            UVec2 { x: 142, y: 141 },
            3, 3,
            None, Some(UVec2::Y)
        )
    );
    let anim_frames = animation::AnimationFrames { first: 0, last: 8 };

    commands.spawn((
        Robot,
        Sprite::from_atlas_image(
            texture.clone(),
            TextureAtlas { layout, index: anim_frames.first }
        ),
        Transform::from_translation(Vec3::X * 150.0),
        anim_frames,
        animation::AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    ));
}

