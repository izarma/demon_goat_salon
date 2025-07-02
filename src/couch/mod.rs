use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::engine::{asset_loader::ImageAssets, GameState};

pub struct CouchPlugin;

impl Plugin for CouchPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::InGame), setup_couch)
        .add_systems(Update, (couch_movement).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component)]
pub struct Couch;

fn setup_couch(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
) {
    commands.spawn((
        Couch,
        Sprite {
            image: image_assets.couch.clone(),
            custom_size: Some(Vec2::new(202.2, 94.6)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-400., -170., 1.)),
        RigidBody::Dynamic,
        Collider::rectangle(202.2, 94.6)
    ));
}

fn couch_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut couch: Query<&mut LinearVelocity, With<Couch>>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_secs_f64().adjust_precision();

    for mut linear_velocity in &mut couch {
        if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            // Use a higher acceleration for upwards movement to overcome gravity
            linear_velocity.y += 2500.0 * delta_time;
        }
        if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            linear_velocity.y -= 500.0 * delta_time;
        }
        if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            linear_velocity.x -= 500.0 * delta_time;
        }
        if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            linear_velocity.x += 500.0 * delta_time;
        }
    }
}