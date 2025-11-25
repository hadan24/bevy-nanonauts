use bevy::{
    prelude::*,
    math::bounding::{Aabb2d, IntersectsVolume},
};
use crate::{
    Dimensions,
    nanonaut::Nanonaut,
    robot::Robot
};


#[derive(Event)]
pub struct NanonautCollided;

fn dimensions_to_aabb(location: &Transform, dims: &Dimensions) -> Aabb2d {
    let location = location.translation.truncate();
    let dims = dims.as_vec2() / 2.0;

    Aabb2d {
        min: location - dims,
        max: location + dims
    }
}

// AABB dimensions tweaked w/ trial+error
pub fn detect_collision(
    mut commands: Commands,
    nanonaut: Single<(&Transform, &Dimensions), With<Nanonaut>>,
    robots: Query<(&Transform, &Dimensions), With<Robot>>
) {
    let (location, dims) = nanonaut.into_inner();
    let raw = dimensions_to_aabb(location, dims);
    let nanonaut_box = Aabb2d {
        min: raw.min + Vec2 {x: 25.0, y: 5.0},
        max: raw.max + Vec2 {x: -40.0, y: -10.0}
    };
    
    for (location, dims) in &robots {
        let raw = dimensions_to_aabb(location, dims);
        let robot_box = Aabb2d {
            min: raw.min + Vec2 {x: 15.0, y: 5.0},
            max: raw.max + Vec2 {x: -25.0, y: -15.0}
        };

        if nanonaut_box.intersects(&robot_box) {
            println!("Collided");
            commands.trigger(NanonautCollided);
        }
    }
}