use bevy::prelude::*;

mod camera;
mod cube;
mod floor;
use camera::{CameraController, camera_system};
use cube::{Controllable, Velocity, cube_system};
use floor::setup_floor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, setup_floor))
        .add_systems(Update, (camera_system, cube_system))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(3.0, 5.0, -8.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController::default(),
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, -4.0),
        ..default()
    });

    // Controllable Cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.7)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Controllable,
        Velocity::default(),
    ));
}
