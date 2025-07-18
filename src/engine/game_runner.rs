use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    customer::CustomerPlugin, engine::{asset_loader::ImageAssets, input_manager::PlayerInputPlugin, physics_engine::PhysicsEnginePlugin, GameState}, imp::ImpPlugin, salon::SalonPlugin, ui::GameUiPlugin
};

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameUiPlugin, CustomerPlugin, SalonPlugin, ImpPlugin, PhysicsEnginePlugin, PlayerInputPlugin))
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    //.load_collection::<AudioAssets>()
                    .load_collection::<ImageAssets>()
                    .continue_to_state(GameState::InGame),
            )
            .add_systems(Startup, setup_camera)
            .insert_resource(Gravity(Vector::NEG_Y * 9.81 * 100.0));
    }
}

#[derive(Component)]
pub struct MainGameCamera;

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
