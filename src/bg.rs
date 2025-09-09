use bevy::prelude::*;

#[derive(Component)]
struct Ground;

pub fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let gnd_rect = meshes.add(Rectangle::new(crate::WINDOW_WIDTH, 100.0));
    // forest green #228b22
    let gnd_color = materials.add(Color::srgb_u8(34, 139, 34));

    commands.spawn((
        Mesh2d(gnd_rect),
        MeshMaterial2d(gnd_color),
        Transform::from_xyz(0.0, -250.0, 0.1)
    ));
}


#[derive(Component, Default)]
pub struct Background;

#[derive(Component)]
pub struct BgDimensions {
    width: f32,
    //height: f32,  // currently not needed
    scale: f32
}
impl Default for BgDimensions {
    fn default() -> Self {
        BgDimensions {width: 1.0, scale: 1.0 }  //height: 1.0,
    }
}

#[derive(Bundle, Default)]
struct BgBundle {
    tag: Background,
    img: Sprite,
    transform: Transform,
    dims: BgDimensions
}
pub fn spawn_bg(
    mut commands: Commands,
    assets: ResMut<AssetServer>
) {
    let width = 1000.0;
    //let height = 752.0;
    let y_offset = 75.0;
    let scale = 0.8;

    let bgs = [
        BgBundle {
            img: Sprite::from(assets.load("background.png")),
            transform: Transform::from_translation(Vec3::Y * y_offset)
                .with_scale(Vec3::splat(scale)),
            dims: BgDimensions { width, scale },
            ..Default::default()
        },
        BgBundle {
            img: Sprite {
                image: assets.load("background.png"),
                flip_x: true,   // to hide seam between imgs bc edge colors don't match
                ..Default::default()
            },
            transform: Transform
                ::from_translation(Vec3 {x: width*scale, y: y_offset, z: 0.0})
                .with_scale(Vec3::splat(scale)),
            dims: BgDimensions { width, scale },
            ..Default::default()
        }
    ];
    commands.spawn_batch(bgs);
}

pub fn scroll_bgs(
    mut bgs: Query<(&mut Transform, &BgDimensions), With<Background>>,
) {
    let scroll_spd = 2.0;
    let alignment_offset = scroll_spd * 2.0;

    for (mut bg, dims) in &mut bgs {
        let real_width = dims.width * dims.scale;

        bg.translation.x = if bg.translation.x < -real_width {
            real_width - alignment_offset
        }
        else {
            bg.translation.x - scroll_spd
        };
    }
}