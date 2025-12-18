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
pub struct NanonautCollidedEvent;   // Collision(Entity, Entity)

fn dimensions_to_aabb(location: &Transform, dims: &Dimensions) -> Aabb2d {
    let location = location.translation.truncate();
    let dims = dims.as_vec2() / 2.0;

    Aabb2d {
        min: location - dims,
        max: location + dims
    }
}

// AABB dimensions tweaked w/ trial+error
pub fn detect_collisions(
    mut commands: Commands,
    nanonaut: Single<(&Transform, &Dimensions), With<Nanonaut>>,
    robots: Query<(&Transform, &Dimensions), With<Robot>>
) {
    let raw = dimensions_to_aabb(nanonaut.0, nanonaut.1);
    let nanonaut_box = Aabb2d {
        min: raw.min + Vec2::new(25.0, 5.0),
        max: raw.max + Vec2::new(-40.0, -10.0)
    };
    
    for (location, dims) in &robots {
        let raw = dimensions_to_aabb(location, dims);
        let robot_box = Aabb2d {
            min: raw.min + Vec2::new(15.0, 5.0),
            max: raw.max + Vec2::new(-25.0, -15.0)
        };

        if nanonaut_box.intersects(&robot_box) {
            commands.trigger(NanonautCollidedEvent);
        }
    }
}

pub fn over_robots(
    nanonaut: Single<(&Transform, &Dimensions), With<Nanonaut>>,
    robots: Query<&Transform, With<Robot>>,
    mut score_reqs: ResMut<crate::ScoreRequirements>
) {
    if nanonaut.0.translation.y <= crate::nanonaut::NANONAUT_GROUND_LEVEL {
        return;
    }

    let raw = dimensions_to_aabb(nanonaut.0, nanonaut.1);
    let nanonaut_box = Aabb2d {
        min: raw.min + Vec2::new(25.0, 5.0),
        max: raw.max + Vec2::new(-40.0, -10.0)
    };
    for r in robots {
        let closest = nanonaut_box.closest_point(r.translation.truncate());
        if closest.x == nanonaut_box.min.x {
            score_reqs.over_robot = true;
        }
    }
}