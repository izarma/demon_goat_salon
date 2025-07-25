use bevy::prelude::*;

use crate::{
    consts::TEXT_COLOR,
    engine::{asset_loader::ImageAssets, game_runner::OnGameScreen},
    imp::score::Score,
};

pub fn spawn_game_over_ui(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    points_query: Query<&Score>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((Node {
            align_content: AlignContent::Center,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            position_type: PositionType::Relative,
            flex_wrap: FlexWrap::NoWrap,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            row_gap: Val::Px(100.0),
            ..Default::default()
        },))
        .with_children(|parent| {
            parent.spawn((
                ImageNode {
                    image: image_assets.game_over_text.clone(),
                    ..default()
                },
                Node {
                    width: Val::Px(982.0),
                    height: Val::Px(248.0),
                    ..default()
                },
            ));
            if let Ok(points) = points_query.single() {
                info!("Points: {:#?}", points.total);
                parent.spawn((
                    Text::new(format!("Your Total Score : {:#?}", points.total)),
                    BorderRadius::ZERO,
                    TextFont {
                        font: asset_server.load("fonts/Nasa21.ttf"),
                        font_size: 60.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        align_items: AlignItems::FlexStart,
                        justify_content: JustifyContent::FlexStart,
                        ..Default::default()
                    },
                ));
            }
        });
}
