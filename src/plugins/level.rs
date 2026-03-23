use bevy::prelude::*;

use crate::components::*;
use crate::config::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnLevelEvent>()
            .add_systems(Startup, insert_level_resource)
            .add_systems(Update, activate_level_when_asteroids_present)
            .add_systems(
                Update,
                detect_level_clear.run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::LevelTransition), start_level_transition)
            .add_systems(
                Update,
                tick_level_transition.run_if(in_state(GameState::LevelTransition)),
            )
            .add_systems(Update, on_reset_game);
    }
}

fn insert_level_resource(mut commands: Commands) {
    commands.insert_resource(Level {
        number: 1,
        active: false,
    });
}

fn asteroid_count_for_level(level: u32) -> usize {
    let count = ASTEROID_INITIAL_COUNT + (level as usize - 1) * ASTEROID_COUNT_INCREMENT;
    count.min(ASTEROID_MAX_COUNT)
}

fn activate_level_when_asteroids_present(
    asteroids: Query<(), With<Asteroid>>,
    mut level: ResMut<Level>,
) {
    if !level.active && !asteroids.is_empty() {
        level.active = true;
    }
}

fn detect_level_clear(
    asteroids: Query<(), With<Asteroid>>,
    mut level: ResMut<Level>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if level.active && asteroids.is_empty() {
        level.active = false;
        level.number += 1;
        next_state.set(GameState::LevelTransition);
    }
}

fn start_level_transition(mut commands: Commands) {
    commands.insert_resource(LevelTransitionTimer(Timer::from_seconds(
        LEVEL_TRANSITION_SECS,
        TimerMode::Once,
    )));
}

fn tick_level_transition(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Option<ResMut<LevelTransitionTimer>>,
    mut spawn_events: EventWriter<SpawnLevelEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    level: Res<Level>,
) {
    let Some(ref mut timer) = timer else {
        warn!("tick_level_transition: LevelTransitionTimer resource missing — skipping tick");
        return;
    };
    timer.0.tick(time.delta());
    if timer.0.finished() {
        commands.remove_resource::<LevelTransitionTimer>();
        spawn_events.send(SpawnLevelEvent {
            count: asteroid_count_for_level(level.number),
        });
        next_state.set(GameState::Playing);
    }
}

fn on_reset_game(
    mut events: EventReader<ResetGameEvent>,
    mut level: ResMut<Level>,
    mut commands: Commands,
    mut spawn_events: EventWriter<SpawnLevelEvent>,
) {
    for _ in events.read() {
        *level = Level {
            number: 1,
            active: false,
        };
        commands.remove_resource::<LevelTransitionTimer>();
        spawn_events.send(SpawnLevelEvent {
            count: ASTEROID_INITIAL_COUNT,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asteroid_count_level_1_is_initial_count() {
        assert_eq!(asteroid_count_for_level(1), ASTEROID_INITIAL_COUNT);
    }

    #[test]
    fn asteroid_count_level_2_adds_increment() {
        assert_eq!(
            asteroid_count_for_level(2),
            ASTEROID_INITIAL_COUNT + ASTEROID_COUNT_INCREMENT
        );
    }

    #[test]
    fn asteroid_count_level_3_adds_two_increments() {
        assert_eq!(
            asteroid_count_for_level(3),
            ASTEROID_INITIAL_COUNT + 2 * ASTEROID_COUNT_INCREMENT
        );
    }

    #[test]
    fn asteroid_count_caps_at_max() {
        assert_eq!(asteroid_count_for_level(100), ASTEROID_MAX_COUNT);
    }

    #[test]
    fn asteroid_count_at_cap_boundary() {
        // Level 4: 6 + 3*2 = 12 == ASTEROID_MAX_COUNT
        assert_eq!(asteroid_count_for_level(4), ASTEROID_MAX_COUNT);
    }
}
