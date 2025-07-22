use bevy::prelude::*;

use crate::{
    animation::sprite_animation::{SpriteAnimState, animate_sprite},
    engine::{GameState, asset_loader::ImageAssets, game_runner::play_salon_bg},
};

pub struct CustomerPlugin;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (spawn_customer, play_salon_bg))
            .add_systems(Update, (animate_sprite).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component)]
pub struct Customer;

fn spawn_customer(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        Customer,
        Sprite {
            image: image_assets.goat_base.clone(),
            custom_size: Some(Vec2::new(2787., 1390.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0., -15., -10.),
            scale: Vec3::new(0.6, 0.6, 1.0),
            ..default()
        },
    ));
}
