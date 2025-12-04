use bevy::prelude::*;

#[derive(Component)]
pub struct Controllable;

#[derive(Component, Default)]
pub struct Velocity {
    pub vec: Vec3,
}

pub fn cube_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Controllable>>,
    camera_query: Query<&Transform, (With<Camera>, Without<Controllable>)>, // Query for the camera's transform, explicitly without Controllable
    time: Res<Time>,
) {
    if let (Ok((mut transform, mut velocity)), Ok(camera_transform)) = (query.get_single_mut(), camera_query.get_single()) {
        let mut direction = Vec3::ZERO;

        // Get camera's forward and right vectors, projected onto the XZ plane
        let mut camera_forward_flat = *camera_transform.forward();
        camera_forward_flat.y = 0.0;
        let camera_forward_flat_vec = camera_forward_flat.normalize_or_zero();

        let mut camera_right_flat = *camera_transform.right();
        camera_right_flat.y = 0.0;
        let camera_right_flat_vec = camera_right_flat.normalize_or_zero();

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += camera_forward_flat_vec;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= camera_right_flat_vec;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= camera_forward_flat_vec;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += camera_right_flat_vec;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        let acceleration = 20.0;
        velocity.vec += direction * acceleration * time.delta_seconds();

        let friction = 0.95;
        velocity.vec *= friction;

        transform.translation += velocity.vec * time.delta_seconds();
    }
}
