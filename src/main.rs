use bevy::prelude::*;

use crate::engine::{GameState, game_runner::GameRunnerPlugin};

mod animation;
mod consts;
mod couch;
mod customer;
mod engine;
mod imp;
mod ui;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins.set(create_window_plugin()), GameRunnerPlugin))
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
