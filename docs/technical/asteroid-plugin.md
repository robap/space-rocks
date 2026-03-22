# AsteroidPlugin — Technical Reference

**Source:** `src/plugins/asteroid.rs`
**Spec:** `specs/project-bootstrap.md`
**Review:** `reviews/project-bootstrap-task4.md`
**Last updated:** 2026-03-22

---

## Overview

`AsteroidPlugin` handles the full asteroid lifecycle: spawning 6 large asteroids at startup at random screen-edge positions, moving them each frame, and wrapping them at screen edges. It does not handle bullet collisions or asteroid splitting — that is `CollisionPlugin`'s responsibility.

---

## Key Types

```rust
// src/components.rs

/// Asteroid entity — carries its size for split logic and radius lookup.
#[derive(Component)]
pub struct Asteroid {
    pub size: AsteroidSize,
}

/// Size tier — determines visual radius and split behaviour.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AsteroidSize {
    Large,   // radius 48.0 — splits into 2 Medium
    Medium,  // radius 24.0 — splits into 2 Small
    Small,   // radius 12.0 — fully destroyed on hit
}

/// Linear velocity in world-space pixels/sec.
#[derive(Component)]
pub struct Velocity(pub Vec2);

/// Rotational speed in radians/sec.
#[derive(Component)]
pub struct AngularVelocity(pub f32);
```

```rust
// src/config.rs (asteroid constants)

pub const ASTEROID_INITIAL_COUNT: usize = 6;
pub const ASTEROID_MIN_SPEED: f32 = 40.0;           // pixels/sec
pub const ASTEROID_MAX_SPEED: f32 = 120.0;          // pixels/sec
pub const ASTEROID_MIN_ANGULAR_VELOCITY: f32 = -1.5; // rad/s
pub const ASTEROID_MAX_ANGULAR_VELOCITY: f32 = 1.5;  // rad/s
```

---

## Systems

| System | Schedule | Set | Responsibility |
|--------|----------|-----|----------------|
| `spawn_asteroids` | `Startup` | — | Spawns `ASTEROID_INITIAL_COUNT` large asteroids at random screen edges |
| `move_asteroids` | `Update` | `GameSet::Movement` | Translates and rotates each asteroid by its `Velocity` and `AngularVelocity` |
| `wrap_asteroids` | `Update` | `GameSet::Movement` | Teleports asteroid to opposite edge when it exits screen bounds |

---

## Data Flow

**Startup:**

`spawn_asteroids` fires once. For each of the 6 asteroids:
1. `random_edge_position` picks a point on one of the four screen edges using `rand::thread_rng()`
2. A random angle and speed in `[ASTEROID_MIN_SPEED, ASTEROID_MAX_SPEED)` determine the `Velocity`
3. A random angular velocity in `[MIN, MAX)` determines `AngularVelocity`
4. Entity spawned with `Mesh2d(Circle::new(48.0))`, `MeshMaterial2d(grey)`, `Transform`, `Asteroid { size: Large }`, `Velocity`, `AngularVelocity`

**Each frame (GameSet::Movement):**

1. `move_asteroids`: `transform.translation += vel.0.extend(0.0) * dt`; `transform.rotate_z(ang_vel.0 * dt)`
2. `wrap_asteroids`: reads `PrimaryWindow` dimensions; if translation exceeds `±half_w` or `±half_h`, teleports to opposite side

---

## Helpers

**`random_edge_position(rng, half_w, half_h) -> (f32, f32)`**

Picks one of four edges (top/bottom/left/right) at random via `rng.gen_range(0u8..4)`, then picks a random coordinate along that edge. This means asteroids always start at a screen boundary and drift inward (since their velocity direction is random, not always aimed at center).

**`wrap_position(translation: &mut Vec3, half_w, half_h)`**

Pure function, unit-tested (5 tests). Mutates `x` and `y` independently: if past the right edge, teleport to the left edge, etc. No diagonal handling — each axis is independent.

---

## Design Decisions

- **Spawn at edges, not at random interior positions.** Starting on-screen would cause asteroids to immediately overlap the ship. Edge spawning gives the player a grace period at game start.

- **`AngularVelocity` is a separate component, not part of `Asteroid`.** The spec defines it as a shared type because future features (e.g., spinning debris) might use rotation without the `Asteroid` marker. Keeping it separate costs nothing.

- **Screen wrapping is local to this plugin.** The spec explicitly requires "not a shared system" to keep plugins self-contained. `wrap_asteroids`, `wrap_ship`, and `wrap_bullets` are independent copies of the same logic.

- **`rand::thread_rng()` called at spawn time only.** The RNG is seeded once per function call (not stored as a resource). This is fine for an MVP — no reproducible seeds needed.

---

## Integration Points

| System / Plugin | Relationship |
|-----------------|-------------|
| `CollisionPlugin` | Queries `(Entity, &Transform, &Asteroid)` for hit detection; despawns asteroid entities and spawns child asteroids via `spawn_split_asteroids` |
| `components.rs` | Provides `Asteroid`, `AsteroidSize`, `Velocity`, `AngularVelocity` |
| `config.rs` | Provides all asteroid spawn constants |
| `GameSet::Movement` | All Update systems tagged in this set — runs before `GameSet::Collision` |

---

## Known Constraints and Gotchas

- **`AsteroidSize::radius()` must stay in sync with spawn mesh size.** `spawn_asteroids` calls `meshes.add(Circle::new(AsteroidSize::Large.radius()))` — the mesh radius and the collision radius come from the same method. If you change the radius values, collision detection updates automatically.

- **Initial asteroids are always `Large`.** Splitting produces `Medium` and `Small` entities — those are spawned by `CollisionPlugin`, not `AsteroidPlugin`. There is no `spawn_asteroid_of_size` function; `CollisionPlugin` inlines the spawn logic in `spawn_split_asteroids`.

- **`wrap_asteroids` skips wrapping if `PrimaryWindow` is unavailable.** Uses `get_single()` with early return on `Err`. Asteroids off-screen are harmless and will be caught next frame.

- **Max asteroid count is bounded but not enforced explicitly.** Starting with 6 large asteroids and splitting fully: 6 → 12 medium → 24 small = 42 entities maximum. The spec notes O(n²) collision is fine up to this count.
