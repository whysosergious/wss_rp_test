use bevy::prelude::*;

use crate::game::Faction;
use crate::ui::GameSettings;

// Component for hexagonal axial coordinates
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexAxialPos {
    pub q: i32,
    pub r: i32,
}

// Component for a game tile
#[derive(Component, Debug, Clone, PartialEq)]
pub struct Tile {
    pub owner_faction: Faction,
    pub army_count: u32,
}

// Marker component for entities that are part of the map
#[derive(Component)]
pub struct MapEntity;

// Hexagonal geometry constants
const HEX_RADIUS: f32 = 1.0;
const HEX_SQRT3: f32 = 1.7320508; // sqrt(3)

pub fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_settings: Res<GameSettings>, // Get game settings
    _asset_server: Res<AssetServer>, // AssetServer is no longer needed in this system
) {
    let hex_mesh = meshes.add(RegularPolygon::new(HEX_RADIUS, 6));
    let hex_outline_mesh = meshes.add(RegularPolygon::new(HEX_RADIUS * 1.05, 6)); // Slightly larger for outline

    for r_idx in 0..game_settings.board_height as i32 {
        for q_idx in 0..game_settings.board_width as i32 {
            let axial_pos = HexAxialPos { q: q_idx, r: r_idx };
            let world_pos = axial_to_world_coords(axial_pos, HEX_RADIUS);

            let initial_army_count = game_settings.initial_armies; // Use initial armies from settings

            let tile_entity = commands.spawn((
                PbrBundle {
                    mesh: hex_mesh.clone(),
                    material: materials.add(Faction::Neutral.color()), // Neutral color initially
                    transform: Transform::from_xyz(world_pos.x, 0.0, world_pos.z) // Y position for main hex
                                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)), // Lay flat
                    ..default()
                },
                axial_pos,
                Tile {
                    owner_faction: Faction::Neutral,
                    army_count: initial_army_count,
                },
                MapEntity, // Mark as map entity for cleanup
            )).id();

            // Spawn outline hex as a child of the main hex
            commands.entity(tile_entity).with_children(|parent| {
                parent.spawn((
                    PbrBundle {
                        mesh: hex_outline_mesh.clone(),
                        material: materials.add(Color::BLACK),
                        transform: Transform::from_xyz(0.0, -0.01, 0.0) // Relative to parent hex, slightly lower for outline
                                    .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)), // Lay flat
                        ..default()
                    },
                    MapEntity,
                ));
            });
        }
    }
}

fn axial_to_world_coords(hex_pos: HexAxialPos, hex_radius: f32) -> Vec3 {
    let x = hex_radius * HEX_SQRT3 * (hex_pos.q as f32 + hex_pos.r as f32 / 2.0);
    let z = hex_radius * 1.5 * hex_pos.r as f32;
    Vec3::new(x, 0.0, z)
}