use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod camera;
mod game;
mod game_state_ui; // New: Game State UI module
mod map;
mod ui;
use camera::{CameraController, CameraFocus, camera_system};
use game::{assign_initial_territories, spawn_army_text_ui, update_army_text_ui};
use game_state_ui::{spawn_turn_indicator, update_turn_indicator}; // New: Import turn indicator systems
use map::{setup_map, MapEntity}; // Removed Tile import
use ui::{GameSettings, setup_ui};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    #[default]
    SetupState,
    GameState,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<GameSettings>()
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::SetupState), setup_map) // Initial map setup on entering SetupState
        .add_systems(OnEnter(AppState::GameState), (cleanup_game_state, setup_map, assign_initial_territories, spawn_army_text_ui, spawn_turn_indicator).chain()) // Composed systems for game transition, including spawning army text and turn indicator
        .add_systems(Update, camera_system.run_if(in_state(AppState::GameState))) // Camera system only in GameState
        .add_systems(Update, handle_escape_key.run_if(in_state(AppState::GameState)))
        .add_systems(Update, setup_ui.run_if(in_state(AppState::SetupState)))
        .add_systems(Update, update_army_text_ui.run_if(in_state(AppState::GameState))) // Update army text position
        .add_systems(Update, update_turn_indicator.run_if(in_state(AppState::GameState))) // New: Update turn indicator
        .add_systems(OnExit(AppState::GameState), cleanup_game_state) // This cleanup runs when exiting GameState
        .run();
}

fn setup(
    mut commands: Commands,
    _meshes: Res<Assets<Mesh>>,
    _materials: Res<Assets<StandardMaterial>>,
) {
    // Camera Focus
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
        CameraFocus,
    ));

    // 3D Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 20.0, 0.0), // Camera initial position
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
}

// New system to handle escape key press
fn handle_escape_key(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        info!("Returning to SetupState.");
        next_state.set(AppState::SetupState);
    }
}

// New system to clean up game state entities
fn cleanup_game_state(
    mut commands: Commands,
    map_entity_query: Query<Entity, With<MapEntity>>, // Query for all entities marked with MapEntity
) {
    info!("Cleaning up GameState entities.");
    for entity in map_entity_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}



