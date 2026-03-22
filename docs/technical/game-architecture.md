# Game Architecture — Technical Reference

**Sources:** `src/main.rs`, `src/components.rs`, `src/config.rs`
**Spec:** `specs/project-bootstrap.md`
**Last updated:** 2026-03-22

---

## Overview

Space Rocks is a single-binary Bevy 0.15 application. The game loop is structured around four plugins (`ShipPlugin`, `AsteroidPlugin`, `BulletPlugin`, `CollisionPlugin`), a shared component module, a shared constants module, and a system-set ordering mechanism. This document covers the glue that connects those pieces: `main.rs`, `components.rs`, `config.rs`, and `GameSet`.

---

## Module Layout

```
src/
  main.rs          — App setup, plugin registration, system set configuration, camera
  components.rs    — All shared component/marker types, GameSet enum
  config.rs        — All tunable game constants
  plugins/
    mod.rs         — pub mod declarations
    ship.rs        — ShipPlugin
    asteroid.rs    — AsteroidPlugin
    bullet.rs      — BulletPlugin
    collision.rs   — CollisionPlugin
```

---

## App Startup (`main.rs`)

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((ShipPlugin, AsteroidPlugin, BulletPlugin, CollisionPlugin))
        .configure_sets(
            Update,
            (GameSet::Movement, GameSet::Collision, GameSet::Despawn).chain(),
        )
        .add_systems(Startup, setup_camera)
        .run();
}
```

`DefaultPlugins` provides the Bevy window, renderer, input, and asset systems. `bevy_audio` is disabled in `Cargo.toml` (no ALSA dependency needed on Linux for a sound-free MVP).

`setup_camera` spawns a `Camera2d` entity — required for anything to render.

---

## System Set Ordering (`GameSet`)

```rust
// src/components.rs
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    Movement,   // move_asteroids, wrap_asteroids, move_bullets, wrap_bullets, all ship systems
    Collision,  // bullet_asteroid_collision
    Despawn,    // bullet_lifetime
}
```

`GameSet` is defined in `components.rs` (not `main.rs`) so all plugins can import it via `use crate::components::*` without additional module dependencies.

The sets are chained in `main.rs`: `Movement → Collision → Despawn`. This ensures:

1. All entities are at their final positions for the frame before hit detection runs
2. Bullets are not despawned by lifetime before collision has a chance to process them
3. Collision despawns happen before lifetime despawns, minimising the double-despawn window

Each plugin tags its systems at registration:
- `AsteroidPlugin`: `(move_asteroids, wrap_asteroids).in_set(GameSet::Movement)`
- `BulletPlugin`: `(move_bullets, wrap_bullets).in_set(GameSet::Movement)`, `bullet_lifetime.in_set(GameSet::Despawn)`
- `ShipPlugin`: all Update systems `.in_set(GameSet::Movement)`
- `CollisionPlugin`: `bullet_asteroid_collision.in_set(GameSet::Collision)`

---

## Shared Components (`components.rs`)

All component types used by more than one plugin live here. Plugin-private types would live in the plugin's own file, but the MVP has none — every type is shared.

| Type | Kind | Used by |
|------|------|---------|
| `Player` | Marker component | ShipPlugin |
| `Bullet` | Marker component | ShipPlugin (spawn), BulletPlugin, CollisionPlugin |
| `BulletLifetime(Timer)` | Component | ShipPlugin (spawn), BulletPlugin (tick/despawn) |
| `Asteroid { size }` | Component | AsteroidPlugin, CollisionPlugin |
| `AsteroidSize` | Enum | AsteroidPlugin, CollisionPlugin, components.rs (split/radius logic) |
| `Velocity(Vec2)` | Component | ShipPlugin, AsteroidPlugin, BulletPlugin, CollisionPlugin |
| `AngularVelocity(f32)` | Component | AsteroidPlugin, CollisionPlugin |
| `Thruster { active }` | Component | ShipPlugin |
| `GameSet` | SystemSet enum | All plugins, main.rs |

`AsteroidSize` carries two methods in `components.rs`:
- `split(self) -> Option<AsteroidSize>` — `Large→Medium`, `Medium→Small`, `Small→None`
- `radius(self) -> f32` — `48.0 / 24.0 / 12.0` — used for both mesh size and collision radius

---

## Shared Constants (`config.rs`)

All tunable gameplay values live here as `pub const`. No magic numbers in systems. If you want to change the feel of the game, this is the only file you need to touch.

```rust
pub const SHIP_ROTATION_SPEED: f32 = 3.0;        // radians/sec
pub const SHIP_THRUST: f32 = 200.0;               // pixels/sec²
pub const SHIP_MAX_SPEED: f32 = 400.0;            // pixels/sec
pub const SHIP_DRAG: f32 = 0.98;                  // velocity multiplier per frame

pub const BULLET_SPEED: f32 = 500.0;              // pixels/sec
pub const BULLET_LIFETIME: f32 = 1.2;             // seconds
pub const BULLET_RADIUS: f32 = 3.0;               // pixels (mesh + collision)
pub const BULLET_SPAWN_OFFSET: f32 = 22.0;        // pixels forward from ship nose

pub const ASTEROID_INITIAL_COUNT: usize = 6;
pub const ASTEROID_MIN_SPEED: f32 = 40.0;         // pixels/sec
pub const ASTEROID_MAX_SPEED: f32 = 120.0;        // pixels/sec
pub const ASTEROID_MIN_ANGULAR_VELOCITY: f32 = -1.5; // rad/s
pub const ASTEROID_MAX_ANGULAR_VELOCITY: f32 = 1.5;  // rad/s
```

---

## Rendering

All entities are rendered using Bevy 0.15's required-components 2D mesh API — no sprite assets:

```rust
commands.spawn((
    Mesh2d(meshes.add(Circle::new(radius))),
    MeshMaterial2d(materials.add(Color::srgb(r, g, b))),
    Transform::from_xyz(x, y, z),
    // ... game components
));
```

The ship is a `Triangle2d`. Asteroids and bullets are `Circle`. Colors:
- Ship: `srgb(0.8, 0.9, 1.0)` — pale blue-white
- Asteroids: `srgb(0.6, 0.6, 0.6)` — grey
- Bullets: `srgb(1.0, 1.0, 0.5)` — yellow

Z-values: ship at `z=1.0`, asteroids and bullets at `z=0.0`. Ship always renders on top.

---

## Known Constraints and Gotchas

- **`Bevy 0.15` — no bundle types.** `SpriteBundle`, `MaterialMesh2dBundle` are removed. Always use the required-components spawn pattern shown above.

- **`DefaultPlugins` minus audio.** `Cargo.toml` sets `default-features = false` with explicit feature list to exclude `bevy_audio`. If adding audio in a future feature, re-enable the audio feature.

- **`GameSet` must be imported wherever systems are registered.** All plugins do `use crate::components::*` which picks it up. `main.rs` imports it explicitly as `use components::GameSet`.

- **No `Result` propagation in systems.** Bevy systems return `()`. Errors use early return; invariant violations use `panic!`. See spec error handling strategy.
