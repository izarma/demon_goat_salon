use bevy::{input::gamepad::GamepadConnectionEvent, prelude::*};
use bevy_enhanced_input::{EnhancedInputPlugin, prelude::*};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnhancedInputPlugin))
            .add_event::<PlayerInputs>()
            .init_resource::<Gamepads>()
            .add_input_context::<Player>()
            .add_observer(bind)
            .add_observer(apply_movement)
            .add_observer(apply_jump)
            .add_systems(
                FixedUpdate,
                update_gamepads.run_if(on_event::<GamepadConnectionEvent>),
            );
    }
}

/// Used as both input context and component.
#[derive(InputContext, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    First,
    Second,
}

/// A resource that tracks all connected gamepads to pick them by index.
#[derive(Resource, Default, Deref, DerefMut)]
struct Gamepads(Vec<Entity>);

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
pub struct Move;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct Jump;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct Interact;

fn bind(
    trigger: Trigger<Bind<Player>>,
    gamepads: Query<Entity, With<Gamepad>>,
    mut players: Query<(&Player, &mut Actions<Player>)>,
) {
    let (&player, mut actions) = players.get_mut(trigger.target()).unwrap();
    let gamepad_entity = gamepads.iter().nth(player as usize);
    actions.set_gamepad(gamepad_entity.unwrap_or(Entity::PLACEHOLDER));

    match player {
        Player::First => {
            actions
                .bind::<Move>()
                .to(((Cardinal::wasd_keys()), Axial::left_stick()));
            actions.bind::<Jump>().to(KeyCode::KeyW);
        }
        Player::Second => {
            actions
                .bind::<Move>()
                .to((Cardinal::arrow_keys(), Axial::left_stick()));
            actions.bind::<Jump>().to(KeyCode::ArrowUp);
        }
    }

    actions
        .bind::<Move>()
        .with_modifiers((DeadZone::default(), SmoothNudge::default()));
}

fn update_gamepads(mut commands: Commands) {
    commands.trigger(RebindAll);
}

#[derive(Event, Debug)]
pub enum PlayerInputs {
    Walk(Entity, f32),
    Jump(Entity),
}

fn apply_movement(
    trigger: Trigger<Fired<Move>>,
    mut player_input_events: EventWriter<PlayerInputs>,
) {
    player_input_events.write(PlayerInputs::Walk(trigger.target(), trigger.value.x));
    //info!("entity : {:#?} , movement : {}", trigger.target(), trigger.value.x);
}

fn apply_jump(trigger: Trigger<Fired<Jump>>, mut player_input_events: EventWriter<PlayerInputs>) {
    player_input_events.write(PlayerInputs::Jump(trigger.target()));
}
