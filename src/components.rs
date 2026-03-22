use bevy::prelude::*;

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
}
