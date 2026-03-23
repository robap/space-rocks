use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Attract,
    Playing,
    Dead,
    LevelTransition,
    GameOver,
}

#[derive(Resource)]
pub struct Lives(pub u32);

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct Invincible {
    pub timer: Timer,
    pub blink_timer: Timer,
}

#[derive(Resource)]
pub struct RespawnTimer(pub Timer);

#[derive(Event)]
pub struct BulletFiredEvent;

#[derive(Event)]
pub struct ShipDestroyedEvent;

#[derive(Event)]
pub struct AsteroidDestroyedEvent {
    pub size: AsteroidSize,
}

#[derive(Event)]
pub struct ResetGameEvent;

#[derive(Event)]
pub struct SpawnShipEvent {
    pub invincible: bool,
}

#[derive(Resource)]
pub struct Level {
    pub number: u32,
    pub active: bool,
}

#[derive(Event)]
pub struct SpawnLevelEvent {
    pub count: usize,
}

#[derive(Resource)]
pub struct LevelTransitionTimer(pub Timer);

#[derive(Component)]
pub struct HudLevelText;

#[derive(Component)]
pub struct LevelReadyText;

#[derive(Component)]
pub struct HudLivesText;

#[derive(Component)]
pub struct HudScoreText;

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct PressAnyKeyText;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    Movement,
    Collision,
    Despawn,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct BulletLifetime(pub Timer);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AsteroidSize {
    Large,
    Medium,
    Small,
}

impl AsteroidSize {
    pub fn split(self) -> Option<AsteroidSize> {
        match self {
            AsteroidSize::Large => Some(AsteroidSize::Medium),
            AsteroidSize::Medium => Some(AsteroidSize::Small),
            AsteroidSize::Small => None,
        }
    }

    pub fn radius(self) -> f32 {
        match self {
            AsteroidSize::Large => 48.0,
            AsteroidSize::Medium => 24.0,
            AsteroidSize::Small => 12.0,
        }
    }
}

#[derive(Component)]
pub struct Asteroid {
    pub size: AsteroidSize,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct AngularVelocity(pub f32);

#[derive(Component, Default)]
pub struct Thruster {
    pub active: bool,
}

pub fn wrap_position(translation: &mut Vec3, half_w: f32, half_h: f32) {
    if translation.x > half_w {
        translation.x = -half_w;
    }
    if translation.x < -half_w {
        translation.x = half_w;
    }
    if translation.y > half_h {
        translation.y = -half_h;
    }
    if translation.y < -half_h {
        translation.y = half_h;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_splits_to_medium() {
        assert_eq!(AsteroidSize::Large.split(), Some(AsteroidSize::Medium));
    }

    #[test]
    fn medium_splits_to_small() {
        assert_eq!(AsteroidSize::Medium.split(), Some(AsteroidSize::Small));
    }

    #[test]
    fn small_does_not_split() {
        assert_eq!(AsteroidSize::Small.split(), None);
    }

    #[test]
    fn radius_large() {
        assert_eq!(AsteroidSize::Large.radius(), 48.0);
    }

    #[test]
    fn radius_medium() {
        assert_eq!(AsteroidSize::Medium.radius(), 24.0);
    }

    #[test]
    fn radius_small() {
        assert_eq!(AsteroidSize::Small.radius(), 12.0);
    }

    #[test]
    fn lives_decrement_reaches_zero() {
        let mut lives = Lives(1);
        lives.0 = lives.0.saturating_sub(1);
        assert_eq!(lives.0, 0);
    }

    #[test]
    fn lives_saturating_sub_does_not_underflow() {
        let mut lives = Lives(0);
        lives.0 = lives.0.saturating_sub(1);
        assert_eq!(lives.0, 0);
    }

    #[test]
    fn wrap_position_past_right_edge_wraps_to_left() {
        let mut pos = Vec3::new(600.0, 0.0, 0.0);
        wrap_position(&mut pos, 500.0, 300.0);
        assert_eq!(pos.x, -500.0);
    }

    #[test]
    fn wrap_position_past_left_edge_wraps_to_right() {
        let mut pos = Vec3::new(-600.0, 0.0, 0.0);
        wrap_position(&mut pos, 500.0, 300.0);
        assert_eq!(pos.x, 500.0);
    }

    #[test]
    fn wrap_position_past_top_edge_wraps_to_bottom() {
        let mut pos = Vec3::new(0.0, 400.0, 0.0);
        wrap_position(&mut pos, 500.0, 300.0);
        assert_eq!(pos.y, -300.0);
    }

    #[test]
    fn wrap_position_past_bottom_edge_wraps_to_top() {
        let mut pos = Vec3::new(0.0, -400.0, 0.0);
        wrap_position(&mut pos, 500.0, 300.0);
        assert_eq!(pos.y, 300.0);
    }

    #[test]
    fn wrap_position_within_bounds_is_unchanged() {
        let mut pos = Vec3::new(100.0, 150.0, 0.0);
        wrap_position(&mut pos, 500.0, 300.0);
        assert_eq!(pos, Vec3::new(100.0, 150.0, 0.0));
    }
}
