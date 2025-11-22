// animation code from bevy/examples/2d/spritesheet.rs

use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]   // deref to easily call Timer fns on this
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationFrames {
    pub first: usize,
    pub last: usize
}

pub fn animate_sprites(
    time: Res<Time>,
    mut sprite: Query<(&AnimationFrames, &mut AnimationTimer, &mut Sprite)>
) {
    for (indices, mut timer, mut sprite) in &mut sprite {
        timer.tick(time.delta());

        if timer.just_finished() && let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}