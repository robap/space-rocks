# Plan: Project Bootstrap & MVP

**Spec:** `specs/project-bootstrap.md`
**Status:** Complete ✓
**Created:** 2026-03-22

---

## Overview

Build the complete Space Rocks MVP from an empty directory: project scaffolding first, then shared types, then each plugin bottom-up (Asteroid → Ship → Bullet → Collision). This ordering keeps every compile-checkpoint green — shared types are defined before any plugin references them, and CollisionPlugin is last because it reads types from all other plugins. Rendering uses Bevy 0.15's required-components 2D mesh API (`Mesh2d` + `MeshMaterial2d<ColorMaterial>`) — no art assets required.

---

## Prerequisites

- None — this is the first and only plan for this project.

---

## Tasks

### 1. Project Scaffolding

> Establish a compiling Bevy app skeleton with the correct module structure so all subsequent tasks have a stable home.

- [x] **1.1** Create `Cargo.toml` at the workspace root:
  - `[package]` with `name = "space_rocks"`, `edition = "2021"`
  - `[dependencies]` with `bevy = "0.15"`
- [x] **1.2** Create `src/main.rs` with a minimal `fn main()` that builds `App::new().add_plugins(DefaultPlugins).run()`
- [x] **1.3** Create `src/config.rs` as an empty module (`// TODO: constants`); declare `mod config;` in `main.rs`
- [x] **1.4** Create `src/components.rs` as an empty module (`// TODO: components`); declare `mod components;` in `main.rs`
- [x] **1.5** Create `src/plugins/mod.rs` with stub `pub mod ship; pub mod asteroid; pub mod bullet; pub mod collision;`; declare `mod plugins;` in `main.rs`
- [x] **1.6** Create stub plugin files that each define a zero-body plugin struct and `impl Plugin`:
  - `src/plugins/ship.rs` — `pub struct ShipPlugin;` + `impl Plugin for ShipPlugin { fn build(&self, _app: &mut App) {} }`
  - `src/plugins/asteroid.rs` — same pattern for `AsteroidPlugin`
  - `src/plugins/bullet.rs` — same pattern for `BulletPlugin`
  - `src/plugins/collision.rs` — same pattern for `CollisionPlugin`
- [x] **1.7** Register all four stub plugins in `main.rs` `App::new().add_plugins(DefaultPlugins).add_plugins((ShipPlugin, AsteroidPlugin, BulletPlugin, CollisionPlugin)).run()`

*Checkpoint: `cargo build` compiles cleanly with no errors. A blank Bevy window opens on `cargo run`. No gameplay yet.*

---

### 2. Shared Types — `components.rs` and `config.rs`

> Define every component and constant the plugins will share so downstream tasks have concrete types to reference. Requires group 1.

- [x] **2.1** Populate `src/config.rs` with all constants from the spec:
  ```rust
  pub const SHIP_ROTATION_SPEED: f32 = 3.0;
  pub const SHIP_THRUST: f32 = 200.0;
  pub const SHIP_MAX_SPEED: f32 = 400.0;
  pub const SHIP_DRAG: f32 = 0.98;
  pub const BULLET_SPEED: f32 = 500.0;
  pub const BULLET_LIFETIME: f32 = 1.2;
  pub const ASTEROID_INITIAL_COUNT: usize = 6;
  pub const ASTEROID_MIN_SPEED: f32 = 40.0;
  pub const ASTEROID_MAX_SPEED: f32 = 120.0;
  ```
- [x] **2.2** Add marker components to `src/components.rs`:
  - `#[derive(Component)] pub struct Player;`
  - `#[derive(Component)] pub struct Bullet;`
- [x] **2.3** Add `BulletLifetime` to `src/components.rs`:
  - `#[derive(Component)] pub struct BulletLifetime(pub Timer);`
