use bevy::prelude::*;

/*
- spd
*/

#[derive(Component)]
struct Ground;

pub fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let gnd_rect = meshes.add(Rectangle::new(800.0, 100.0));
    let gnd_color = materials.add(Color::srgb_u8(34, 139, 34));

    commands.spawn((
        Mesh2d(gnd_rect),
        MeshMaterial2d(gnd_color),
        Transform::from_xyz(0.0, -250.0, 0.1)
    ));
}


#[derive(Component, Default)]
pub struct Background;

#[derive(Bundle, Default)]
struct BgBundle {
    tag: Background,
    img: Sprite,
    transform: Transform
}
pub fn spawn_bg(
    mut commands: Commands,
    assets: ResMut<AssetServer>
) {
    let bgs = [
        BgBundle {
            img: Sprite::from(assets.load("background.png")),
            transform: Transform::from_translation(Vec3::Y * 75.0)
                .with_scale(Vec3::splat(0.8)),
            ..Default::default()
        },
        BgBundle {
            img: Sprite::from(assets.load("background.png")),
            transform: Transform::from_translation(Vec3 {x: 800.0, y: 75.0, z: 0.0})
                .with_scale(Vec3::splat(0.8)),
            ..Default::default()
        }
    ];
    commands.spawn_batch(bgs);
}

pub fn scroll_bgs(
    mut bgs: Query<&mut Transform, With<Background>>,
) {
    for mut bg in &mut bgs {
        bg.translation.x = if bg.translation.x < -800.0 {
            800.0-4.0
        } else {
            bg.translation.x - 2.0
        };
    }
}