use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{engine::GameState, imp::{Player1, Player2}};

pub struct SalonPlugin;

impl Plugin for SalonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_salon)
        .add_systems(FixedUpdate, player_interact_system);
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
        Collider::rectangle(50.0, 50.0),
    ));

    // Platform
    commands.spawn((
        square_sprite.clone(),
        MovingPlatform,
        Transform::from_xyz(0.0, -440.0, 0.0).with_scale(Vec3::new(3.0, 1.0, 1.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0),
    ));

    // Control Panel
    commands.spawn((
        square_sprite.clone(),
        ControlPanel,
        Interactable,
        Transform::from_xyz(-500.0, -440.0, 0.0).with_scale(Vec3::new(0.5, 1.0, 1.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0),
    ));
}


fn player_interact_system(
    mut platform_query: Query<&mut Transform, (With<MovingPlatform>, Without<ControlPanel>)>,
    player_query: Query<&Transform, (With<Player1>, Without<MovingPlatform>)>,
    panel_query: Query<&Transform, (With<ControlPanel>, With<Interactable>)>,
    keys: Res<ButtonInput<KeyCode>>, 
) {
    for player_transform in player_query.iter() {
        for panel_transform in panel_query.iter() {
        let distance = player_transform
            .translation
            .truncate()
            .distance(panel_transform.translation.truncate());

        // Simple proximity check
        if distance < 100.0 {
            if keys.just_pressed(KeyCode::KeyE) {
                for mut platform_transform in platform_query.iter_mut() {
                    platform_transform.translation.y += 100.0; // or animate this movement
                    println!("Platform moved!");
                }
            }
            if keys.just_pressed(KeyCode::KeyQ) {
                for mut platform_transform in platform_query.iter_mut() {
                    platform_transform.translation.y += -100.0; // or animate this movement
                    println!("Platform moved!");
                }
            }
        }
    }
}
}
