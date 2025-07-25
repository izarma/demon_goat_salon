use bevy::prelude::*;

use crate::{
    engine::GameState,
    ui::{
        game_over::spawn_game_over_ui,
        main_menu::{button_interaction_system, cleanup_menu, setup_main_menu},
    },
};

pub mod game_over;
pub mod main_menu;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                button_interaction_system.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu)
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over_ui);
    }
}
