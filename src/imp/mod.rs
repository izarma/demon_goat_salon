use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::{builtins::TnuaBuiltinClimb, prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController}, TnuaAction, TnuaAnimatingState, TnuaObstacleRadar, TnuaUserControlsSystemSet};

use crate::{animation::{animation_state::AnimationState, sprite_animation::SpriteAnimState}, engine::{asset_loader::ImageAssets, GameState}};

pub struct ImpPlugin;

impl Plugin for ImpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_characters)
        .add_systems(
            FixedUpdate,
            apply_controls.in_set(TnuaUserControlsSystemSet),
        );
    }
}

#[derive(Component)]
pub struct Players;

#[derive(Component)]
pub struct Player1;

#[derive(Component)]
pub struct Player2;

fn setup_characters(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let frame_size = UVec2::new(128, 128);
    let player_layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        frame_size as UVec2,
        13,
        1,
        None,
        None,
    ));
    let mut cmd = commands.spawn((
        Player1,
        Players,
        Sprite {
            image: image_assets.imp_idle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: player_layout_handle.clone(),
                index: 0,
            }),
            custom_size: Some(Vec2::new(128., 128.)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-400., -170., 1.)),
        SpriteAnimState {
            start_index: 0,
            end_index: 12,
            timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
        },
    ));
    cmd.insert((
        RigidBody::Dynamic,
        Collider::capsule(10., 48.0),
        TnuaController::default(),
        TnuaObstacleRadar::new(1.0, 3.0),
        TnuaAnimatingState::<AnimationState>::default(),
    ));
    let mut cmd2 = commands.spawn((
        Player2,
        Players,
        Sprite {
            image: image_assets.imp_idle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: player_layout_handle,
                index: 0,
            }),
            custom_size: Some(Vec2::new(128., 128.)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-250., -170., 1.1)),
        SpriteAnimState {
            start_index: 0,
            end_index: 12,
            timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
        },
    ));
    cmd2.insert((
        RigidBody::Dynamic,
        Collider::capsule(10., 48.0),
        TnuaController::default(),
        TnuaObstacleRadar::new(1.0, 3.0),
        TnuaAnimatingState::<AnimationState>::default(),
    ));
}

fn apply_controls(
    keyboard: Res<ButtonInput<KeyCode>>, 
    mut player1_query: Query<(&mut TnuaController, &TnuaObstacleRadar), (With<Player1>, Without<Player2>)>,
    mut player2_query: Query<(&mut TnuaController, &TnuaObstacleRadar), (With<Player2>, Without<Player1>)>,
) {
    // Player 1
    let Ok((mut controller, radar)) = player1_query.single_mut() else {
        return;
    };

    let is_climbing = controller.action_name() == Some(TnuaBuiltinClimb::NAME);

    let mut direction = Vec3::ZERO;

    if keyboard.any_pressed([KeyCode::ArrowLeft]) {
            direction -= Vec3::X;
        }
        if keyboard.any_pressed([KeyCode::ArrowRight]) {
            direction += Vec3::X;
        }

    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: direction.normalize_or_zero() * 200.0,
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 65.0,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });

    // Feed the jump action every frame as long as the player holds the jump button. If the player
    // stops holding the jump button, simply stop feeding the action.
    if keyboard.pressed(KeyCode::ArrowUp) {
        controller.action(TnuaBuiltinJump {
            // The height is the only mandatory field of the jump button.
            height: 65.0,
            // `TnuaBuiltinJump` also has customization fields with sensible defaults.
            ..Default::default()
        });
    }


    // Player 2
    let Ok((mut controller, radar)) = player2_query.single_mut() else {
        return;
    };

    let is_climbing = controller.action_name() == Some(TnuaBuiltinClimb::NAME);

    let mut direction = Vec3::ZERO;

    if keyboard.any_pressed([KeyCode::KeyA]) {
            direction -= Vec3::X;
        }
        if keyboard.any_pressed([KeyCode::KeyD]) {
            direction += Vec3::X;
        }

    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: direction.normalize_or_zero() * 200.0,
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 65.0,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });

    // Feed the jump action every frame as long as the player holds the jump button. If the player
    // stops holding the jump button, simply stop feeding the action.
    if keyboard.pressed(KeyCode::KeyW) {
        controller.action(TnuaBuiltinJump {
            // The height is the only mandatory field of the jump button.
            height: 65.0,
            // `TnuaBuiltinJump` also has customization fields with sensible defaults.
            ..Default::default()
        });
    }

}