- [x] **2.4** Add `AsteroidSize` enum and `Asteroid` component to `src/components.rs`:
  ```rust
  #[derive(Clone, Copy, PartialEq, Eq, Debug)]
  pub enum AsteroidSize { Large, Medium, Small }

  impl AsteroidSize {
      pub fn split(self) -> Option<AsteroidSize> { ... }  // Large→Medium, Medium→Small, Small→None
      pub fn radius(self) -> f32 { ... }                  // 48.0 / 24.0 / 12.0
  }

  #[derive(Component)]
  pub struct Asteroid { pub size: AsteroidSize }
  ```
- [x] **2.5** Add movement components to `src/components.rs`:
  - `#[derive(Component)] pub struct Velocity(pub Vec2);`
  - `#[derive(Component)] pub struct AngularVelocity(pub f32);`
  - `#[derive(Component, Default)] pub struct Thruster { pub active: bool }`
- [x] **2.6** Add `use bevy::prelude::*;` at the top of `components.rs` (needed for `Component`, `Timer`)
- [x] **2.7** Write unit tests in `src/components.rs` in a `#[cfg(test)]` module:
  - `fn large_splits_to_medium()` — assert `AsteroidSize::Large.split() == Some(AsteroidSize::Medium)`
  - `fn medium_splits_to_small()` — assert `AsteroidSize::Medium.split() == Some(AsteroidSize::Small)`
  - `fn small_does_not_split()` — assert `AsteroidSize::Small.split() == None`
  - `fn radius_large()` — assert `AsteroidSize::Large.radius() == 48.0`

*Checkpoint: `cargo test` passes all 4 unit tests. `cargo build` compiles cleanly.*

---

### 3. Camera

> Spawn the 2D camera needed to see anything rendered. Requires group 1.

- [x] **3.1** In `src/main.rs`, add a startup system `fn setup_camera(mut commands: Commands)` that spawns `commands.spawn(Camera2d)`
- [x] **3.2** Register `setup_camera` in the `App` via `.add_systems(Startup, setup_camera)`

*Checkpoint: `cargo run` opens a black window — no content yet, but Bevy's 2D camera is active.*

---

### 4. AsteroidPlugin — Spawn, Movement, Screen Wrap

> Implement the full asteroid lifecycle: initial spawn with random velocity, per-frame movement, and screen wrapping. Requires groups 2 and 3.

- [x] **4.1** Add `rand = "0.8"` to `Cargo.toml` `[dependencies]` (needed for random spawn positions/velocities)
- [x] **4.2** In `src/plugins/asteroid.rs`, add `use` imports: `bevy::prelude::*`, `bevy::window::PrimaryWindow`, `rand::Rng`, `crate::components::*`, `crate::config::*`
- [x] **4.3** Write `fn spawn_asteroids` startup system signature:
  ```rust
  fn spawn_asteroids(
      mut commands: Commands,
      mut meshes: ResMut<Assets<Mesh>>,
      mut materials: ResMut<Assets<ColorMaterial>>,
      window: Query<&Window, With<PrimaryWindow>>,
  )
  ```
  Implementation:
  - Get window width/height from the query (`window.single()`)
  - Loop `ASTEROID_INITIAL_COUNT` times, each iteration:
    - Pick a random edge position (top/bottom/left/right) using `rand::thread_rng()`
    - Pick a random velocity between `ASTEROID_MIN_SPEED` and `ASTEROID_MAX_SPEED` in a random direction
    - Pick a random angular velocity between `-1.5` and `1.5` rad/s
    - Spawn entity with: `Mesh2d(meshes.add(Circle::new(AsteroidSize::Large.radius())))`, `MeshMaterial2d(materials.add(Color::srgb(0.6, 0.6, 0.6)))`, `Transform::from_xyz(x, y, 0.0)`, `Asteroid { size: AsteroidSize::Large }`, `Velocity(vel)`, `AngularVelocity(ang_vel)`
- [x] **4.4** Write `fn move_asteroids` update system:
  ```rust
  fn move_asteroids(
      time: Res<Time>,
      mut query: Query<(&Velocity, &AngularVelocity, &mut Transform), With<Asteroid>>,
  )
  ```
  - Each frame: `transform.translation += vel.0.extend(0.0) * time.delta_secs()`
  - Each frame: `transform.rotate_z(ang_vel.0 * time.delta_secs())`
