use bevy::prelude::*;
use crate::animation;

/*
- jump
- controller
- collider
- HP
*/
const NANONAUT_WIDTH: u32 = 181;
const NANONAUT_HEIGHT: u32 = 229;
// puts nanonaut on ground level, offset by 1/2 height bc centers are origins
const NANONAUT_GROUND_LEVEL: f32 = crate::GROUND_LEVEL + ((NANONAUT_HEIGHT/2) as f32);

#[derive(Component)]
struct Nanonaut;

pub fn spawn_nanonaut(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let texture = assets.load("animatedNanonaut.png");
    let layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(
            UVec2 { x: NANONAUT_WIDTH, y: NANONAUT_HEIGHT }, 
            5, 2, 
            None, None
        )
    );
    let anim_frames = animation::AnimationFrames { first: 0, last: 6 };

    // offset by -400 bc center is origin, put nanonaut on left quarter of window
    let nanonaut_x = -400.0 + (crate::WINDOW_WIDTH as f32)*0.25;

    commands.spawn((
        Nanonaut,
        Sprite::from_atlas_image(
            texture,
            TextureAtlas { layout, index: anim_frames.first }
        ),
        Transform::from_xyz(nanonaut_x, NANONAUT_GROUND_LEVEL, 1.0),
        anim_frames,
        animation::AnimationTimer(Timer::from_seconds(0.09, TimerMode::Repeating))
    ));
}

