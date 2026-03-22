pub const SHIP_ROTATION_SPEED: f32 = 3.0;
pub const SHIP_THRUST: f32 = 200.0;
pub const SHIP_MAX_SPEED: f32 = 400.0;
pub const SHIP_DRAG: f32 = 0.98;
pub const BULLET_SPEED: f32 = 500.0;
pub const BULLET_LIFETIME: f32 = 1.2;
pub const BULLET_RADIUS: f32 = 3.0;
pub const BULLET_SPAWN_OFFSET: f32 = 22.0; // pixels forward from ship nose
pub const ASTEROID_INITIAL_COUNT: usize = 6;
pub const ASTEROID_MIN_SPEED: f32 = 40.0;
pub const ASTEROID_MAX_SPEED: f32 = 120.0;
pub const ASTEROID_MIN_ANGULAR_VELOCITY: f32 = -1.5; // rad/s
pub const ASTEROID_MAX_ANGULAR_VELOCITY: f32 = 1.5; // rad/s
pub const SHIP_RADIUS: f32 = 16.0;
pub const PLAYER_STARTING_LIVES: u32 = 3;
pub const SHIP_RESPAWN_DELAY_SECS: f32 = 1.5;
pub const SHIP_INVINCIBILITY_SECS: f32 = 2.0;
pub const SHIP_BLINK_INTERVAL_SECS: f32 = 0.1;

pub const SCORE_LARGE: u32 = 20;
pub const SCORE_MEDIUM: u32 = 50;
pub const SCORE_SMALL: u32 = 100;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn respawn_and_invincibility_constants_are_positive() {
        assert!(SHIP_RESPAWN_DELAY_SECS > 0.0);
        assert!(SHIP_INVINCIBILITY_SECS > 0.0);
        assert!(SHIP_BLINK_INTERVAL_SECS > 0.0);
        assert!(SHIP_BLINK_INTERVAL_SECS < SHIP_INVINCIBILITY_SECS);
    }
}
