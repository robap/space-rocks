use bevy::prelude::*;

use crate::components::*;

type LivesTextFilter = (
    With<HudLivesText>,
    Without<HudScoreText>,
    Without<HudLevelText>,
);
type ScoreTextFilter = (
    With<HudScoreText>,
    Without<HudLivesText>,
    Without<HudLevelText>,
);
type LevelTextFilter = (
    With<HudLevelText>,
    Without<HudLivesText>,
    Without<HudScoreText>,
);

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_hud, spawn_overlay_text))
            .add_systems(Update, update_hud)
            .add_systems(OnEnter(GameState::GameOver), show_game_over_text)
            .add_systems(OnEnter(GameState::Playing), hide_game_over_text)
            .add_systems(OnEnter(GameState::Attract), show_press_any_key)
            .add_systems(OnEnter(GameState::GameOver), show_press_any_key)
            .add_systems(OnEnter(GameState::Playing), hide_press_any_key)
            .add_systems(OnEnter(GameState::LevelTransition), show_level_ready_text)
            .add_systems(OnExit(GameState::LevelTransition), hide_level_ready_text);
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

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Level: 1"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                HudLevelText,
            ));
        });
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

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(40.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Level 1 - Get Ready"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Visibility::Hidden,
                LevelReadyText,
            ));
        });
}

fn update_hud(
    lives: Res<Lives>,
    score: Res<Score>,
    level: Res<Level>,
    mut lives_text: Query<&mut Text, LivesTextFilter>,
    mut score_text: Query<&mut Text, ScoreTextFilter>,
    mut level_text: Query<&mut Text, LevelTextFilter>,
) {
    if let Ok(mut text) = lives_text.get_single_mut() {
        **text = format!("Lives: {}", lives.0);
    }
    if let Ok(mut text) = score_text.get_single_mut() {
        **text = format!("Score: {}", score.0);
    }
    if let Ok(mut text) = level_text.get_single_mut() {
        **text = format!("Level: {}", level.number);
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

fn show_level_ready_text(
    level: Res<Level>,
    mut text_query: Query<&mut Text, With<LevelReadyText>>,
    mut vis_query: Query<&mut Visibility, With<LevelReadyText>>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        **text = format!("Level {} - Get Ready", level.number);
    }
    if let Ok(mut vis) = vis_query.get_single_mut() {
        *vis = Visibility::Visible;
    }
}

fn hide_level_ready_text(mut query: Query<&mut Visibility, With<LevelReadyText>>) {
    if let Ok(mut vis) = query.get_single_mut() {
        *vis = Visibility::Hidden;
    }
}
