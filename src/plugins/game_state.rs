use bevy::prelude::*;

use crate::components::*;
use crate::config::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShipDestroyedEvent>()
            .add_event::<ResetGameEvent>()
            .add_systems(Startup, insert_game_resources)
            .add_systems(
                Update,
                handle_attract_input.run_if(in_state(GameState::Attract)),
            )
            .add_systems(
                Update,
                handle_game_over_input.run_if(in_state(GameState::GameOver)),
            )
            .add_systems(Update, on_ship_destroyed)
            .add_systems(Update, tick_respawn_timer.run_if(in_state(GameState::Dead)))
            .add_systems(
                Update,
                tick_invincibility
                    .run_if(in_state(GameState::Playing))
                    .in_set(GameSet::Movement),
            );
    }
}

fn insert_game_resources(mut commands: Commands) {
    commands.insert_resource(Lives(0));
    commands.insert_resource(Score(0));
}

fn handle_attract_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut reset_events: EventWriter<ResetGameEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut lives: ResMut<Lives>,
    mut score: ResMut<Score>,
) {
    if keys.get_just_pressed().next().is_none() {
        return;
    }
    lives.0 = PLAYER_STARTING_LIVES;
    score.0 = 0;
    reset_events.send(ResetGameEvent);
    next_state.set(GameState::Playing);
}

fn handle_game_over_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut reset_events: EventWriter<ResetGameEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut lives: ResMut<Lives>,
    mut score: ResMut<Score>,
) {
    if keys.get_just_pressed().next().is_none() {
        return;
    }
    lives.0 = PLAYER_STARTING_LIVES;
    score.0 = 0;
    reset_events.send(ResetGameEvent);
    next_state.set(GameState::Playing);
}

fn on_ship_destroyed(
    mut events: EventReader<ShipDestroyedEvent>,
    mut lives: ResMut<Lives>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    for _ in events.read() {
        if *state.get() != GameState::Playing {
            warn!("ShipDestroyedEvent received outside Playing state — ignoring");
            continue;
        }
        lives.0 = lives.0.saturating_sub(1);
        if lives.0 > 0 {
            commands.insert_resource(RespawnTimer(Timer::from_seconds(
                SHIP_RESPAWN_DELAY_SECS,
                TimerMode::Once,
            )));
            next_state.set(GameState::Dead);
        } else {
            next_state.set(GameState::GameOver);
        }
    }
}

fn tick_respawn_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<RespawnTimer>,
    mut spawn_events: EventWriter<SpawnShipEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        commands.remove_resource::<RespawnTimer>();
        spawn_events.send(SpawnShipEvent { invincible: true });
        next_state.set(GameState::Playing);
    }
}

fn tick_invincibility(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Invincible, &mut Visibility), With<Player>>,
) {
    let Ok((entity, mut inv, mut vis)) = query.get_single_mut() else {
        return;
    };
    inv.timer.tick(time.delta());
    inv.blink_timer.tick(time.delta());
    if inv.blink_timer.just_finished() {
        *vis = toggle_visibility(*vis);
    }
    if inv.timer.finished() {
        *vis = Visibility::Visible;
        commands.entity(entity).remove::<Invincible>();
    }
}

fn toggle_visibility(current: Visibility) -> Visibility {
    match current {
        Visibility::Visible => Visibility::Hidden,
        Visibility::Hidden | Visibility::Inherited => Visibility::Visible,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible_toggles_to_hidden() {
        assert_eq!(toggle_visibility(Visibility::Visible), Visibility::Hidden);
    }

    #[test]
    fn test_hidden_toggles_to_visible() {
        assert_eq!(toggle_visibility(Visibility::Hidden), Visibility::Visible);
    }

    #[test]
    fn test_inherited_toggles_to_visible() {
        assert_eq!(
            toggle_visibility(Visibility::Inherited),
            Visibility::Visible
        );
    }
}
