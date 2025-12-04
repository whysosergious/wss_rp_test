use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

use crate::cube::Controllable;

#[derive(Component)]
pub struct CameraController {
    pub sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub distance: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            sensitivity: 0.1,
            zoom_sensitivity: 0.1,
            distance: 8.0,
        }
    }
}

pub fn camera_system(
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<(&mut Transform, &mut CameraController), Without<Controllable>>,
    controllable_query: Query<&Transform, With<Controllable>>,
) {
    if let Ok(controllable_transform) = controllable_query.get_single() {
        let mut rotation_move = Vec2::ZERO;
        if input_mouse.pressed(MouseButton::Left) {
            for ev in ev_motion.read() {
                rotation_move += ev.delta;
            }
        } else {
            ev_motion.clear();
        }

        let mut scroll = 0.0;
        for ev in ev_scroll.read() {
            scroll += ev.y;
        }

        for (mut transform, mut controller) in camera_query.iter_mut() {
            if rotation_move.length_squared() > 0.0 {
                let delta_x = rotation_move.x * controller.sensitivity * 0.01;
                let delta_y = rotation_move.y * controller.sensitivity * 0.01;
                let yaw = Quat::from_rotation_y(-delta_x);
                let pitch = Quat::from_rotation_x(-delta_y);
                transform.rotation = yaw * transform.rotation * pitch;
            }

            if scroll.abs() > 0.0 {
                controller.distance -= scroll * controller.zoom_sensitivity;
                controller.distance = controller.distance.clamp(1.0, 100.0);
            }

            // Follow the controllable
            let translation_offset = transform.rotation * Vec3::new(0.0, 0.0, controller.distance);
            transform.translation = controllable_transform.translation + translation_offset;
            transform.look_at(controllable_transform.translation, Vec3::Y);
        }
    }
}
