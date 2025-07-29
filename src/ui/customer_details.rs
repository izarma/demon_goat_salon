use bevy::prelude::*;

use crate::{
    consts::TEXT_COLOR,
    engine::{game_runner::OnGameScreen, GameState}, game_world::goat::Customer, ui::game_over::OnGameOver,
};

#[derive(Component)]
pub struct TimerUi;

pub fn spawn_timer(
    mut commands: Commands,
    customer_query: Query<&Customer>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(customer) = customer_query.single() {
        let customer_timer = customer.anger_timer.clone();
        commands.spawn((
            TimerUi,
            OnGameScreen,
            Text::new(format!(
                "Customer angry in : {:#?}",
                customer_timer.remaining()
            )),
            Transform::from_translation(Vec3::new(400.0, 0.0, 0.0)),
            BorderRadius::ZERO,
            TextFont {
                font: asset_server.load("fonts/UncialAntiqua-Regular.ttf"),
                font_size: 20.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                right: Val::Px(20.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
        ));
    }
}

pub fn update_timer(
    time: Res<Time>,
    mut customer_query: Query<&mut Customer>,
    mut timer_ui: Query<&mut Text, With<TimerUi>>,
) {
    if let Ok(mut customer) = customer_query.single_mut() {
        if let Ok(mut timer_text) = timer_ui.single_mut() {
            customer.anger_timer.tick(time.delta());
            // info!("Timer : {:.1?}", customer.anger_timer.remaining());
            timer_text.0 = format!("Goat Angry in : {:.1?}", customer.anger_timer.remaining());
        }
    }
}

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
            font: asset_server.load("fonts/UncialAntiqua-Regular.ttf"),
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


pub fn game_over(mut game_state: ResMut<NextState<GameState>>, customer_query: Query<&Customer>) {
    if let Ok(customer) = customer_query.single() {
        if customer.anger_timer.finished() {
            game_state.set(GameState::GameOver);
        }
    }
}
