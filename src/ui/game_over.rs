use bevy::prelude::*;

use crate::{
    consts::{BUTTON_BORDER, HOVERED_BUTTON, NORMAL_BUTTON, TEXT_COLOR},
    engine::{GameState, asset_loader::ImageAssets},
    imp::score::Score,
};

#[derive(Component)]
pub enum GameOverButtons {
    Retry,
    MainMenu,
}

#[derive(Component)]
pub struct OnGameOver;

pub fn spawn_game_over_ui(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    points_query: Query<&Score>,
    asset_server: Res<AssetServer>,
) {
    let menu_font = asset_server.load("fonts/Nasa21.ttf");
    commands
        .spawn((
            OnGameOver,
            Node {
                align_content: AlignContent::Center,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                position_type: PositionType::Relative,
                flex_wrap: FlexWrap::NoWrap,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                row_gap: Val::Px(40.0),
                ..Default::default()
            },
        ))
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
                        font: menu_font.clone(),
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
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(BUTTON_BORDER),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .insert(GameOverButtons::Retry)
                .with_children(|parent| {
                    parent.spawn((
                        Text::from("Try Again"),
                        TextFont {
                            font: menu_font.clone(),
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ));
                });
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(BUTTON_BORDER),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .insert(GameOverButtons::MainMenu)
                .with_children(|parent| {
                    parent.spawn((
                        Text::from("Main Menu"),
                        TextFont {
                            font: menu_font.clone(),
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ));
                });
        });
}

pub fn retry_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &GameOverButtons),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => match button {
                &GameOverButtons::Retry => {
                    game_state.set(GameState::InGame);
                }
                GameOverButtons::MainMenu => {
                    game_state.set(GameState::MainMenu);
                }
            },
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn cleanup_gameover(mut commands: Commands, query: Query<Entity, With<OnGameOver>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
