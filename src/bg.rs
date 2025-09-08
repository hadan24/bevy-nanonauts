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
        Transform::from_xyz(0.0, -250.0, 0.0)
    ));
}