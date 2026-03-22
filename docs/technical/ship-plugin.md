# ShipPlugin — Technical Reference

**Source:** `src/plugins/ship.rs`
**Spec:** `specs/project-bootstrap.md`
**Review:** `reviews/project-bootstrap-task5.md`
**Last updated:** 2026-03-22

---

## Overview

`ShipPlugin` manages the full lifecycle of the player's ship: spawning it at startup, applying rotation and thrust-based movement each frame, wrapping it at screen edges, and firing bullets on Space. It is the only plugin that reads keyboard input in the current MVP. The ship entity is identified by the `Player` marker component.

---

## Key Types

```rust
// src/components.rs

/// Marker component — identifies the single player ship entity.
#[derive(Component)]
pub struct Player;

/// Linear velocity in world-space pixels/sec. Used by ship, asteroids, and bullets.
#[derive(Component)]
pub struct Velocity(pub Vec2);

/// Tracks whether the thruster is active this frame.
/// Unused in MVP rendering, but available for a future thrust-flame effect.
#[derive(Component, Default)]
pub struct Thruster {
    pub active: bool,
}

/// Marker component for bullet entities spawned by the ship.
#[derive(Component)]
pub struct Bullet;

/// Countdown timer attached to each bullet entity; BulletPlugin despawns on expiry.
#[derive(Component)]
pub struct BulletLifetime(pub Timer);
```

```rust
// src/config.rs (ship and bullet constants)

pub const SHIP_ROTATION_SPEED: f32 = 3.0;   // radians/sec
pub const SHIP_THRUST: f32 = 200.0;          // pixels/sec² of acceleration
pub const SHIP_MAX_SPEED: f32 = 400.0;       // hard cap on velocity magnitude
pub const SHIP_DRAG: f32 = 0.98;             // velocity multiplier applied every frame

pub const BULLET_SPEED: f32 = 500.0;         // pixels/sec added in ship's facing direction
pub const BULLET_LIFETIME: f32 = 1.2;        // seconds before bullet is despawned
pub const BULLET_RADIUS: f32 = 3.0;          // collision radius; reused in CollisionPlugin
pub const BULLET_SPAWN_OFFSET: f32 = 22.0;   // pixels forward from ship centre to spawn bullet
```

---

## Systems

| System | Schedule | Responsibility |
|--------|----------|----------------|
| `spawn_ship` | `Startup` | Spawns the ship entity with mesh and initial components |
| `ship_rotation` | `Update` | Rotates ship on A/D or arrow keys |
| `ship_thrust` | `Update` | Applies forward acceleration on W/up, tracks `Thruster.active`, applies drag |
| `ship_movement` | `Update` | Translates ship by `Velocity` × `delta_secs` |
| `wrap_ship` | `Update` | Teleports ship to opposite edge when it exits the screen |
| `ship_shoot` | `Update` | Spawns a `Bullet` entity on Space (just-pressed) |

All `Update` systems are registered without explicit ordering relative to each other. `CollisionPlugin` (task 7) must order `bullet_asteroid_collision` **after** `ship_movement` to avoid acting on stale positions.

---

## Data Flow

Each frame, the Update systems run in registration order:

1. `ship_rotation` reads `ButtonInput<KeyCode>` → mutates `Transform.rotation`
2. `ship_thrust` reads `ButtonInput<KeyCode>` + `Transform.rotation` → mutates `Velocity.0`, sets `Thruster.active`, applies `SHIP_DRAG`
3. `ship_movement` reads `Velocity.0` → mutates `Transform.translation`
4. `wrap_ship` reads `Window` dimensions → teleports `Transform.translation` if out of bounds
5. `ship_shoot` reads `ButtonInput<KeyCode>` + `Transform` + `Velocity` → spawns a new bullet entity

Bullet entities are moved and despawned by `BulletPlugin`, not `ShipPlugin`.

---

## Design Decisions

- **Thrust-based movement, not direct.** Classic Asteroids feel: thrust adds to velocity, drag bleeds it off. `SHIP_DRAG` is applied every frame regardless of thrust input, so the ship always decelerates when not thrusting. The cap `SHIP_MAX_SPEED` prevents runaway velocity after sustained thrust.

- **Forward direction from rotation.** The ship mesh points up (+Y in local space). World-space forward is `(transform.rotation * Vec3::Y).truncate()`. This pattern is used identically in both `ship_thrust` and `ship_shoot` — any change to ship orientation convention must update both.

- **Bullet velocity inherits ship velocity.** `bullet_vel = ship_velocity.0 + forward * BULLET_SPEED`. Bullets fired while moving fast travel faster in the forward direction. This is faithful to the original arcade game's feel.

- **`BULLET_RADIUS` lives in `config.rs`, not hardcoded.** The value `3.0` appears in both the bullet mesh spawn (here) and the collision detection radius check (task 7 — `CollisionPlugin`). Extracting it to a named constant ensures both stay in sync if the bullet size is tuned.

- **Screen wrapping is local to each plugin.** The spec explicitly requires "not a shared system" to keep plugins self-contained. `wrap_ship` and `wrap_asteroids` are independent copies of the same logic. See spec: "Wrapping is handled in each plugin's movement system."

---

## Integration Points

| System / Plugin | Relationship |
|-----------------|-------------|
| `BulletPlugin` | Moves and despawns `Bullet` entities spawned here |
| `CollisionPlugin` | Reads `(Bullet, Transform)` and despawns bullets on asteroid hit; must use `BULLET_RADIUS` from `config.rs` for collision math |
| `components.rs` | Provides `Player`, `Velocity`, `Thruster`, `Bullet`, `BulletLifetime` |
| `config.rs` | Provides all tunable ship and bullet constants |

---

## Known Constraints and Gotchas

- **`BULLET_RADIUS` must stay in sync with `CollisionPlugin`.** The constant is defined in `config.rs` and used in both `ship_shoot` (mesh size) and the collision detection radius check in `collision.rs`. Never hardcode `3.0` in collision.rs — always import `BULLET_RADIUS`.

- **Single-ship assumption.** All systems use `query.get_single()` / `query.get_single_mut()`, which silently returns early if zero or multiple `Player` entities exist. Spawning a second `Player` entity will cause both to be ignored by all ship systems. There is no guard against this.

- **`Thruster.active` is set but not yet read.** The field tracks thrust state for a future flame/exhaust visual effect. It is set correctly each frame but has no consumer in the MVP. Do not remove it — it is the hook for that effect.

- **Triangle vertices are magic literals.** `Vec2::new(0.0, 20.0)`, `Vec2::new(-12.0, -14.0)`, `Vec2::new(12.0, -14.0)` are not in `config.rs`. These are rendering geometry (not tunable gameplay values) and are left as inline literals per MVP scope. If ship shape becomes configurable, extract to constants.
