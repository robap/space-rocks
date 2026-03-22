use bevy::prelude::*;

use crate::components::*;
use crate::config::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AsteroidDestroyedEvent>().add_systems(
            Update,
            on_asteroid_destroyed
                .after(GameSet::Collision)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

pub fn points_for_size(size: AsteroidSize) -> u32 {
    match size {
        AsteroidSize::Large => SCORE_LARGE,
        AsteroidSize::Medium => SCORE_MEDIUM,
        AsteroidSize::Small => SCORE_SMALL,
    }
}

fn on_asteroid_destroyed(
    mut events: EventReader<AsteroidDestroyedEvent>,
    mut score: ResMut<Score>,
) {
    for event in events.read() {
        score.0 = score.0.saturating_add(points_for_size(event.size));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_asteroid_scores_20() {
        assert_eq!(points_for_size(AsteroidSize::Large), 20);
    }

    #[test]
    fn medium_asteroid_scores_50() {
        assert_eq!(points_for_size(AsteroidSize::Medium), 50);
    }

    #[test]
    fn small_asteroid_scores_100() {
        assert_eq!(points_for_size(AsteroidSize::Small), 100);
    }
}
