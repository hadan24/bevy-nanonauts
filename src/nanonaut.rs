use bevy::prelude::*;
use crate::animation;

/*
- jump
- controller
- collider
- HP
*/

#[derive(Component)]
struct Nanonaut;

pub fn spawn_nanonaut(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    commands.spawn(Camera2d);

    let texture = assets.load("animatedNanonaut.png");
    let layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(
            UVec2 { x: 181, y: 229 }, 
            5, 2, 
            None, None
        )
    );
    let anim_frames = animation::AnimationFrames { first: 0, last: 6 };

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas { layout, index: anim_frames.first }
        ),
        Transform::from_scale(Vec3::splat(1.0)),
        anim_frames,
        animation::AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    ));
}

