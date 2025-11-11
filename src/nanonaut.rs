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
    let width = 181;
    let height = 229;

    let texture = assets.load("animatedNanonaut.png");
    let layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(
            UVec2 { x: width, y: height }, 
            5, 2, 
            None, None
        )
    );
    let anim_frames = animation::AnimationFrames { first: 0, last: 6 };

    // offset by -400 bc center is origin, put nanonaut on left quarter of window
    let nanonaut_x = -400.0 + crate::WINDOW_WIDTH*0.25;
    // put nanonaut on ground level, offset by 1/2 height bc centers are origins
    let nanonaut_y = crate::GROUND_LEVEL + ((height/2) as f32);

    commands.spawn((
        Nanonaut,
        Sprite::from_atlas_image(
            texture,
            TextureAtlas { layout, index: anim_frames.first }
        ),
        Transform::from_xyz(nanonaut_x, nanonaut_y, 1.0),
        anim_frames,
        animation::AnimationTimer(Timer::from_seconds(0.09, TimerMode::Repeating))
    ));
}

