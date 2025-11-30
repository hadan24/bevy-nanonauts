use bevy::prelude::*;
use crate::{
    animation,
    collision::NanonautCollided
};

/*
- HP
*/
const NANONAUT_WIDTH: u32 = 148;
const NANONAUT_HEIGHT: u32 = 200;
pub const MAX_HP: f32 = 100.0;
// puts nanonaut on ground level, offset by 1/2 height bc centers are origins
const NANONAUT_GROUND_LEVEL: f32 = crate::GROUND_LEVEL + ((NANONAUT_HEIGHT/2) as f32);

#[derive(Component)]
pub struct Nanonaut;

#[derive(Component, Clone, Copy)]
pub struct Hp(f32);
impl Hp {
    pub fn value(self) -> f32 {
        self.0
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Bundle)]
pub struct KinematicsBundle {
    pub transform: Transform,
    pub velocity: Velocity
}

pub fn spawn_nanonaut(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let dims = crate::Dimensions(UVec2 { x: NANONAUT_WIDTH, y: NANONAUT_HEIGHT });
    let texture = assets.load("animatedNanonaut.png");
    let layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(
            dims.0,
            5, 2, 
            None, None
        )
    );
    let frames = animation::AnimationFrames::new(0, 6);
    let animation_bundle = animation::AnimatedSprite {
        sprite: Sprite::from_atlas_image(
            texture,
            TextureAtlas { layout, index: frames.first() }
        ),
        frames,
        timer: animation::AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating))
    };

    // put nanonaut on left quarter of window
    let nanonaut_x = -(crate::WINDOW_WIDTH as f32)/2.0 + (crate::WINDOW_WIDTH as f32)*0.25;

    commands.spawn((
        Nanonaut,
        dims,
        Hp(MAX_HP),
        Observer::new(nanonaut_damage),
        animation_bundle,
        KinematicsBundle {
            transform: Transform::from_xyz(nanonaut_x, NANONAUT_GROUND_LEVEL + 300.0, 1.0),
            velocity: Velocity(Vec2::ZERO),
        }
    ));
}


// physics values chosen using method outlined here + minor tweaks to refine feel
// https://youtu.be/hG9SzQxaCm8?si=zMId1NRJDpq9K1Dk
pub fn nanonaut_gravity(
    kinematics: Single<(&mut Transform, &mut Velocity), With<Nanonaut>>,
    time: Res<Time>
) {
    let (mut transform, mut vel) = kinematics.into_inner();

    if transform.translation.y > NANONAUT_GROUND_LEVEL {
        let g = if vel.y > 0.0 { 
            -800.0
        } else {    // for a faster fall
            -1600.0
        };

        vel.y += g * time.delta_secs();
        transform.translation.y += vel.y * time.delta_secs();
    }
    else {
        vel.y = 0.0;
        transform.translation.y = NANONAUT_GROUND_LEVEL;
    }
}

pub fn nanonaut_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    kinematics: Single<(&mut Transform, &mut Velocity), With<Nanonaut>>,
    time: Res<Time>
) {
    let (mut transform, mut vel) = kinematics.into_inner();
    let jump_spd = 500.0;

    if keyboard_input.pressed(KeyCode::Space) && transform.translation.y <= NANONAUT_GROUND_LEVEL {
        vel.y = jump_spd;
        transform.translation.y += time.delta_secs() * vel.y;
    }
}

pub fn nanonaut_damage(
    _collided: On<NanonautCollided>,
    mut hp: Single<&mut Hp, With<Nanonaut>>
) {
    hp.0 -= 1.0;
}