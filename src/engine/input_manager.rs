use bevy::prelude::*;
use bevy_enhanced_input::{prelude::*, EnhancedInputPlugin};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnhancedInputPlugin));
    }
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct Player1Jump;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct Player2Jump;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
struct Player1Move;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
struct Player2Move;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct Player1Interact;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct Player2Interact;

#[derive(InputContext)]
struct OnFoot;

#[derive(InputContext)]
struct OperatingControlPanel;