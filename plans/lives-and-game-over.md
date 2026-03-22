# Plan: Lives and Game Over

**Spec:** `specs/lives-and-game-over.md`
**Status:** In progress
**Created:** 2026-03-22

---

## Overview

Build the full game loop: ship-asteroid collision, a three-life system, and a four-state machine (Attract → Playing → Dead → GameOver). The work proceeds bottom-up — shared types first, then the new plugins (`GameStatePlugin`, `HudPlugin`), then modifications to existing plugins (`ShipPlugin`, `AsteroidPlugin`, `CollisionPlugin`), then wiring. A new `SpawnShipEvent { invincible: bool }` event centralises all ship spawning in `ShipPlugin` so mesh setup stays in one place regardless of whether a spawn comes from a reset or a respawn. The key integration risk is the `bevy_state` feature being absent from `Cargo.toml` — that must be fixed first.

---

## Prerequisites

- [x] `plans/project-bootstrap.md` — complete ✓

---

## Tasks

### 1. Cargo.toml — Enable `bevy_state` Feature

> Bevy 0.15 with `default-features = false` does not include the state machine machinery; `#[derive(States)]` will fail to compile without it.

- [x] **1.1** In `Cargo.toml`, add `"bevy_state"` to the bevy features list alongside the existing ones.

*Checkpoint: `cargo check` compiles cleanly. No other changes yet.*

---

### 2. Shared Types — `components.rs` and `config.rs`

> Add all new components, resources, and events so every downstream task has concrete types to reference. Requires group 1.

- [x] **2.1** Add `GameState` enum to `src/components.rs`:
  ```rust
  #[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
  pub enum GameState {
      #[default]
      Attract,
      Playing,
      Dead,
      GameOver,
  }
  ```

- [x] **2.2** Add resource types to `src/components.rs`:
  ```rust
  #[derive(Resource)]
  pub struct Lives(pub u32);

  #[derive(Resource)]
  pub struct Score(pub u32);
  ```

- [x] **2.3** Add `Invincible` component to `src/components.rs` with two timers — one for the total window, one to track blink phase:
  ```rust
  #[derive(Component)]
  pub struct Invincible {
      pub timer: Timer,        // total invincibility duration
      pub blink_timer: Timer,  // toggles visibility every SHIP_BLINK_INTERVAL_SECS
  }
  ```

- [x] **2.4** Add `RespawnTimer` resource to `src/components.rs`:
  ```rust
  #[derive(Resource)]
  pub struct RespawnTimer(pub Timer);
  ```

- [x] **2.5** Add events to `src/components.rs`:
  ```rust
  #[derive(Event)]
  pub struct ShipDestroyedEvent;

  #[derive(Event)]
  pub struct ResetGameEvent;

  #[derive(Event)]
  pub struct SpawnShipEvent {
      pub invincible: bool,
  }
  ```

- [x] **2.6** Add HUD marker components to `src/components.rs`:
  ```rust
  #[derive(Component)]
  pub struct HudLivesText;

  #[derive(Component)]
  pub struct HudScoreText;

  #[derive(Component)]
  pub struct GameOverText;

  #[derive(Component)]
  pub struct PressAnyKeyText;
  ```

- [x] **2.7** Add constants to `src/config.rs`:
  ```rust
  pub const SHIP_RADIUS: f32 = 16.0;
  pub const PLAYER_STARTING_LIVES: u32 = 3;
  pub const SHIP_RESPAWN_DELAY_SECS: f32 = 1.5;
  pub const SHIP_INVINCIBILITY_SECS: f32 = 2.0;
  pub const SHIP_BLINK_INTERVAL_SECS: f32 = 0.1;
  ```

*Checkpoint: `cargo check` compiles cleanly. All new types are defined, nothing is wired yet.*

---

### 3. Stub New Plugin Files

> Create minimal-but-compiling stubs for the two new plugin files so the module tree is valid before filling in the systems. Requires group 2.

- [ ] **3.1** Create `src/plugins/game_state.rs` with a stub:
  ```rust
  use bevy::prelude::*;
  use crate::components::*;
  use crate::config::*;

  pub struct GameStatePlugin;

  impl Plugin for GameStatePlugin {
      fn build(&self, _app: &mut App) {}
  }
  ```

