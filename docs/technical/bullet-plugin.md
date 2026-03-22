# BulletPlugin — Technical Reference

**Source:** `src/plugins/bullet.rs`
**Spec:** `specs/project-bootstrap.md`
**Review:** `reviews/project-bootstrap-task6.md`
**Last updated:** 2026-03-22

---

## Overview

`BulletPlugin` is responsible for everything that happens to a bullet after it is spawned: moving it each frame, wrapping it at screen edges, and despawning it when its lifetime expires. It does not spawn bullets — that is `ShipPlugin`'s responsibility (`ship_shoot` system). The plugin is deliberately narrow: three systems, no startup logic, no state.

---

## Key Types

Bullet types are defined in `src/components.rs` and documented in [ShipPlugin — Technical Reference](ship-plugin.md). Relevant types:

```rust
/// Marker component — identifies bullet entities.
#[derive(Component)]
pub struct Bullet;

/// Countdown timer attached to each bullet entity.
/// Created by ShipPlugin with duration BULLET_LIFETIME seconds.
/// BulletPlugin ticks this timer and despawns the entity when it finishes.
#[derive(Component)]
pub struct BulletLifetime(pub Timer);

/// Linear velocity in world-space pixels/sec.
/// Set at spawn time to ship_velocity + forward * BULLET_SPEED.
#[derive(Component)]
pub struct Velocity(pub Vec2);
```

Relevant constants from `src/config.rs`:

```rust
pub const BULLET_LIFETIME: f32 = 1.2;  // seconds — timer duration set at spawn
pub const BULLET_RADIUS: f32 = 3.0;    // collision radius — used by CollisionPlugin
```

`BulletPlugin` does not use `BULLET_LIFETIME` directly — the timer is already constructed with that duration when `ShipPlugin` spawns the bullet. `BULLET_RADIUS` is not used here either; it lives in `config.rs` for `CollisionPlugin` to import.

---

## Systems

| System | Schedule | Responsibility |
|--------|----------|----------------|
| `move_bullets` | `Update` | Translates each bullet by `Velocity` × `delta_secs` |
| `wrap_bullets` | `Update` | Teleports bullet to opposite edge when it exits the screen |
| `bullet_lifetime` | `Update` | Ticks `BulletLifetime` timer; despawns entity when timer finishes |

System set assignments: `move_bullets` and `wrap_bullets` are in `GameSet::Movement`; `bullet_lifetime` is in `GameSet::Despawn`. `GameSet::Despawn` runs after `GameSet::Collision` (configured in `main.rs`), so collision detection always sees current bullet positions and processes hits before lifetime despawn runs.

---

## Data Flow

Each frame:

1. `move_bullets` translates every `Bullet` entity's `Transform` by `velocity.0.extend(0.0) * time.delta_secs()`
2. `wrap_bullets` reads `Window` dimensions from `PrimaryWindow`; teleports any `Bullet` `Transform` that has exited the screen bounds to the opposite edge
3. `bullet_lifetime` ticks each `BulletLifetime` timer by `time.delta()`; if `lifetime.0.finished()`, calls `commands.entity(entity).despawn()`

Bullets are spawned by `ShipPlugin::ship_shoot` and consumed (despawned) either by `bullet_lifetime` here or by `CollisionPlugin::bullet_asteroid_collision` (task 7).

---

## Design Decisions

- **BulletPlugin moves but does not spawn.** Bullet spawning depends on ship state (position, rotation, velocity) and input — that logic belongs in `ShipPlugin`. `BulletPlugin` only needs to know that `Bullet` entities exist and have `Velocity`, `Transform`, and `BulletLifetime`. This keeps the plugin focused on one concern.

- **Screen wrapping added during review.** The initial Execute pass omitted `wrap_bullets`; the review caught it as a spec gap (spec scope: "Screen wrapping for ship, asteroids, and bullets"). Bullets at 500 px/s with a 1.2s lifetime can travel ~600 px — enough to cross a typical window. Wrapping is consistent with ship and asteroid behaviour.

- **Screen wrapping is local to each plugin.** The spec explicitly requires "not a shared system" to keep plugins self-contained. `wrap_bullets`, `wrap_ship`, and `wrap_asteroids` are independent copies of the same logic. See spec: "Wrapping is handled in each plugin's movement system."

---

## Integration Points

| System / Plugin | Relationship |
|-----------------|-------------|
| `ShipPlugin` | Spawns `Bullet` entities with `Velocity`, `Transform`, and `BulletLifetime` pre-set |
| `CollisionPlugin` | Also despawns `Bullet` entities on asteroid hit — double-despawn risk; see Gotchas |
| `components.rs` | Provides `Bullet`, `BulletLifetime`, `Velocity` |
| `config.rs` | Provides `BULLET_RADIUS` (not used here, but referenced by `CollisionPlugin`) |

---

## Known Constraints and Gotchas

- **Bullet double-despawn edge case.** If a bullet hits an asteroid in `GameSet::Collision` and its lifetime timer also expires in the same frame's `GameSet::Despawn`, both `bullet_asteroid_collision` and `bullet_lifetime` queue a despawn for the same entity. Since commands are deferred, the entity still exists in the query when `Despawn` runs. In Bevy 0.15 this is a safe no-op (the second despawn silently does nothing), but verify if upgrading Bevy versions. See [CollisionPlugin — Technical Reference](collision-plugin.md) for details.

- **`wrap_bullets` reads `PrimaryWindow` every frame.** If `get_single()` fails (no primary window), the system returns early and skips wrapping that frame. This is a safe fallback — bullets continue moving and will be despawned by `bullet_lifetime` regardless.
