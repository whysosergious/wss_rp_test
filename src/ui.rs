use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::AppState;

// Enum to hold UI settings
#[derive(Resource)]
pub struct GameSettings {
    pub board_width: u32,
    pub board_height: u32,
    pub num_players: u32,
    pub initial_armies: u32,
    pub reinforcement_rate: u32,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            board_width: 10,
            board_height: 10,
            num_players: 2,
            initial_armies: 2,
            reinforcement_rate: 1,
        }
    }
}

pub fn setup_ui(mut contexts: EguiContexts, mut game_settings: ResMut<GameSettings>, mut next_state: ResMut<NextState<AppState>>) {
    let mut errors = Vec::new();

    egui::Window::new("Game Setup")
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.add(egui::Slider::new(&mut game_settings.board_width, 5..=20).text("Board Width"));
            ui.add(egui::Slider::new(&mut game_settings.board_height, 5..=20).text("Board Height"));
            ui.add(egui::Slider::new(&mut game_settings.num_players, 2..=6).text("Number of Players"));
            ui.add(egui::Slider::new(&mut game_settings.initial_armies, 1..=5).text("Initial Armies per Tile"));
            ui.add(egui::Slider::new(&mut game_settings.reinforcement_rate, 1..=10).text("Reinforcement Rate"));

            // Validation logic
            if game_settings.board_width < 5 || game_settings.board_width > 20 {
                errors.push("Board Width must be between 5 and 20.");
            }
            if game_settings.board_height < 5 || game_settings.board_height > 20 {
                errors.push("Board Height must be between 5 and 20.");
            }
            if game_settings.num_players < 2 || game_settings.num_players > 6 {
                errors.push("Number of Players must be between 2 and 6.");
            }
            if game_settings.initial_armies < 1 || game_settings.initial_armies > 5 {
                errors.push("Initial Armies per Tile must be between 1 and 5.");
            }
            if game_settings.reinforcement_rate < 1 || game_settings.reinforcement_rate > 10 {
                errors.push("Reinforcement Rate must be between 1 and 10.");
            }
            // Add check for number of players vs board size to ensure enough starting territories
            if game_settings.num_players > game_settings.board_width * game_settings.board_height / 4 { // Arbitrary heuristic
                 errors.push("Too many players for the chosen board size.");
            }


            ui.separator();

            for error in &errors {
                ui.colored_label(egui::Color32::RED, *error);
            }

            ui.add_enabled_ui(errors.is_empty(), |ui| {
                if ui.button("Generate Board").clicked() {
                    next_state.set(AppState::GameState);
                }
            });
        });
}