- [x] **4.5** Write `fn wrap_asteroids` update system:
  ```rust
  fn wrap_asteroids(
      window: Query<&Window, With<PrimaryWindow>>,
      mut query: Query<&mut Transform, With<Asteroid>>,
  )
  ```
  - Get half-width and half-height from the window
  - Wrap `transform.translation.x` and `.y` if they exceed `±half_width` / `±half_height`
- [x] **4.6** In `AsteroidPlugin::build`, register systems:
  - `.add_systems(Startup, spawn_asteroids)`
  - `.add_systems(Update, (move_asteroids, wrap_asteroids))`

*Checkpoint: `cargo run` shows 6 grey circles drifting and wrapping around the screen.*

---

### 5. ShipPlugin — Rotation, Thrust, Bullet Spawn

> Implement the player ship: spawn with triangle mesh, thrust-based movement with drag, rotation, screen wrap, and bullet spawning on Space. Requires groups 2 and 3.

- [x] **5.1** In `src/plugins/ship.rs`, add `use` imports: `bevy::prelude::*`, `bevy::window::PrimaryWindow`, `crate::components::*`, `crate::config::*`
- [x] **5.2** Write `fn spawn_ship` startup system:
  - Build a triangle `Mesh` pointing up (+Y) using `Triangle2d::new(Vec2::new(0.0, 20.0), Vec2::new(-12.0, -14.0), Vec2::new(12.0, -14.0))`
  - Spawn entity with: `Mesh2d(meshes.add(triangle))`, `MeshMaterial2d(materials.add(Color::srgb(0.8, 0.9, 1.0)))`, `Transform::from_xyz(0.0, 0.0, 1.0)`, `Player`, `Velocity(Vec2::ZERO)`, `Thruster::default()`
- [x] **5.3** Write `fn ship_rotation` update system:
  ```rust
  fn ship_rotation(
      time: Res<Time>,
      keys: Res<ButtonInput<KeyCode>>,
      mut query: Query<&mut Transform, With<Player>>,
  )
  ```
  - Left (A or ArrowLeft): `transform.rotate_z(SHIP_ROTATION_SPEED * time.delta_secs())`
  - Right (D or ArrowRight): `transform.rotate_z(-SHIP_ROTATION_SPEED * time.delta_secs())`
- [x] **5.4** Write `fn ship_thrust` update system:
  ```rust
  fn ship_thrust(
      time: Res<Time>,
      keys: Res<ButtonInput<KeyCode>>,
      mut query: Query<(&mut Velocity, &Transform, &mut Thruster), With<Player>>,
  )
  ```
  - If W or ArrowUp pressed:
    - Compute forward = `(transform.rotation * Vec3::Y).truncate()`
    - `velocity.0 += forward * SHIP_THRUST * time.delta_secs()`
    - Clamp magnitude to `SHIP_MAX_SPEED`
    - `thruster.active = true`
  - Else: `thruster.active = false`
  - Apply drag every frame: `velocity.0 *= SHIP_DRAG`
- [x] **5.5** Write `fn ship_movement` update system:
  ```rust
  fn ship_movement(
      time: Res<Time>,
      mut query: Query<(&Velocity, &mut Transform), With<Player>>,
  )
  ```
  - `transform.translation += velocity.0.extend(0.0) * time.delta_secs()`
