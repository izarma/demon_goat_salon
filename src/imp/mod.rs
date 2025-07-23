use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::{Actions, Fired};
use bevy_tnua::{
    TnuaAction, TnuaAnimatingState, TnuaObstacleRadar, TnuaUserControlsSystemSet,
    builtins::TnuaBuiltinClimb,
    prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController},
};

use crate::{
    animation::{animation_state::AnimationState, sprite_animation::SpriteAnimState},
    engine::{
        GameState,
        asset_loader::ImageAssets,
        input_manager::{Player, PlayerInputs},
    },
};

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
pub struct Player1;

#[derive(Component)]
pub struct Player2;

fn setup_characters(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let frame_size = UVec2::new(1023, 1024);
    let player_layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        frame_size as UVec2,
        10,
        1,
        None,
        None,
    ));
    let mut cmd = commands.spawn((
        Player1,
        Player::First,
        Actions::<Player>::default(),
        Sprite {
            image: image_assets.imp_idle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: player_layout_handle.clone(),
                index: 0,
            }),
            custom_size: Some(Vec2::new(256., 256.)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-400., -170., 1.)),
        SpriteAnimState {
            start_index: 0,
            end_index: 9,
            timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
        },
    ));
    cmd.insert((
        RigidBody::Dynamic,
        Collider::capsule(40., 144.0),
        TnuaController::default(),
        TnuaObstacleRadar::new(1.0, 3.0),
        TnuaAnimatingState::<AnimationState>::default(),
    ));
    let mut cmd2 = commands.spawn((
        Player2,
        Player::Second,
        Actions::<Player>::default(),
        Sprite {
            image: image_assets.imp_idle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: player_layout_handle,
                index: 0,
            }),
            custom_size: Some(Vec2::new(256., 256.)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-250., -170., 1.1)),
        SpriteAnimState {
            start_index: 0,
            end_index: 9,
            timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
        },
    ));
    cmd2.insert((
        RigidBody::Dynamic,
        Collider::capsule(40., 144.0),
        TnuaController::default(),
        TnuaAnimatingState::<AnimationState>::default(),
    ));
}

fn apply_controls(
    mut event_reader: EventReader<PlayerInputs>,
    mut players_query: Query<&mut TnuaController, With<Player>>,
) {
    for events in event_reader.read() {
        match events {
            PlayerInputs::Walk(entity, mov) => {
                let mut direction = Vec3::ZERO;
                info!("{}", mov);
                direction.x += mov * 10.0;
                let player_entity = entity.clone();
                let mut controller = players_query.get_mut(player_entity).unwrap();
                controller.basis(TnuaBuiltinWalk {
                    float_height: 70.0,
                    desired_velocity: direction.normalize_or_zero() * 20.0,
                    ..Default::default()
                });
            }
            PlayerInputs::Jump(entity) => {
                info!("Jump pressed for {:#?}", entity);
                let player_entity = entity.clone();
                let mut controller = players_query.get_mut(player_entity).unwrap();
                controller.action(TnuaBuiltinJump {
                    height: 65.0,
                    ..Default::default()
                })
            }
        }
    }
}
