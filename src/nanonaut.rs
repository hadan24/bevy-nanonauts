use bevy::{
    prelude::*,
    sprite::Anchor
};
use systems::animation;


const NANONAUT_WIDTH: f32 = 148.0;
const NANONAUT_HEIGHT: f32 = 200.0;
pub const MAX_HP: f32 = 100.0;
// puts nanonaut on ground level, offset by 1/2 height bc centers are origins
pub const NANONAUT_GROUND_LEVEL: f32 = crate::GROUND_LEVEL + (NANONAUT_HEIGHT / 2.0);

#[derive(Component)]
pub struct Nanonaut;

#[derive(Component)]
pub struct Hp(f32);
impl Hp {
    pub fn value(&self) -> f32 {
        self.0
    }
}

#[derive(Component)]
pub struct Velocity {
    linear: Vec2,
    rotation: f32   // rad per sec
}
#[derive(Bundle)]
struct KinematicsBundle {
    transform: Transform,
    velocity: Velocity
}

pub fn spawn_nanonaut(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let texture = assets.load("animatedNanonaut.png");
    let layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(
            UVec2::new(NANONAUT_WIDTH as u32, NANONAUT_HEIGHT as u32),
            5, 2, 
            None, None
        )
    );
    let frames = animation::AnimationFrames::new(0, 6);
    let animation_bundle = animation::AnimatedSprite {
        sprite: Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas { layout, index: frames.first() }),
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..default()
        },
        frames,
        timer: animation::AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating))
    };

    // put nanonaut on left quarter of window
    let nanonaut_x = -(crate::WINDOW_WIDTH as f32)/2.0 + (crate::WINDOW_WIDTH as f32)*0.25;

    commands.spawn((
        Nanonaut,
        Hp(MAX_HP),
        Observer::new(nanonaut_damage),
        animation_bundle,
        KinematicsBundle {
            transform: Transform::from_xyz(nanonaut_x, NANONAUT_GROUND_LEVEL + 300.0, 1.0)
                .with_scale(Vec3::new(NANONAUT_WIDTH, NANONAUT_HEIGHT, 1.0)),
            velocity: Velocity { linear: Vec2::ZERO, rotation: 0.1 }
        }
    ));
}
fn nanonaut_damage(
    _collided: On<crate::NanonautCollidedEvent>,
    mut hp: Single<&mut Hp, With<Nanonaut>>,
    mut score_reqs: ResMut<crate::ScoreRequirements>,
    mut game_mode: ResMut<crate::GameMode>
) {
    hp.0 -= 1.0;
    score_reqs.no_damage = false;
    if hp.0 <= 0.0 {
        *game_mode = game_mode.change();
    }
}


// physics values chosen using method outlined here + minor tweaks to refine feel
// https://youtu.be/hG9SzQxaCm8?si=zMId1NRJDpq9K1Dk
pub fn nanonaut_gravity(
    kinematics: Single<(&mut Transform, &mut Velocity), With<Nanonaut>>,
    score_reqs: Res<crate::ScoreRequirements>,
    mut score: ResMut<crate::Score>,
    time: Res<Time>
) {
    let (mut transform, mut vel) = kinematics.into_inner();

    if transform.translation.y > NANONAUT_GROUND_LEVEL {
        let g = if vel.linear.y > 0.0 { 
            -800.0
        } else {    // for a faster fall
            -1600.0
        };

        vel.linear.y += g * time.delta_secs();
        transform.translation.y += vel.linear.y * time.delta_secs();
    }
    else {
        if vel.linear.y < 0.0 && score_reqs.fully_met() {
            **score += 1;   // ResMut -> Score -> field
        }
        vel.linear.y = 0.0;
        //transform.translation.y = NANONAUT_GROUND_LEVEL;
    }
}

pub fn nanonaut_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    kinematics: Single<(&mut Transform, &mut Velocity), With<Nanonaut>>,
    mut score_reqs: ResMut<crate::ScoreRequirements>,
    time: Res<Time>
) {
    let (mut transform, mut vel) = kinematics.into_inner();
    let jump_spd = 500.0;

    if keyboard_input.pressed(KeyCode::Space) && transform.translation.y <= NANONAUT_GROUND_LEVEL {
        vel.linear.y = jump_spd;
        transform.translation.y += time.delta_secs() * vel.linear.y;

        // unsure why it must be reset here instead of in `else` block of `gravity`
        score_reqs.reset();
    }
}

pub fn nanonaut_death(
    nanonaut: Single<(Entity, &mut Transform, &mut Velocity), With<Nanonaut>>,
    mut commands: Commands,
    time: Res<Time>
) {
    let (id, mut transform, mut vel) = nanonaut.into_inner();

    // reset sprite to more easily rotate around bottom center
    commands.entity(id).insert(Anchor::BOTTOM_CENTER);
    transform.translation.y = crate::GROUND_LEVEL;

    let ds = time.delta_secs();
    let nanonaut_euler_z = transform.rotation.to_euler(EulerRot::XYZ).2;

    let accel = if nanonaut_euler_z > -std::f32::consts::FRAC_PI_6 {
        50.0
    } else {
        800.0
    };
    vel.rotation += accel * ds * ds;
    if nanonaut_euler_z > -std::f32::consts::FRAC_PI_2 {
        transform.rotate_z(-vel.rotation * ds);
    }
}