- [x] **5.6** Write `fn wrap_ship` update system — same edge-wrapping logic as `wrap_asteroids` but queries `With<Player>`
- [x] **5.7** Write `fn ship_shoot` update system:
  ```rust
  fn ship_shoot(
      mut commands: Commands,
      mut meshes: ResMut<Assets<Mesh>>,
      mut materials: ResMut<Assets<ColorMaterial>>,
      keys: Res<ButtonInput<KeyCode>>,
      query: Query<(&Transform, &Velocity), With<Player>>,
  )
  ```
  - On `Space` just_pressed:
    - Get ship transform and velocity
    - Compute forward = `(transform.rotation * Vec3::Y).truncate()`
    - Spawn position = `transform.translation + (forward * 22.0).extend(0.0)` (past ship nose)
    - Bullet velocity = `ship_velocity.0 + forward * BULLET_SPEED`
    - Spawn entity with: `Mesh2d(meshes.add(Circle::new(3.0)))`, `MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.5)))`, `Transform::from_translation(spawn_pos)`, `Bullet`, `Velocity(bullet_vel)`, `BulletLifetime(Timer::from_seconds(BULLET_LIFETIME, TimerMode::Once))`
- [x] **5.8** In `ShipPlugin::build`, register systems:
  - `.add_systems(Startup, spawn_ship)`
  - `.add_systems(Update, (ship_rotation, ship_thrust, ship_movement, wrap_ship, ship_shoot))`

*Checkpoint: `cargo run` — ship appears at center, rotates with A/D, thrusts with W, wraps at edges, fires yellow dots on Space.*

---

### 6. BulletPlugin — Movement, Lifetime, Despawn

> Move bullets each frame, tick their lifetime timers, and despawn them when expired. Requires group 2.

- [x] **6.1** In `src/plugins/bullet.rs`, add `use` imports: `bevy::prelude::*`, `crate::components::*`, `crate::config::*`
- [x] **6.2** Write `fn move_bullets` update system:
  ```rust
  fn move_bullets(
      time: Res<Time>,
      mut query: Query<(&Velocity, &mut Transform), With<Bullet>>,
  )
  ```
  - `transform.translation += velocity.0.extend(0.0) * time.delta_secs()`
- [x] **6.3** Write `fn bullet_lifetime` update system:
  ```rust
  fn bullet_lifetime(
      mut commands: Commands,
      time: Res<Time>,
      mut query: Query<(Entity, &mut BulletLifetime)>,
  )
  ```
  - Tick timer: `lifetime.0.tick(time.delta())`
  - If `lifetime.0.finished()`: `commands.entity(entity).despawn()`
- [x] **6.4** In `BulletPlugin::build`, register:
  - `.add_systems(Update, (move_bullets, bullet_lifetime))`

*Checkpoint: `cargo run` — bullets travel straight and disappear after ~1.2 seconds.*

---

### 7. CollisionPlugin — Detection and Asteroid Splitting

> Detect bullet↔asteroid collisions each frame, despawn both, and spawn two child asteroids (or none for Small). Requires groups 4, 5, and 6.

- [x] **7.1** In `src/plugins/collision.rs`, add `use` imports: `bevy::prelude::*`, `crate::components::*`, `crate::config::*`
- [x] **7.2** Write `fn bullet_asteroid_collision` update system:
  ```rust
  fn bullet_asteroid_collision(
      mut commands: Commands,
      mut meshes: ResMut<Assets<Mesh>>,
      mut materials: ResMut<Assets<ColorMaterial>>,
      bullets: Query<(Entity, &Transform), With<Bullet>>,
      asteroids: Query<(Entity, &Transform, &Asteroid)>,
  )
  ```
  Implementation:
  - For each `(bullet_entity, bullet_transform)` in `bullets`:
    - For each `(asteroid_entity, asteroid_transform, asteroid)` in `asteroids`:
      - Compute distance between `bullet_transform.translation` and `asteroid_transform.translation`
      - If `distance < asteroid.size.radius() + 3.0` (bullet radius):
        - `commands.entity(bullet_entity).despawn()`
        - `commands.entity(asteroid_entity).despawn()`
        - Call `spawn_split_asteroids(&mut commands, &mut meshes, &mut materials, asteroid_transform.translation, asteroid.size)`
        - `break` inner loop (bullet is consumed)
