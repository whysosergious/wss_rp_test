use bevy::{
    prelude::*,
    text::{Text, TextSection, TextStyle},
    ui::{PositionType, Style, Val},
    window::PrimaryWindow,
};
use rand::seq::SliceRandom;
#[allow(deprecated)] // Suppress deprecated warning for thread_rng
use rand::thread_rng;

use crate::map::{HexAxialPos, MapEntity, Tile}; // Import MapEntity
use crate::ui::GameSettings;

#[allow(dead_code)] // Temporarily allow dead code for Faction variants until they are used
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Faction {
    Neutral,
    Player1,
    Player2,
    Player3,
    Player4,
    Player5,
    Player6,
}

impl Faction {
    pub fn color(&self) -> Color {
        match self {
            Faction::Neutral => Color::srgb(0.5, 0.5, 0.5), // Grey
            Faction::Player1 => Color::srgb(1.0, 0.0, 0.0), // Red
            Faction::Player2 => Color::srgb(0.0, 0.0, 1.0), // Blue
            Faction::Player3 => Color::srgb(0.0, 1.0, 0.0), // Green
            Faction::Player4 => Color::srgb(1.0, 1.0, 0.0), // Yellow
            Faction::Player5 => Color::srgb(1.0, 0.0, 1.0), // Magenta
            Faction::Player6 => Color::srgb(0.0, 1.0, 1.0), // Cyan - Corrected
        }
    }

    pub fn all_players() -> Vec<Faction> {
        vec![
            Faction::Player1,
            Faction::Player2,
            Faction::Player3,
            Faction::Player4,
            Faction::Player5,
            Faction::Player6,
        ]
    }
}

#[allow(dead_code)] // Temporarily allow dead code for ArmyText until it is used
#[derive(Component)]
pub struct ArmyText; // Marker for 3D text (not used for now)

#[derive(Component)]
pub struct ArmyCountDisplay {
    pub tile_entity: Entity,
}

pub fn assign_initial_territories(
    _commands: Commands,
    game_settings: Res<GameSettings>,
    mut tile_query: Query<(Entity, &HexAxialPos, &mut Tile, &Handle<StandardMaterial>)>, // Removed Option<&Children>
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut army_text_query: Query<(&mut Text, &ArmyCountDisplay)>, // Query for ArmyCountDisplay and Text
) {
    let mut rng = thread_rng();
    let mut all_hex_axial_positions: Vec<HexAxialPos> =
        tile_query.iter().map(|(_, pos, _, _)| *pos).collect(); // Adjusted map to ignore children
    all_hex_axial_positions.shuffle(&mut rng);

    let players = Faction::all_players()
        .into_iter()
        .take(game_settings.num_players as usize)
        .collect::<Vec<Faction>>();

    for (_player_idx, &player_faction) in players.iter().enumerate() {
        if let Some(start_pos) = all_hex_axial_positions.pop() {
            for (_entity, hex_axial_pos, mut tile, material_handle) in tile_query.iter_mut() { // Removed children
                if *hex_axial_pos == start_pos {
                    tile.owner_faction = player_faction;
                    tile.army_count = game_settings.initial_armies;

                    if let Some(material) = materials.get_mut(material_handle) {
                        material.base_color = player_faction.color();
                    }
                    // Removed army count text update logic, it's handled by update_army_text_ui
                    info!(
                        "Assigned {:?} to {:?} with {} armies",
                        start_pos, tile.owner_faction, tile.army_count
                    );
                    break;
                }
            }
        }
    }
}

// System to spawn 2D UI text for army counts
pub fn spawn_army_text_ui(
    mut commands: Commands,
    tile_query: Query<(Entity, &Tile, &Transform)>, // Query all Tile entities
    asset_server: Res<AssetServer>,
) {
    for (tile_entity, tile, _tile_transform) in tile_query.iter() {
        commands.spawn((
            TextBundle::from_section(
                tile.army_count.to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ),
            ArmyCountDisplay { tile_entity },
            MapEntity, // Mark ArmyCountDisplay as MapEntity for cleanup
        ));
    }
}

// System to update army count text content and position
pub fn update_army_text_ui(
    mut army_text_query: Query<(&mut Text, &mut Style, &ArmyCountDisplay)>,
    tile_query: Query<(&Tile, &GlobalTransform)>, // Use GlobalTransform for screen projection
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    _windows: Query<&Window, With<PrimaryWindow>>, // Prefixed window with _
) {
    let (camera, camera_transform) = camera_query.single();
    let _window = _windows.single(); // Access the window here

    for (mut text, mut style, army_count_display) in army_text_query.iter_mut() {
        if let Ok((tile, tile_global_transform)) = tile_query.get(army_count_display.tile_entity) {
            // Update text content
            text.sections[0].value = tile.army_count.to_string();

            // Hide if army count is 0 or tile is neutral
            if tile.army_count == 0 || tile.owner_faction == Faction::Neutral {
                style.display = Display::None;
                continue; // Skip further processing for hidden text
            } else {
                style.display = Display::Flex; // Ensure text is visible
            }

            // Project 3D world position to 2D screen position
            let world_pos = tile_global_transform.translation() + Vec3::Y * 0.5; // Position above tile
            let screen_pos_option = camera
                .world_to_viewport(camera_transform, world_pos);
                // .map(|pos| pos + Vec2::new(window.width() / 2.0, -window.height() / 2.0)); // Adjust to Bevy UI origin

            if let Some(screen_pos) = screen_pos_option {
                style.position_type = PositionType::Absolute;
                // Center horizontally and vertically within the projected point
                style.left = Val::Px(screen_pos.x - text.sections[0].style.font_size / 2.0);
                style.top = Val::Px(screen_pos.y - text.sections[0].style.font_size / 2.0);
            }
        }
    }
}
