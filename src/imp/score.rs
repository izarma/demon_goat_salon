use bevy::prelude::*;

use crate::{consts::TEXT_COLOR, engine::game_runner::OnGameScreen, ui::game_over::OnGameOver};

#[derive(Component, Debug)]
pub struct Score {
    pub total: i32,
}

pub fn setup_points(mut commands: Commands, asset_server: Res<AssetServer>) {
    let initial_points = 0;
    commands.spawn((
        OnGameScreen,
        Text::new(format!("Current Score : {:#?}", initial_points)),
        Transform::from_translation(Vec3::new(400.0, 0.0, 0.0)),
        BorderRadius::ZERO,
        TextFont {
            font: asset_server.load("fonts/Nasa21.ttf"),
            font_size: 20.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(60.0),
            right: Val::Px(20.0),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexStart,
            ..Default::default()
        },
    ));
    commands.spawn((
        Score {
            total: initial_points,
        },
        OnGameOver,
    ));
}
