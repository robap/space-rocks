# Space Rocks — Project Bootstrap & MVP

**Status:** Draft
**Created:** 2026-03-22
**Spec author:** Refined via /refine skill

---

## Summary

Space Rocks is a classic Asteroids-style game built in Rust using the Bevy game engine. The MVP delivers the core mechanic: a player-controlled ship that can move and shoot, asteroids that float around the arena and split when hit. No scoring, lives, sound, or UI in the MVP — just the physics and feel.

---

## Motivation

This project exists to learn the Refine → Plan → Execute → Review skill pipeline using a real, playable hobby game as the vehicle. Bevy's ECS architecture is chosen to practice idiomatic Rust game development patterns that scale cleanly.

---

## Scope

### In scope (MVP)
- Project initialization: `Cargo.toml`, workspace structure, `CLAUDE.md` conventions
- Bevy app setup with a game loop
- Ship movement and rotation (thrust-based, classic Asteroids feel)
- Ship shooting — spawning bullets on input
- Asteroids floating across the screen with random velocity
- Asteroids splitting on bullet hit: Large → 2× Medium → 2× Small → destroyed
- Bullet lifetime — bullets despawn after a fixed duration
- Screen wrapping for ship, asteroids, and bullets

### Out of scope (MVP)
- Scoring system
- Lives and game over state
- Main menu or UI of any kind
- Sound effects or music
- Enemy ships
- Power-ups
- Web/WASM build target
- Save/load

---

## Architecture

### Where it lives

Single binary crate at the workspace root. No Cargo workspace — one `Cargo.toml`, one `src/` tree.

```
space_rocks/
├── Cargo.toml
├── CLAUDE.md
├── ROADMAP.md
├── specs/
│   ├── project-bootstrap.md   ← this file
│   └── diagrams/
│       └── initial-concept.excalidraw
└── src/
    ├── main.rs                 ← App setup, plugin registration
    ├── components.rs           ← All shared component and resource types
    ├── plugins/
    │   ├── mod.rs
    │   ├── ship.rs             ← ShipPlugin
    │   ├── asteroid.rs         ← AsteroidPlugin
    │   ├── bullet.rs           ← BulletPlugin
    │   └── collision.rs        ← CollisionPlugin
    └── config.rs               ← Game constants (speeds, sizes, counts)
```

### Key types

```rust
// components.rs

/// Marker component for the player's ship entity
#[derive(Component)]
pub struct Player;

/// Marker component for bullet entities
#[derive(Component)]
pub struct Bullet;

/// Bullet despawn timer — attached to each bullet entity
#[derive(Component)]
pub struct BulletLifetime(pub Timer);

/// Asteroid entity — carries its size for split logic
#[derive(Component)]
pub struct Asteroid {
    pub size: AsteroidSize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]  // Debug added post-implementation
pub enum AsteroidSize {
    Large,
    Medium,
    Small,
}

impl AsteroidSize {
    /// Returns the two children spawned when this asteroid is destroyed.
    /// Returns None for Small (fully destroyed).
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

/// Linear velocity — used by ship, asteroids, and bullets
#[derive(Component)]
pub struct Velocity(pub Vec2);

/// Rotational speed — used by ship and asteroids
#[derive(Component)]
pub struct AngularVelocity(pub f32);

/// Thrust force currently applied — used by ship movement system
#[derive(Component, Default)]
pub struct Thruster {
    pub active: bool,
}
```

```rust
// config.rs — tunable constants, not magic numbers in systems

pub const SHIP_ROTATION_SPEED: f32 = 3.0;   // radians/sec
pub const SHIP_THRUST: f32 = 200.0;          // pixels/sec²
pub const SHIP_MAX_SPEED: f32 = 400.0;       // pixels/sec
pub const SHIP_DRAG: f32 = 0.98;             // velocity multiplier per frame

pub const BULLET_SPEED: f32 = 500.0;         // pixels/sec
pub const BULLET_LIFETIME: f32 = 1.2;        // seconds
pub const BULLET_RADIUS: f32 = 3.0;          // pixels — used for collision (updated post-implementation)
pub const BULLET_SPAWN_OFFSET: f32 = 22.0;   // pixels forward from ship nose (updated post-implementation)

pub const ASTEROID_INITIAL_COUNT: usize = 6; // large asteroids at start
pub const ASTEROID_MIN_SPEED: f32 = 40.0;
pub const ASTEROID_MAX_SPEED: f32 = 120.0;
pub const ASTEROID_MIN_ANGULAR_VELOCITY: f32 = -1.5; // rad/s (updated post-implementation)
pub const ASTEROID_MAX_ANGULAR_VELOCITY: f32 = 1.5;  // rad/s (updated post-implementation)
```

