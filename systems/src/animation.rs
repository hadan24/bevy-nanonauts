// animation code from bevy/examples/2d/spritesheet.rs
use bevy::prelude::*;


#[derive(Component, Clone, Deref, DerefMut)]    // deref to easily call Timer fns on this
pub struct AnimationTimer(pub Timer);

#[derive(Component, Clone)]
pub struct AnimationFrames {
    first: usize,
    last: usize
}
impl AnimationFrames {
    pub fn new(first: usize, last: usize) -> Self {
        Self {first, last}
    }

    pub fn first(&self) -> usize {
        self.first
    }

    pub fn last(&self) -> usize {
        self.last
    }
}

#[derive(Bundle, Clone)]
pub struct AnimatedSprite {
    pub sprite: Sprite,
    pub frames: AnimationFrames,
    pub timer: AnimationTimer
}

pub fn animate_sprites(
    time: Res<Time>,
    mut sprites: Query<(&mut Sprite, &AnimationFrames, &mut AnimationTimer)>
) {
    for (mut sprite, indices, mut timer) in &mut sprites {
        timer.tick(time.delta());

        if timer.just_finished() && let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = if atlas.index == indices.last() {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}