- [ ] **3.2** Create `src/plugins/hud.rs` with a stub:
  ```rust
  use bevy::prelude::*;
  use crate::components::*;

  pub struct HudPlugin;

  impl Plugin for HudPlugin {
      fn build(&self, _app: &mut App) {}
  }
  ```

- [ ] **3.3** In `src/plugins/mod.rs`, add `pub mod game_state;` and `pub mod hud;`.

*Checkpoint: `cargo check` compiles cleanly with new modules visible.*

---

### 4. Wire App — `main.rs`

> Register `GameState`, the two new plugins, and the new events in the `App`. Requires group 3.

- [ ] **4.1** In `src/main.rs`, add imports:
  ```rust
  use plugins::{game_state::GameStatePlugin, hud::HudPlugin};
  use components::GameState;
  ```

- [ ] **4.2** Add `.init_state::<GameState>()` to the `App` builder (before `.run()`).

- [ ] **4.3** Add `GameStatePlugin` and `HudPlugin` to `.add_plugins(...)` tuple alongside existing plugins.

*Checkpoint: `cargo run` opens a window, asteroids move (Attract state), no ship spawns — correct for Attract.*

---

### 5. `GameStatePlugin` — Full Implementation

> Implement all state-transition systems: startup resources, attract/game-over input, ship-destroyed handler, respawn timer, and invincibility tick. Requires group 4.

- [ ] **5.1** In `GameStatePlugin::build`, register events:
  ```rust
  app.add_event::<ShipDestroyedEvent>()
     .add_event::<ResetGameEvent>();
  ```
  *(Note: `SpawnShipEvent` is registered by `ShipPlugin` — see group 6.)*

- [ ] **5.2** Write `fn insert_game_resources(mut commands: Commands)` startup system:
  - `commands.insert_resource(Lives(PLAYER_STARTING_LIVES));`
  - `commands.insert_resource(Score(0));`
  - Register in `build` with `.add_systems(Startup, insert_game_resources)`.

- [ ] **5.3** Write `fn handle_attract_input` system:
  ```rust
  fn handle_attract_input(
      keys: Res<ButtonInput<KeyCode>>,
      mut reset_events: EventWriter<ResetGameEvent>,
      mut next_state: ResMut<NextState<GameState>>,
      mut lives: ResMut<Lives>,
      mut score: ResMut<Score>,
  )
  ```
  - If `keys.get_just_pressed().next().is_some()`: set `lives.0 = PLAYER_STARTING_LIVES`, `score.0 = 0`, fire `ResetGameEvent`, `next_state.set(GameState::Playing)`.
  - Register with `.add_systems(Update, handle_attract_input.run_if(in_state(GameState::Attract)))`.

- [ ] **5.4** Write `fn handle_game_over_input` system — identical logic to `handle_attract_input`. Register with `.run_if(in_state(GameState::GameOver))`.

- [ ] **5.5** Write `fn on_ship_destroyed` system:
  ```rust
  fn on_ship_destroyed(
      mut events: EventReader<ShipDestroyedEvent>,
      mut lives: ResMut<Lives>,
      mut commands: Commands,
      mut next_state: ResMut<NextState<GameState>>,
      state: Res<State<GameState>>,
  )
  ```
  - For each event: if `*state.get() != GameState::Playing` log warning and skip (idempotency guard).
  - `lives.0 = lives.0.saturating_sub(1)`
  - If `lives.0 > 0`: `commands.insert_resource(RespawnTimer(Timer::from_seconds(SHIP_RESPAWN_DELAY_SECS, TimerMode::Once)))`, `next_state.set(GameState::Dead)`.
  - Else: `next_state.set(GameState::GameOver)`.
  - Register with `.add_systems(Update, on_ship_destroyed)`.

- [ ] **5.6** Write `fn tick_respawn_timer` system:
  ```rust
  fn tick_respawn_timer(
      mut commands: Commands,
      time: Res<Time>,
      mut timer: ResMut<RespawnTimer>,
      mut spawn_events: EventWriter<SpawnShipEvent>,
      mut next_state: ResMut<NextState<GameState>>,
  )
  ```
  - `timer.0.tick(time.delta())`
  - If `timer.0.finished()`: `commands.remove_resource::<RespawnTimer>()`, fire `SpawnShipEvent { invincible: true }`, `next_state.set(GameState::Playing)`.
  - Register with `.add_systems(Update, tick_respawn_timer.run_if(in_state(GameState::Dead)))`.

