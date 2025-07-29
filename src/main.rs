use bevy::prelude::*;
use bevy::log::LogPlugin;

use crate::engine::{GameState, game_runner::GameRunnerPlugin};

mod animation;
mod consts;
mod customer;
mod engine;
mod imp;
mod salon;
mod ui;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins.set(
        LogPlugin {
    filter: "tnua=trace".into(),
    ..Default::default()
}
//create_window_plugin()
), GameRunnerPlugin))
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .run();
}

fn create_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Demon Goat Salon".to_string(),
            ..default()
        }),
        ..default()
    }
}
