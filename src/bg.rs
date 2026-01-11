use bevy::prelude::*;


pub fn backgrounds_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ground)
        .add_systems(Startup, spawn_bg)
        .add_systems(Update, scroll_bgs.run_if(crate::in_play_mode));
}

pub fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    // larger ground rect + offset keeps nanonaut at same place while also
    // preventing camera from showing "out of bounds" when shaking
    let gnd_offset = 25.0;
    let gnd_rect = meshes.add(Rectangle::new(
        (crate::WINDOW_WIDTH as f32) * 2.0,
        100.0 + (gnd_offset * 2.0)
    ));
    // forest green #228b22
    let gnd_color = materials.add(Color::srgb_u8(34, 139, 34));

    commands.spawn((
        Mesh2d(gnd_rect),
        MeshMaterial2d(gnd_color),
        Transform::from_xyz(0.0, crate::GROUND_LEVEL - gnd_offset, 0.1)
    ));
}


#[derive(Component, Copy, Clone)]
pub struct Background;
#[derive(Component, Copy, Clone)]
pub struct BgDimensions {
    width: f32,
    //height: f32,  // currently not needed
    scale: f32
}
impl Default for BgDimensions {
    fn default() -> Self {
        BgDimensions { width: 1.0, scale: 1.0 }  //height: 1.0,
    }
}

const SCREEN_LEFT: f32 = -(crate::WINDOW_WIDTH as f32) / 2.0;
#[derive(Bundle, Clone)]
struct BgBundle {
    tag: Background,
    sprite: Sprite,
    transform: Transform,
    dims: BgDimensions
}
pub fn spawn_bg(
    mut commands: Commands,
    assets: ResMut<AssetServer>
) {
    //let height = 752.0;
    // unsure why this specific num works, but it puts trees just above ground (100px from bottom)
    let y_offset = 72.0;
    let dims = BgDimensions { width: 1000.0, scale: 0.8 };
    let real_width = dims.width * dims.scale;
    let start_x = SCREEN_LEFT + (real_width / 2.0);
    let bg_img = assets.load("background.png");

    let first_bg = BgBundle {
        tag: Background,
        sprite: Sprite::from(bg_img.clone()),
        transform: Transform
            ::from_translation(Vec3::new(start_x, y_offset, 0.0))
            .with_scale(Vec3::splat(dims.scale)),
        dims
    };
    let flipped_bg = BgBundle {
        sprite: Sprite {
            image: bg_img.clone(),
            flip_x: true,   // to hide seam between imgs bc edge colors don't match
            ..default()
        },
        transform: Transform
            ::from_translation(Vec3::new(start_x + real_width, y_offset, 0.0))
            .with_scale(Vec3::splat(dims.scale)),
        ..first_bg
    };

    let bgs = [
        BgBundle {
            sprite: first_bg.sprite.clone(),
            transform: Transform
                ::from_translation(Vec3::new(start_x + (real_width * 2.0), y_offset, 0.0))
                .with_scale(Vec3::splat(dims.scale)),
            ..first_bg
        },
        BgBundle {
            sprite: flipped_bg.sprite.clone(),
            transform: Transform
                ::from_translation(Vec3::new(start_x + (real_width * 3.0), y_offset, 0.0))
                .with_scale(Vec3::splat(dims.scale)),
            ..flipped_bg
        },
        first_bg,   // don't move first_bg till after copying/cloning all we need
        flipped_bg
    ];
    commands.spawn_batch(bgs);
}

pub fn scroll_bgs(
    mut bgs: Query<(&mut Transform, &BgDimensions)>,
    time: Res<Time>
) {
    let scroll_spd = 350.0;
    let num_imgs = bgs.count() as f32;

    for (mut bg, dims) in &mut bgs {
        let real_width = dims.width * dims.scale;
        let right_edge = bg.translation.x + (real_width / 2.0);

        // -100 to ENSURE furthest left img is off-screen (avoids empty space during shakes)
        if right_edge < (SCREEN_LEFT - 100.0) {
            bg.translation.x += real_width * num_imgs;
        }
        bg.translation.x -= time.delta_secs()*scroll_spd;
    }
}