- [ ] **5.7** Write `fn tick_invincibility` system:
  ```rust
  fn tick_invincibility(
      mut commands: Commands,
      time: Res<Time>,
      mut query: Query<(Entity, &mut Invincible, &mut Visibility), With<Player>>,
  )
  ```
  - `let Ok((entity, mut inv, mut vis)) = query.get_single_mut() else { return; }`
  - Tick `inv.timer` and `inv.blink_timer`.
  - If `inv.blink_timer.just_finished()`: toggle `*vis` between `Visibility::Visible` and `Visibility::Hidden`.
  - If `inv.timer.finished()`: `*vis = Visibility::Visible`, `commands.entity(entity).remove::<Invincible>()`.
  - Register with `.add_systems(Update, tick_invincibility.run_if(in_state(GameState::Playing)).in_set(GameSet::Movement))`.

*Checkpoint: `cargo check` compiles cleanly. Logic is in place; full integration requires group 6.*

---

### 6. Modify `ShipPlugin`

> Remove the Startup spawn, centralise all ship spawning behind `SpawnShipEvent`, and handle `ResetGameEvent` to despawn the old ship. Requires group 5.

- [ ] **6.1** In `ShipPlugin::build`:
  - Remove `.add_systems(Startup, spawn_ship)`.
  - Register `app.add_event::<SpawnShipEvent>()`.
  - Add `.add_systems(Update, spawn_ship_from_event)`.
  - Add `.add_systems(Update, handle_ship_reset)`.

- [ ] **6.2** Rename existing `fn spawn_ship` to `fn spawn_ship_entity(commands, meshes, materials)` (a plain helper, not a system) and add an `invincible: bool` parameter. When `invincible` is true, attach:
  ```rust
  Invincible {
      timer: Timer::from_seconds(SHIP_INVINCIBILITY_SECS, TimerMode::Once),
      blink_timer: Timer::from_seconds(SHIP_BLINK_INTERVAL_SECS, TimerMode::Repeating),
  }
  ```

- [ ] **6.3** Write `fn spawn_ship_from_event` system:
  ```rust
  fn spawn_ship_from_event(
      mut commands: Commands,
      mut meshes: ResMut<Assets<Mesh>>,
      mut materials: ResMut<Assets<ColorMaterial>>,
      mut events: EventReader<SpawnShipEvent>,
  )
  ```
  - For each event: call `spawn_ship_entity(&mut commands, &mut meshes, &mut materials, event.invincible)`.

- [ ] **6.4** Write `fn handle_ship_reset` system:
  ```rust
  fn handle_ship_reset(
      mut events: EventReader<ResetGameEvent>,
      mut commands: Commands,
      player: Query<Entity, With<Player>>,
      mut spawn_events: EventWriter<SpawnShipEvent>,
  )
  ```
  - For each `ResetGameEvent`: if `player.get_single()` is `Ok(entity)`, `commands.entity(entity).despawn()`.
  - Fire `SpawnShipEvent { invincible: false }`.

- [ ] **6.5** Gate ship movement systems with `.run_if(in_state(GameState::Playing))` so the ship stops accepting input while `Dead` or `GameOver`.

*Checkpoint: `cargo run` — no ship on attract screen. Pressing any key spawns a fresh ship and asteroids reset.*

---

### 7. Modify `AsteroidPlugin`

> Handle `ResetGameEvent` to despawn all asteroids and spawn a fresh set. Requires group 4.

- [ ] **7.1** Write `fn handle_asteroid_reset` system:
  ```rust
  fn handle_asteroid_reset(
      mut events: EventReader<ResetGameEvent>,
      mut commands: Commands,
      mut meshes: ResMut<Assets<Mesh>>,
      mut materials: ResMut<Assets<ColorMaterial>>,
      asteroids: Query<Entity, With<Asteroid>>,
      window: Query<&Window, With<PrimaryWindow>>,
  )
  ```
  - For each `ResetGameEvent`:
    - Despawn all entities in `asteroids` query.
    - Call the existing `spawn_asteroids_into` helper (see 7.2) to spawn a fresh set.

- [ ] **7.2** Extract the body of `fn spawn_asteroids` into a helper `fn spawn_asteroids_into(commands, meshes, materials, window)` that both the `Startup` system and `handle_asteroid_reset` call.

