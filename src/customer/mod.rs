use bevy::prelude::*;

use crate::{
    animation::sprite_animation::{SpriteAnimState, animate_sprite},
    engine::{GameState, asset_loader::ImageAssets},
};

pub struct CustomerPlugin;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_customer)
            .add_systems(Update, (animate_sprite).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component)]
pub struct Customer;

fn spawn_customer(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let frame_size = UVec2::new(128, 128);
    let customer_layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        frame_size as UVec2,
        8,
        1,
        None,
        None,
    ));
    commands.spawn((
        Customer,
        Sprite {
            image: image_assets.werewolf.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: customer_layout_handle,
                index: 0,
            }),
            custom_size: Some(Vec2::new(1024., 1024.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(400., 20., 1.),
            scale: Vec3::new(-1.0, 1.0, 1.0),
            ..default()
        },
        SpriteAnimState {
            start_index: 0,
            end_index: 7,
            timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
        },
    ));
}
