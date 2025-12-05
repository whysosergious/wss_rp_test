use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

#[derive(Component)]
pub struct CameraController {
    pub sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub distance: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub pan_speed: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            sensitivity: 0.1,
            zoom_sensitivity: 0.1,
            distance: 20.0, // Default distance for map overview
            min_distance: 5.0,
            max_distance: 100.0,
            pan_speed: 10.0,
        }
    }
}

#[derive(Component)]
pub struct CameraFocus;

pub fn camera_system(
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut CameraController), Without<CameraFocus>>,
    mut focus_query: Query<&mut Transform, With<CameraFocus>>,
    time: Res<Time>,
) {
    let (mut camera_transform, mut controller) = camera_query.single_mut();
    let mut focus_transform = focus_query.single_mut();

    // --- Rotation (Right Mouse Button) ---
    let mut rotation_move = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::ControlLeft) && input_mouse.pressed(MouseButton::Left) {
        for ev in ev_motion.read() {
            rotation_move += ev.delta;
        }
    } else {
        ev_motion.clear();
    }

    // --- Zoom (Mouse Wheel) ---
    let mut scroll = 0.0;
    for ev in ev_scroll.read() {
        scroll += ev.y;
    }

    // --- WASD Movement (Pan Focus Point) ---
    let mut direction = Vec3::ZERO;

    // Get camera's forward and right vectors, projected onto the XZ plane
    let mut camera_forward_flat = *camera_transform.forward();
    camera_forward_flat.y = 0.0;
    camera_forward_flat = camera_forward_flat.normalize_or_zero();

    let mut camera_right_flat = *camera_transform.right();
    camera_right_flat.y = 0.0;
    camera_right_flat = camera_right_flat.normalize_or_zero();

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction += camera_forward_flat;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction -= camera_right_flat;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction -= camera_forward_flat;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += camera_right_flat;
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    focus_transform.translation += direction * controller.pan_speed * time.delta_seconds();

    // --- Apply changes to Camera ---
    // Apply rotation
    if rotation_move.length_squared() > 0.0 {
        let delta_x = rotation_move.x * controller.sensitivity * 0.01;
        let delta_y = rotation_move.y * controller.sensitivity * 0.01;

        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);

        camera_transform.rotation = yaw * camera_transform.rotation * pitch;
    }

    // Apply zoom
    controller.distance -= scroll * controller.zoom_sensitivity;
    controller.distance = controller
        .distance
        .clamp(controller.min_distance, controller.max_distance);

    // Calculate new camera position relative to focus point
    let camera_offset = camera_transform.rotation * Vec3::new(0.0, 0.0, controller.distance);
    camera_transform.translation = focus_transform.translation + camera_offset; // Camera position is behind focus
}