- [x] **7.3** Write helper `fn spawn_split_asteroids(commands, meshes, materials, position: Vec3, size: AsteroidSize)`:
  - If `size.split()` is `None`: return (Small — fully destroyed)
  - Otherwise let `child_size = size.split().unwrap()`
  - Spawn 2 child asteroids at `position` with:
    - Diverging velocities: pick a random base direction, then offset `±30°` for the two children, scaled to a speed between `ASTEROID_MIN_SPEED` and `ASTEROID_MAX_SPEED`
    - `Mesh2d(meshes.add(Circle::new(child_size.radius())))`, `MeshMaterial2d(materials.add(Color::srgb(0.6, 0.6, 0.6)))`, `Transform::from_translation(position)`, `Asteroid { size: child_size }`, `Velocity(vel)`, `AngularVelocity(ang_vel)`
- [x] **7.4** In `CollisionPlugin::build`, register:
  - `.add_systems(Update, bullet_asteroid_collision)`
  - Ensure this runs after `move_bullets` and `move_asteroids` — use `.after(move_bullets)` and `.after(move_asteroids)` or rely on set ordering

*Checkpoint: `cargo run` — shooting a large asteroid splits it into two medium ones; shooting medium splits to small; shooting small destroys it. Bullets consumed on hit.*

---

## Open Questions

- [ ] **Ship controls** — spec says "TBD: confirm thrust-based (classic) vs. direct movement." **Assumption:** thrust-based (classic Asteroids) as described in the Behavior section. Plan is written to this assumption.
- [ ] **Rendering** — spec says "TBD: meshes vs. sprite assets." **Assumption:** colored `Mesh2d` shapes, no art assets. Bevy 0.15 required-components API used throughout (`Mesh2d` + `MeshMaterial2d<ColorMaterial>`).

---

## Notes for Execute

**Bevy version:** `0.15`. Do not use bundle types (`SpriteBundle`, `MaterialMesh2dBundle`) — they were removed/deprecated. Use the required-components API:
```rust
commands.spawn((
    Mesh2d(meshes.add(Circle::new(radius))),
    MeshMaterial2d(materials.add(Color::srgb(r, g, b))),
    Transform::from_xyz(x, y, z),
));
```

**`Camera2d` spawn (Bevy 0.15):** `commands.spawn(Camera2d)` — no bundle needed.

**Window query pattern:**
```rust
fn my_system(window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.single();
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;
}
```
Import: `use bevy::window::PrimaryWindow;`

**Ship facing direction:** The ship mesh points up (+Y in local space). Forward vector in world space is `(transform.rotation * Vec3::Y).truncate()`.

**`rand` usage:** `rand::thread_rng().gen_range(min..max)` for f32 ranges. Import `use rand::Rng;`.

**Screen wrap logic** (same for ship, asteroids — no shared system per spec):
```rust
if transform.translation.x > half_w { transform.translation.x = -half_w; }
if transform.translation.x < -half_w { transform.translation.x = half_w; }
// same for y / half_h
```

**Collision double-despawn guard:** When two bullets hit the same asteroid in one frame, `commands.entity().despawn()` on an already-despawned entity will panic in Bevy 0.15. Guard with a `HashSet` of already-hit asteroid entities within the system, or use `try_despawn()` if available. The simplest safe approach: collect hits first, then despawn, deduplicating by asteroid entity.

**System ordering for collision:** `bullet_asteroid_collision` should be ordered after `move_bullets` and `move_asteroids` within the `Update` schedule. Use:
```rust
app.add_systems(Update, bullet_asteroid_collision
    .after(move_bullets)
    .after(move_asteroids));
```
This requires that `move_bullets` and `move_asteroids` be public functions (or re-exported) so `collision.rs` can reference them — or use `SystemSet` labels instead.

**Alternative ordering approach (simpler):** Use Bevy's system set ordering with named sets rather than referencing foreign functions directly. Define `MovementSet` and `CollisionSet` in `main.rs` or `components.rs`:
```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet { Movement, Collision, Despawn }
```
Then in `main.rs`:
```rust
app.configure_sets(Update, (GameSet::Movement, GameSet::Collision, GameSet::Despawn).chain());
```
And tag systems: `.add_systems(Update, move_asteroids.in_set(GameSet::Movement))`.
