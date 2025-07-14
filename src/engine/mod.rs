use bevy::state::state::States;

pub mod asset_loader;
pub mod game_runner;
pub mod physics_engine;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    InGame,
    Upgrades,
}
