# CollisionPlugin — Technical Reference

**Source:** `src/plugins/collision.rs`
**Spec:** `specs/project-bootstrap.md`
**Review:** `reviews/project-bootstrap-task7.md`
**Last updated:** 2026-03-22

---

## Overview

`CollisionPlugin` detects bullet↔asteroid collisions each frame, despawns both the bullet and the hit asteroid, and spawns two child asteroids of the next smaller size (or none for `Small`). It runs after all movement systems via `GameSet::Collision`, ensuring positions are current before the hit test.

---

## Key Types

No new types introduced. Uses shared types from `components.rs`:

```rust
pub struct Bullet;         // marker — identifies bullet entities
pub struct Asteroid { pub size: AsteroidSize }
pub struct Velocity(pub Vec2);
pub struct AngularVelocity(pub f32);
```

Relevant constants from `config.rs`:

```rust
pub const BULLET_RADIUS: f32 = 3.0;
pub const ASTEROID_MIN_SPEED: f32 = 40.0;
pub const ASTEROID_MAX_SPEED: f32 = 120.0;
pub const ASTEROID_MIN_ANGULAR_VELOCITY: f32 = -1.5;
pub const ASTEROID_MAX_ANGULAR_VELOCITY: f32 = 1.5;
```

---

## Systems

| System | Schedule | Set | Responsibility |
|--------|----------|-----|----------------|
| `bullet_asteroid_collision` | `Update` | `GameSet::Collision` | O(n²) bullet×asteroid hit test; despawns hits; triggers split logic |

One helper (not a system):

| Function | Responsibility |
|----------|---------------|
| `spawn_split_asteroids` | Spawns 2 child asteroids at ±30° diverging velocities, or returns early for `Small` |
| `circles_are_colliding` | Pure geometry: returns `distance(a, b) < radius_a + radius_b` |

---

## Data Flow

Each frame, after `GameSet::Movement` completes:

1. `bullet_asteroid_collision` allocates a `HashSet<Entity>` to track asteroids already hit this frame
2. Outer loop: iterate every `(Entity, &Transform)` with `Bullet`
3. Inner loop: iterate every `(Entity, &Transform, &Asteroid)`
   - Skip asteroids already in `hit_asteroids` (double-despawn guard)
   - Call `circles_are_colliding(bullet_pos, asteroid_pos, BULLET_RADIUS, asteroid.size.radius())`
   - On hit: queue `despawn(bullet_entity)`, queue `despawn(asteroid_entity)`, insert asteroid into `hit_asteroids`, call `spawn_split_asteroids`, `break` inner loop
4. `spawn_split_asteroids`: if `size.split()` is `Some(child_size)`, spawn 2 entities at ±`FRAC_PI_6` (30°) offsets from a random base angle with random speed in `[ASTEROID_MIN_SPEED, ASTEROID_MAX_SPEED)`

---

## Collision Geometry

`circles_are_colliding` implements strict less-than (`<`):

```rust
fn circles_are_colliding(pos_a: Vec2, pos_b: Vec2, radius_a: f32, radius_b: f32) -> bool {
    pos_a.distance(pos_b) < radius_a + radius_b
}
```

Touching circles (distance == sum of radii) are **not** considered colliding. Unit tests cover: overlapping (hit), touching (miss), separated (miss), and diagonal (3-4-5 triangle, hit).

---

## Design Decisions

- **`HashSet` double-despawn guard for asteroids.** Two bullets can hit the same asteroid in one frame — both see the asteroid in the query (commands are deferred), both would try to despawn it. The HashSet prevents the second despawn attempt. Bullets are guarded by the `break` statement — each bullet processes at most one hit per frame.

- **`break` after bullet hit.** A bullet hitting one asteroid is consumed; the inner loop breaks immediately. A bullet cannot split two asteroids in one frame. This matches classic Asteroids behavior.

- **Split velocity is randomized, not inherited from parent.** The children's base direction is chosen randomly each split. A ±30° offset gives them a visible V-shape divergence. The parent's velocity is not passed to children — this was a deliberate simplicity choice for MVP.

- **`spawn_split_asteroids` uses `size.split()` for the early-return pattern.** `AsteroidSize::split()` returns `None` for `Small`. The helper's `let Some(child_size) = size.split() else { return }` is the canonical way to handle the no-split case without a separate conditional.

- **System set ordering via `GameSet`, not `.after()`.** Making movement systems `pub` to reference them in `.after()` calls would widen their visibility unnecessarily. `GameSet::Collision` after `GameSet::Movement` achieves the same ordering without coupling plugins to each other's internals.

---

## Integration Points

| System / Plugin | Relationship |
|-----------------|-------------|
| `AsteroidPlugin` | Spawns the asteroid entities that `CollisionPlugin` queries and despawns |
| `BulletPlugin` | Spawns the bullet entities; `bullet_lifetime` (in `GameSet::Despawn`) may also despawn bullets — see Gotchas |
| `ShipPlugin` | Spawns bullets via `ship_shoot` (also in `GameSet::Movement`) |
| `components.rs` | Provides all query components; `GameSet` enum controls ordering |
| `config.rs` | Provides `BULLET_RADIUS` and asteroid speed/angular velocity ranges |

---

## Known Constraints and Gotchas

- **Bullet double-despawn edge case.** If a bullet hits an asteroid in `GameSet::Collision` and its `BulletLifetime` timer also expires in the same frame's `GameSet::Despawn`, both `bullet_asteroid_collision` and `bullet_lifetime` will queue a `despawn` for the same bullet entity. Since commands are deferred, the entity still exists in the `BulletLifetime` query when `Despawn` runs. In Bevy 0.15, queuing two deferred despawns for the same entity is safe (the second is a no-op) — but confirm against the Bevy version in use if upgrading.

- **O(n²) collision detection.** At full split: 6 large → 12 medium → 24 small = up to 42 asteroids, plus however many bullets are in flight. With short bullet lifetimes (1.2s) the bullet count stays low. This is fine for the MVP. If asteroid count grows significantly (new-wave spawning, etc.), switch to spatial partitioning.

- **Children spawn at parent's position, not offset.** Both child asteroids spawn at exactly `asteroid_transform.translation`. If both children have very similar initial velocities, they can visually overlap for the first several frames before separating. The ±30° divergence mitigates this but doesn't eliminate overlap at spawn.

- **No collision with the player ship.** `CollisionPlugin` only checks bullet↔asteroid. Ship↔asteroid collision is out of scope for MVP (no lives system). Adding it later means a new query and a new despawn/game-over event.
