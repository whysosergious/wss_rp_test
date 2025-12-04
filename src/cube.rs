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
    time: Res<Time>,
) {
    if let Ok((mut transform, mut velocity)) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 0.0, -1.0);
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, 0.0, 1.0);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let acceleration = 20.0;
        velocity.vec += direction * acceleration * time.delta_seconds();

        let friction = 0.95;
        velocity.vec *= friction;

        transform.translation += velocity.vec * time.delta_seconds();
    }
}
