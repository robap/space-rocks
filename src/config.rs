// These constants are the shared plugin API — used by tasks 4–7, not yet by task 2.
#![allow(dead_code)]

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
