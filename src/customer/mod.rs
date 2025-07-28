use std::time::Duration;

use bevy::prelude::*;

use crate::{
    animation::sprite_animation::animate_sprite,
    customer::customer_timer::{game_over, spawn_timer, update_timer},
    engine::{GameState, asset_loader::ImageAssets, game_runner::play_salon_bg},
    ui::game_over::OnGameOver,
};

mod customer_timer;

pub struct CustomerPlugin;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            ((spawn_customer, spawn_timer).chain(), play_salon_bg),
        )
        .add_systems(
            Update,
            (animate_sprite, update_timer, move_jaw, game_over).run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Component)]
pub struct Customer {
    anger_timer: Timer,
}

#[derive(Component)]
pub struct CustomerBody;

#[derive(Component)]
pub struct GoatJaw;

#[derive(Component)]
pub struct GoatHair;

fn spawn_customer(mut commands: Commands, image_assets: Res<ImageAssets>) {
    // goat base
    commands.spawn((
        Customer {
            anger_timer: Timer::new(Duration::from_secs_f32(60.0), TimerMode::Once),
        },
        Sprite {
            image: image_assets.goat_base.clone(),
            custom_size: Some(Vec2::new(2787., 1390.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0., 50., -10.),
            scale: Vec3::new(0.6, 0.6, 1.0),
            ..default()
        },
        OnGameOver,
    ));

    // goat jaw
    let jaw_center = Vec3::new(0., -115., -11.);
    commands.spawn((
        CustomerBody,
        GoatJaw,
        Sprite {
            image: image_assets.goat_jaw.clone(),
            custom_size: Some(Vec2::new(306., 1002.)),
            ..default()
        },
        Transform {
            translation: jaw_center,
            scale: Vec3::new(0.6, 0.6, 1.0),
            ..default()
        },
        JawMotion {
            center: jaw_center,
            radius: 15.0,
            speed: 6.0,
        },
        OnGameOver,
    ));

    // goat ears
    commands.spawn((
        CustomerBody,
        Sprite {
            image: image_assets.goat_ears.clone(),
            custom_size: Some(Vec2::new(1330., 419.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0., 150., -10.),
            scale: Vec3::new(0.6, 0.6, 1.0),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair A1 left
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.goat_hair_a1_left.clone(),
            custom_size: Some(Vec2::new(427., 675.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-120., 100., -9.),
            scale: Vec3::new(0.6, 0.6, 1.0),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair A1 right
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.goat_hair_a1_right.clone(),
            custom_size: Some(Vec2::new(427., 677.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(120., 100., -9.),
            scale: Vec3::new(0.6, 0.6, 1.0),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair A2 left
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.goat_hair_a2_left.clone(),
            custom_size: Some(Vec2::new(360., 1044.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-200., -140., -9.5),
            scale: Vec3::new(0.6, 0.6, 1.0),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair A2 right
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.goat_hair_a2_right.clone(),
            custom_size: Some(Vec2::new(353., 1039.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(200., -140., -9.5),
            scale: Vec3::new(0.6, 0.6, 1.0),
            ..default()
        },
        OnGameOver,
    ));
}

#[derive(Component)]
struct JawMotion {
    center: Vec3,
    radius: f32,
    speed: f32,
}

fn move_jaw(mut jaw_query: Query<(&mut Transform, &JawMotion), With<GoatJaw>>, time: Res<Time>) {
    for (mut transform, motion) in jaw_query.iter_mut() {
        let t = time.elapsed_secs() * motion.speed;
        let x_offset = motion.radius * t.cos();
        let y_offset = motion.radius * t.sin();

        transform.translation = motion.center + Vec3::new(x_offset, y_offset, 0.0);
    }
}
