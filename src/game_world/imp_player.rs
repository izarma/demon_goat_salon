use avian2d::prelude::*;
use bevy::{prelude::*};
use bevy_enhanced_input::{
    action::Action,
    actions,
    prelude::{Bidirectional, Binding, Bindings, DeltaScale, GamepadDevice},
};
use bevy_tnua::{TnuaAnimatingState, prelude::TnuaController};

use crate::{
    animation::{animation_state::AnimationState, sprite_animation::SpriteAnimState},
    engine::{
        asset_loader::ImageAssets, game_runner::OnGameScreen, input_manager::Move, GameState
    }, ui::customer_details::setup_points,
};

pub struct ImpPlugin;

impl Plugin for ImpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (setup_characters, setup_points));
    }
}

/// Used as both input context and component.
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
#[require(GamepadDevice::None,
    RigidBody::Dynamic,
    Collider::capsule(40., 144.0),
    TnuaController,
    SpriteAnimState {
            start_index: 0,
            end_index: 9,
            timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
    },
)]
pub enum Player {
    First,
    Second,
}

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
        OnGameScreen,
        Player::First,
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
        actions!(
            Player[(
                Action::<Move>::new(),
                DeltaScale,
                Bindings::spawn((
                    Bidirectional {
                        positive: Binding::from(KeyCode::KeyD),
                        negative: Binding::from(KeyCode::KeyA),
                    },
                    Spawn(Binding::from(GamepadAxis::LeftStickX)),
                ))
            )]
        ),
    ));
    cmd.insert((TnuaAnimatingState::<AnimationState>::default(),));
    let mut cmd2 = commands.spawn((
        OnGameScreen,
        Player::Second,
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
        actions!(
            Player[(
                Action::<Move>::new(),
                DeltaScale,
                Bindings::spawn((
                    Bidirectional {
                        positive: Binding::from(KeyCode::ArrowRight),
                        negative: Binding::from(KeyCode::ArrowLeft),
                    },
                    Spawn(Binding::from(GamepadAxis::LeftStickX)),
                ))
            )]
        ),
    ));
    cmd2.insert((TnuaAnimatingState::<AnimationState>::default(),));
}