- [ ] **7.3** Register `handle_asteroid_reset` in `AsteroidPlugin::build`:
  ```rust
  .add_systems(Update, handle_asteroid_reset)
  ```

*Checkpoint: `cargo run` — pressing any key from attract respawns a fresh set of 6 asteroids.*

---

### 8. Extend `CollisionPlugin` — Ship-Asteroid Collision

> Add `ship_asteroid_collision` system that detects ship↔asteroid overlap, despawns the ship, and fires `ShipDestroyedEvent`. Requires groups 5 and 6.

- [ ] **8.1** Write `fn ship_asteroid_collision` system:
  ```rust
  fn ship_asteroid_collision(
      mut commands: Commands,
      mut destroyed: EventWriter<ShipDestroyedEvent>,
      ship: Query<(Entity, &Transform), (With<Player>, Without<Invincible>)>,
      asteroids: Query<(&Transform, &Asteroid)>,
  )
  ```
  - `let Ok((ship_entity, ship_transform)) = ship.get_single() else { return; }`
  - For each `(asteroid_transform, asteroid)` in `asteroids`:
    - If `circles_are_colliding(ship_transform.translation.truncate(), asteroid_transform.translation.truncate(), SHIP_RADIUS, asteroid.size.radius())`:
      - `commands.entity(ship_entity).despawn()`
      - `destroyed.send(ShipDestroyedEvent)`
      - `return` (one hit is enough)

- [ ] **8.2** Register in `CollisionPlugin::build`:
  ```rust
  .add_systems(Update, ship_asteroid_collision
      .in_set(GameSet::Collision)
      .run_if(in_state(GameState::Playing)))
  ```

*Checkpoint: `cargo run` — ship touching asteroid despawns it; lives decrement shown in HUD (once HudPlugin is wired); after 3 deaths Game Over appears.*

---

### 9. `HudPlugin` — Full Implementation

> Spawn HUD text and overlay text, update them from resources, and show/hide based on state. Requires groups 4 and 5.

- [ ] **9.1** Write `fn spawn_hud` startup system that spawns two UI text entities (parented under a root `Node`):
  - Lives text entity: `(Text::new("Lives: 3"), TextFont { font_size: 24.0, ..default() }, TextColor(Color::WHITE), Node { position_type: PositionType::Absolute, top: Val::Px(10.0), left: Val::Px(10.0), ..default() }, HudLivesText)`
  - Score text entity: `(Text::new("Score: 0"), TextFont { font_size: 24.0, ..default() }, TextColor(Color::WHITE), Node { position_type: PositionType::Absolute, top: Val::Px(10.0), right: Val::Px(10.0), ..default() }, HudScoreText)`

- [ ] **9.2** Write `fn spawn_overlay_text` startup system that spawns two overlay text entities centred on screen:
  - "Press Any Key to Start" — visible on startup: `(Text::new("Press Any Key to Start"), TextFont { font_size: 32.0, ..default() }, TextColor(Color::WHITE), Node { position_type: PositionType::Absolute, top: Val::Percent(60.0), left: Val::Percent(50.0), ..default() }, PressAnyKeyText)`
  - "Game Over" — starts hidden (`Visibility::Hidden`): same layout at `top: Val::Percent(45.0)` with `GameOverText`

- [ ] **9.3** Write `fn update_hud` system:
  ```rust
  fn update_hud(
      lives: Res<Lives>,
      score: Res<Score>,
      mut lives_text: Query<&mut Text, With<HudLivesText>>,
      mut score_text: Query<&mut Text, (With<HudScoreText>, Without<HudLivesText>)>,
  )
  ```
  - Update text strings: `format!("Lives: {}", lives.0)` and `format!("Score: {}", score.0)`.
  - Register with `.add_systems(Update, update_hud)`.

- [ ] **9.4** Write visibility toggle systems (each is a one-liner using `get_single_mut` + `if let Ok`):
  - `fn show_game_over_text` → `Visibility::Visible` on `GameOverText` — register with `OnEnter(GameState::GameOver)`
  - `fn hide_game_over_text` → `Visibility::Hidden` on `GameOverText` — register with `OnEnter(GameState::Playing)`
  - `fn show_press_any_key` → `Visibility::Visible` on `PressAnyKeyText` — register with `OnEnter(GameState::Attract)` and `OnEnter(GameState::GameOver)`
  - `fn hide_press_any_key` → `Visibility::Hidden` on `PressAnyKeyText` — register with `OnEnter(GameState::Playing)`

