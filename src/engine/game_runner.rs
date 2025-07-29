use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_enhanced_input::{action::Action, prelude::{ActionOf, InputContextAppExt}};
use bevy_seedling::{SeedlingPlugin, sample::SamplePlayer};
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian2d::TnuaAvian2dPlugin;

use crate::{
    customer::CustomerPlugin,
    engine::{
        asset_loader::{AudioAssets, ImageAssets}, input_manager::{Move, on_move, on_move_end, PlayerInputPlugin}, GameState
    },
    imp::{ImpPlugin, Player},
    salon::SalonPlugin,
    ui::GameUiPlugin,
};

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GameUiPlugin,
            PhysicsPlugins::default(),
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian2dPlugin::new(FixedUpdate),
            PlayerInputPlugin,
            ImpPlugin,
            CustomerPlugin,
            SalonPlugin,
            SeedlingPlugin::default(),
        ))
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .load_collection::<AudioAssets>()
                .load_collection::<ImageAssets>()
                .continue_to_state(GameState::InGame),
        )
        .add_systems(Startup, setup_camera)
        .add_systems(OnExit(GameState::InGame), cleanup_game)
        .insert_resource(Gravity(Vector::NEG_Y * 9.81 * 100.0))
        .add_input_context::<Player>()
        .add_observer(on_move)
        .add_observer(on_move_end);
    }
}

#[derive(Component)]
pub struct MainGameCamera;

#[derive(Component)]
pub struct OnGameScreen;

fn setup_camera(mut commands: Commands) {
    let main_camera = Camera2d::default();
    let projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: bevy::render::camera::ScalingMode::AutoMin {
            min_width: (1920.0),
            min_height: (1080.0),
        },
        ..OrthographicProjection::default_2d()
    });
    commands.spawn((main_camera, MainGameCamera, projection));
}

pub fn play_salon_bg(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn((SamplePlayer::new(audio_assets.background.clone()).looping(),));
}

pub fn cleanup_game(mut commands: Commands, query: Query<Entity, With<OnGameScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}