### Plugin structure

Each plugin is a `struct` implementing `bevy::app::Plugin`. Plugins register their own systems and are added to the `App` in `main.rs`.

| Plugin | Reads | Writes | Responsibility |
|--------|-------|--------|----------------|
| `ShipPlugin` | keyboard input | `Transform`, `Velocity`, `Thruster` | Rotation, thrust, bullet spawn on space |
| `AsteroidPlugin` | — | `Transform`, `Velocity`, asteroid entities | Initial spawn, per-frame movement, screen wrap |
| `BulletPlugin` | — | `Transform`, `Velocity`, `BulletLifetime` | Per-frame movement, lifetime countdown, despawn |
| `CollisionPlugin` | all positions + radii | despawns entities, spawns split asteroids | Bullet↔asteroid detection, asteroid split logic |

### `main.rs` structure

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((ShipPlugin, AsteroidPlugin, BulletPlugin, CollisionPlugin))
        .configure_sets(Update, (GameSet::Movement, GameSet::Collision, GameSet::Despawn).chain())
        .add_systems(Startup, setup_camera)
        .run();
}
```

System set ordering (updated post-implementation): `GameSet` is defined in `components.rs` with three variants — `Movement`, `Collision`, `Despawn` — chained in that order in the `Update` schedule. All movement systems run before collision detection; bullet lifetime despawn runs last.

---

## Behavior

### Ship movement (thrust-based)

- **Rotate left/right**: `A`/`D` or arrow keys — applies angular rotation each frame
- **Thrust**: `W` or up arrow — applies forward force in the direction the ship faces
- **No braking input** — drag (`SHIP_DRAG`) bleeds speed naturally
- **Screen wrap**: when ship exits one edge, it appears on the opposite edge

### Shooting

- **Fire**: `Space` — spawns a `Bullet` entity at the ship's nose, with velocity = ship velocity + bullet speed in ship's facing direction
- Bullets have a `BulletLifetime` timer; `BulletPlugin` despawns them when it expires

### Asteroids

- Spawned at game start at random screen-edge positions with random velocity and angular velocity
- Move in a straight line each frame (no drag)
- Screen wrap applies
- On bullet collision: despawn asteroid, spawn 2× children of the next smaller size at the same position with slightly diverging velocities
- `Small` asteroids are fully destroyed (no children)

### Collision detection

- Simple circle vs. circle: compare distance between centers against sum of radii
- `CollisionPlugin` queries all `(Bullet, Transform)` and all `(Asteroid, Transform)` pairs each frame
- On hit: despawn bullet, despawn asteroid, run split logic

### Screen wrapping

All wrapped entities check if `Transform.translation` is outside window bounds and teleport to the opposite side. Wrapping is handled in each plugin's movement system (not a shared system) to keep plugins self-contained.

---

## Error Handling

No `Result` types in game systems — Bevy systems don't propagate errors. Invariant violations (e.g., asteroid with invalid size) should `panic!` with a clear message during development. No error recovery needed for MVP.

---

## Performance Considerations

No specific constraints. Target 60 FPS on any modern desktop. Asteroid count stays low enough (max ~24 entities from splitting) that naive O(n²) collision detection is fine for MVP.

---

## Testing Strategy

Unit tests on pure logic only — no Bevy app in tests for MVP.

```rust
#[test]
fn large_asteroid_splits_to_medium() {
    assert_eq!(AsteroidSize::Large.split(), Some(AsteroidSize::Medium));
}

#[test]
fn small_asteroid_does_not_split() {
    assert_eq!(AsteroidSize::Small.split(), None);
}
```

Visual/integration testing is manual: run the game and verify behavior.

---

## Open Questions

- TBD: Ship controls — confirm thrust-based (classic) vs. direct movement before implementing `ShipPlugin`
- TBD: Rendering — Bevy's built-in shapes (`bevy_prototype_debug_lines` or `bevy::sprite::MaterialMesh2dBundle`) vs. sprite assets. Simplest approach for MVP: colored meshes, no art assets required.

---

## Diagrams

- `specs/diagrams/initial-concept.excalidraw` — Game loop, entity types, and game state machine overview
