use bevy::{
    prelude::*,
    math::bounding::{Aabb2d, IntersectsVolume},
};
use crate::{
    nanonaut::Nanonaut,
    robot::Robot
};


#[derive(Event)]
pub struct NanonautCollidedEvent;   // Collision(Entity, Entity)

fn aabb_from_transform(transform: &Transform) -> Aabb2d {
    let location = transform.translation.truncate();
    let scale = transform.scale.truncate() / 2.0;

    Aabb2d {
        min: location - scale,
        max: location + scale
    }
}

// AABB dimensions tweaked w/ trial+error
pub fn detect_collisions(
    mut commands: Commands,
    nanonaut: Single<&Transform, With<Nanonaut>>,
    robots: Query<&Transform, With<Robot>>
) {
    let raw = aabb_from_transform(*nanonaut);
    let nanonaut_box = Aabb2d {
        min: raw.min + Vec2::new(25.0, 5.0),
        max: raw.max + Vec2::new(-40.0, -10.0)
    };
    
    for r_transform in &robots {
        let raw = aabb_from_transform(r_transform);
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
    nanonaut: Single<&Transform, With<Nanonaut>>,
    robots: Query<&Transform, With<Robot>>,
    mut score_reqs: ResMut<crate::ScoreRequirements>
) {
    if nanonaut.translation.y <= crate::nanonaut::NANONAUT_GROUND_LEVEL {
        return;
    }

    let raw = aabb_from_transform(*nanonaut);
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