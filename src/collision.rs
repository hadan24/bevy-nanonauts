use bevy::{
    prelude::*,
    math::bounding::{Aabb2d, IntersectsVolume}
};
use crate::{
    Dimensions,
    nanonaut::Nanonaut,
    robot::Robot
};


#[derive(Event)]
struct NanonautCollided;

pub fn dimensions_to_aabb(location: &Transform, dims: &Dimensions) -> Aabb2d {
    let location = location.translation;
    let dims = dims.as_vec2();
    Aabb2d {
        min: Vec2 { x: location.x - (dims.x / 2.0), y: location.y - (dims.y / 2.0) },
        max: Vec2 { x: location.x + (dims.x / 2.0), y: location.y + (dims.y / 2.0) }
    }
}

pub fn detect_collision(
    mut commands: Commands,
    nanonaut: Single<(&Transform, &Dimensions), With<Nanonaut>>,
    robots: Query<(&Transform, &Dimensions), With<Robot>>
) {
    let (location, dims) = nanonaut.into_inner();
    let nanonaut_box = dimensions_to_aabb(location, dims);
    
    for (location, dims) in &robots {
        let robot_box = dimensions_to_aabb(location, dims);

        if nanonaut_box.intersects(&robot_box) {
            println!("Collided");
            commands.trigger(NanonautCollided);
        }
    }
}