- [ ] **9.5** Register `spawn_hud` and `spawn_overlay_text` in `HudPlugin::build` with `.add_systems(Startup, ...)`.

*Checkpoint: `cargo run` — HUD shows "Lives: 3" and "Score: 0". "Press Any Key to Start" visible on attract. "Game Over" visible after 3 deaths. Pressing any key resets.*

---

### 10. Unit Tests

> Add pure-logic unit tests that require no Bevy app. Requires groups 2 and 5.

- [ ] **10.1** In `src/components.rs` `#[cfg(test)]` module, add:
  ```rust
  #[test]
  fn lives_decrement_reaches_zero() {
      let mut lives = Lives(1);
      lives.0 = lives.0.saturating_sub(1);
      assert_eq!(lives.0, 0);
  }

  #[test]
  fn lives_saturating_sub_does_not_underflow() {
      let mut lives = Lives(0);
      lives.0 = lives.0.saturating_sub(1);
      assert_eq!(lives.0, 0);
  }
  ```

- [ ] **10.2** In `src/config.rs` or a `#[cfg(test)]` module within it, add constant-presence assertions:
  ```rust
  #[test]
  fn respawn_and_invincibility_constants_are_positive() {
      assert!(SHIP_RESPAWN_DELAY_SECS > 0.0);
      assert!(SHIP_INVINCIBILITY_SECS > 0.0);
      assert!(SHIP_BLINK_INTERVAL_SECS > 0.0);
      assert!(SHIP_BLINK_INTERVAL_SECS < SHIP_INVINCIBILITY_SECS);
  }
  ```

- [ ] **10.3** Run `cargo test` and confirm all existing tests still pass alongside the new ones.

*Checkpoint: `cargo test` — all unit tests pass. `cargo run` — full game loop works end to end.*

---

## Open Questions

- None — spec states "all decisions made."

---

## Notes for Execute

**`bevy_state` feature (Group 1 is critical):** `default-features = false` in `Cargo.toml` excludes the state machine. `#[derive(States)]` will fail to compile without `"bevy_state"` in the features list. Do this first.

**`init_state` vs `insert_state`:** Use `.init_state::<GameState>()` in `main.rs`. This uses the `#[default]` variant (`Attract`) as the starting state. Do not use `.insert_state(...)` unless you need a non-default starting state.

**`run_if(in_state(...))` import:** Bring in `use bevy::prelude::*` — `in_state` is re-exported there in Bevy 0.15.

**`circles_are_colliding` visibility:** The helper is currently `fn circles_are_colliding` (private) in `collision.rs`. Since `ship_asteroid_collision` lives in the same file, no visibility change is needed.

**`SpawnShipEvent` ownership:** Registered by `ShipPlugin`. `GameStatePlugin`'s `tick_respawn_timer` writes to it — this cross-plugin event usage is normal in Bevy and works as long as the event is registered before any system runs.

**Invincibility blink toggle:** Track the previous visibility state inside `tick_invincibility` using a local match on `*vis`:
```rust
if inv.blink_timer.just_finished() {
    *vis = match *vis {
        Visibility::Visible => Visibility::Hidden,
        _ => Visibility::Visible,
    };
}
```

**`on_ship_destroyed` idempotency:** Multiple asteroids can overlap the ship in one frame. The `Without<Invincible>` filter on the query prevents re-entry once despawned, but the `ShipDestroyedEvent` could still fire twice if the ship entity is gone but event processing happens before the query cache is updated. Guard with the `state` check: only act when state is `Playing`.

**Bevy 0.15 UI text API:** No bundle. Compose components:
```rust
commands.spawn((
    Text::new("Lives: 3"),
    TextFont { font_size: 24.0, ..default() },
    TextColor(Color::WHITE),
    Node { position_type: PositionType::Absolute, top: Val::Px(10.0), left: Val::Px(10.0), ..default() },
    HudLivesText,
));
```

**`despawn` vs `despawn_recursive`:** Ship and asteroid entities have no children, so `commands.entity(e).despawn()` is fine. Use `despawn_recursive()` only if you add child entities (e.g., thruster flame) in a future spec.

**Ordering in `main.rs` `add_plugins` tuple:** Bevy processes plugin `build` methods in order. Add `GameStatePlugin` before `HudPlugin` so events are registered before HUD systems try to read resources they depend on.
