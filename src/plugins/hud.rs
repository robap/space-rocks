use bevy::prelude::*;

use crate::components::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_hud, spawn_overlay_text))
            .add_systems(Update, update_hud)
            .add_systems(OnEnter(GameState::GameOver), show_game_over_text)
            .add_systems(OnEnter(GameState::Playing), hide_game_over_text)
            .add_systems(OnEnter(GameState::Attract), show_press_any_key)
            .add_systems(OnEnter(GameState::GameOver), show_press_any_key)
            .add_systems(OnEnter(GameState::Playing), hide_press_any_key);
    }
}

fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        Text::new("Lives: 3"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        HudLivesText,
    ));

    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
        HudScoreText,
    ));
}

fn spawn_overlay_text(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(45.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Game Over"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Visibility::Hidden,
                GameOverText,
            ));
        });

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(60.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Press Any Key to Start"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                PressAnyKeyText,
            ));
        });
}

fn update_hud(
    lives: Res<Lives>,
    score: Res<Score>,
    mut lives_text: Query<&mut Text, With<HudLivesText>>,
    mut score_text: Query<&mut Text, (With<HudScoreText>, Without<HudLivesText>)>,
) {
    if let Ok(mut text) = lives_text.get_single_mut() {
        **text = format!("Lives: {}", lives.0);
    }
    if let Ok(mut text) = score_text.get_single_mut() {
        **text = format!("Score: {}", score.0);
    }
}

fn show_game_over_text(mut query: Query<&mut Visibility, With<GameOverText>>) {
    if let Ok(mut vis) = query.get_single_mut() {
        *vis = Visibility::Visible;
    }
}

fn hide_game_over_text(mut query: Query<&mut Visibility, With<GameOverText>>) {
    if let Ok(mut vis) = query.get_single_mut() {
        *vis = Visibility::Hidden;
    }
}

fn show_press_any_key(mut query: Query<&mut Visibility, With<PressAnyKeyText>>) {
    if let Ok(mut vis) = query.get_single_mut() {
        *vis = Visibility::Visible;
    }
}

fn hide_press_any_key(mut query: Query<&mut Visibility, With<PressAnyKeyText>>) {
    if let Ok(mut vis) = query.get_single_mut() {
        *vis = Visibility::Hidden;
    }
}
