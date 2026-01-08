use bevy::prelude::*;
use systems::animation;


const ROBOT_WIDTH: f32 = 142.0;
const ROBOT_HEIGHT: f32 = 141.0;
const ROBOT_GROUND_LEVEL: f32 = crate::GROUND_LEVEL + (ROBOT_HEIGHT / 2.0);

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
            UVec2::new(ROBOT_WIDTH as u32, ROBOT_HEIGHT as u32),
            3, 3,
            None, Some(UVec2::Y)
        )
    );
    let frames = animation::AnimationFrames::new(0, 7);
    let animation_bundle = animation::AnimatedSprite {
        sprite: Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas { layout, index: frames.first() }),
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..default()
        },
        frames,
        timer: animation::AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    };

    commands.spawn((
        Robot,
        animation_bundle,
        // z = 1.1 to be just above nanonaut
        Transform::from_xyz(crate::WINDOW_WIDTH as f32, ROBOT_GROUND_LEVEL, 1.1)
            .with_scale(Vec3::new(ROBOT_WIDTH, ROBOT_HEIGHT, 1.0))
    ));
}

pub fn move_robot(
    transforms: Query<&mut Transform, With<Robot>>,
    time: Res<Time>
) {
    let spd = 650.0;
    let screen_left = -((crate::WINDOW_WIDTH / 2) as f32);

    for mut t in transforms {
        t.translation.x = if t.translation.x < (screen_left - ROBOT_WIDTH) {
            t.translation.x + ((crate::WINDOW_WIDTH * 2) as f32)
        }
        else {
            t.translation.x - (time.delta_secs() * spd)
        };
    }
}