use bevy::state::state::States;

pub mod asset_loader;
pub mod game_runner;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    InGame,
    Upgrades,
}
