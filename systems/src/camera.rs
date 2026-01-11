// screen shake code based on bevy/examples/camera/2d_screen_shake.rs
use bevy::prelude::*;
use noise::{Perlin, NoiseFn};


pub fn camera_plugin<T: Event>(app: &mut App) {
    app.add_systems(Startup, setup_camera::<T>)
        .add_systems(PreUpdate, reset_camera)
    /*
        Transform = local to entity, is relative to parent frame of reference
        GlobalTransform = managed by Bevy, the FINAL transform for used rendering
        TransformSystems::Propagate = system set in PostUpdate
            that sends this entity's transform updates to childrens' GlobalTransforms
    */
    // apply shake before propagating changes to ensure that
    // camera's global transform + its children render w/ proper transforms
        .add_systems(PostUpdate, shake_camera.before(TransformSystems::Propagate));
}

#[derive(Component, Default)]
struct CameraShakeState {
    trauma: f32,
    original_position: Transform
}
#[derive(Component)]
#[require(CameraShakeState)]
struct CameraShakeConfig {
    trauma_decay_per_second: f32,
    max_translation: f32,
    noise_speed: f32,
}

fn setup_camera<T: Event>(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Observer::new(increase_trauma::<T>),
        CameraShakeConfig {
            trauma_decay_per_second: 0.5,
            max_translation: 20.0,
            noise_speed: 20.0
        }
    ));
}

fn increase_trauma<T: Event>(
    _collided: On<T>,
    mut camera_state: Single<&mut CameraShakeState>
) {
    camera_state.trauma += 0.4;
    camera_state.trauma = camera_state.trauma.clamp(0.0, 0.1);
}

fn reset_camera(camera_state: Single<(&mut Transform, &CameraShakeState)>) {
    let (mut transform, shake_state) = camera_state.into_inner();
    *transform = shake_state.original_position;
}

fn shake_camera(
    camera: Single<(&mut Transform, &mut CameraShakeState, &CameraShakeConfig)>,
    time: Res<Time>
) {
    let (mut transform, mut state, config) = camera.into_inner();

    // to restore camera position after each frame, allowing rendering to ignore shaking
    state.original_position = *transform;
    
    // apply shake
    let multiplier = 240.0 * config.max_translation * state.trauma * state.trauma;
    let rng = Perlin::new(3);
    let t = time.elapsed_secs_f64() * (config.noise_speed as f64);
    let shake_x = (rng.get([t + 100.0]) as f32) * multiplier;
    let shake_y = (rng.get([t + 200.0]) as f32) * multiplier;
    transform.translation += Vec3::new(shake_x, shake_y, 0.0);

    // gradually phase out shakes
    state.trauma -= config.trauma_decay_per_second * time.delta_secs();
    state.trauma = state.trauma.clamp(0.0, 1.0);
}
