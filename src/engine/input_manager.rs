use bevy::{input::gamepad::GamepadConnectionEvent, prelude::*};
use bevy_enhanced_input::{EnhancedInputPlugin, prelude::*};
use bevy_tnua::prelude::{TnuaBuiltinWalk, TnuaController};

use crate::imp::Player;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnhancedInputPlugin)
            .add_input_context::<Player>()
            .add_observer(on_move)
            .add_observer(on_move_end)
            .add_systems(PreUpdate, gamepad_assignment_system);
    }
}


#[derive(InputAction)]
#[action_output(f32)]
pub struct Move;

#[derive(InputAction)]
#[action_output(bool)]
struct Jump;

#[derive(InputAction)]
#[action_output(bool)]
struct Interact;

fn gamepad_assignment_system(
    mut events: EventReader<GamepadConnectionEvent>,
    mut devices: Query<&mut GamepadDevice, With<Player>>,
) {
    'outer: for event in events.read() {
        if event.connected() {
            for mut device in devices.iter_mut() {
                if *device == GamepadDevice::None {
                    *device = GamepadDevice::Single(event.gamepad);
                    continue 'outer;
                }
            }
        }
    }
}

fn on_move(
    trigger: Trigger<Fired<Move>>,
    mut controllers: Query<&mut TnuaController, With<Player>>,
) {
    info!("player {} moved {}", trigger.target(), trigger.value);
    controllers
        .get_mut(trigger.target())
        .unwrap()
        .basis(TnuaBuiltinWalk {
            desired_velocity: vec3(trigger.value * 10004.0, 0., 0.),
            float_height: 70.,
            ..Default::default()
        });
}

fn on_move_end(
    trigger: Trigger<Completed<Move>>,
    mut controllers: Query<&mut TnuaController, With<Player>>,
) {
    controllers
        .get_mut(trigger.target())
        .unwrap()
        .basis(TnuaBuiltinWalk {
            desired_velocity: Vec3::ZERO,
            float_height: 70.,
            ..Default::default()
        });
}
