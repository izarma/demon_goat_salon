use avian2d::prelude::*;
use bevy::prelude::*;

use crate::engine::{GameState, game_runner::OnGameScreen};

pub struct SalonPlugin;

impl Plugin for SalonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_salon);
    }
}

#[derive(Component)]
struct Interactable;

#[derive(Component)]
pub struct MovingPlatform;

#[derive(Component)]
pub struct ControlPanel;

fn setup_salon(mut commands: Commands) {
    let square_sprite = Sprite {
        color: Color::srgb(0.7, 0.1, 0.1),
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    };
    // Floor
    commands.spawn((
        square_sprite.clone(),
        Transform::from_xyz(0.0, -490.0, 0.0).with_scale(Vec3::new(38.0, 1.0, 1.0)),
        RigidBody::Static,
        OnGameScreen,
        Collider::rectangle(50.0, 50.0),
    ));

    // Platform
    commands.spawn((
        square_sprite.clone(),
        MovingPlatform,
        OnGameScreen,
        Transform::from_xyz(0.0, -440.0, 0.0).with_scale(Vec3::new(3.0, 1.0, 1.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0),
    ));

    // Control Panel
    commands.spawn((
        square_sprite.clone(),
        ControlPanel,
        OnGameScreen,
        Interactable,
        Transform::from_xyz(-500.0, -440.0, 0.0).with_scale(Vec3::new(0.5, 1.0, 1.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0),
    ));
}
