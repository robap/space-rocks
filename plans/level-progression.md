# Plan: Level Progression

**Spec:** `specs/level-progression.md`
**Status:** Complete ✓
**Created:** 2026-03-22

---

## Overview

Build bottom-up: types and constants first, then the new `LevelPlugin` skeleton (so it compiles), then the `AsteroidPlugin` split (despawn/spawn separated via event), then the full level logic, then HUD additions, then unit tests. The `LevelTransition` state naturally freezes ship/bullet systems (all already gated on `Playing`) without any extra guards.

Key integration point: `handle_asteroid_reset` in `AsteroidPlugin` currently both despawns and spawns. It must be split — despawn stays on `ResetGameEvent`, spawn moves to a new handler for `SpawnLevelEvent`.

---

## Prerequisites

- None

---

## Tasks

### 1. Types, components, and constants

> Lay the data foundation in `components.rs` and `config.rs` so all downstream tasks compile. No logic yet.

- [x] **1.1** Add `LevelTransition` variant to `GameState` enum in `src/components.rs` (between `Dead` and `GameOver`)
- [x] **1.2** Add `#[derive(Resource)] pub struct Level { pub number: u32, pub active: bool }` to `src/components.rs`
- [x] **1.3** Add `#[derive(Event)] pub struct SpawnLevelEvent { pub count: usize }` to `src/components.rs`
- [x] **1.4** Add `#[derive(Resource)] pub struct LevelTransitionTimer(pub Timer)` to `src/components.rs`
- [x] **1.5** Add `#[derive(Component)] pub struct HudLevelText;` and `#[derive(Component)] pub struct LevelReadyText;` to `src/components.rs`
- [x] **1.6** Add to `src/config.rs`:
  - `pub const ASTEROID_COUNT_INCREMENT: usize = 2;`
  - `pub const ASTEROID_MAX_COUNT: usize = 12;`
  - `pub const LEVEL_TRANSITION_SECS: f32 = 5.0;`


*Checkpoint: project compiles cleanly. No new behaviour — only new types. All existing tests pass.*

---

### 2. LevelPlugin skeleton

> Create `src/plugins/level.rs` with stub systems so it wires into the app and compiles before any logic is filled in.

- [x] **2.1** Create `src/plugins/level.rs` with:
  - `pub struct LevelPlugin;`
  - `impl Plugin for LevelPlugin` with `build` that registers `SpawnLevelEvent` via `add_event::<SpawnLevelEvent>()` and calls `add_systems` for all systems listed below (stubs only)
- [x] **2.2** Add stub `fn insert_level_resource(mut commands: Commands)` — empty body
- [x] **2.3** Add stub `fn detect_level_clear()` — empty body
- [x] **2.4** Add stub `fn start_level_transition()` — empty body
- [x] **2.5** Add stub `fn tick_level_transition()` — empty body
- [x] **2.6** Add stub `fn on_reset_game()` — empty body
- [x] **2.7** Add `pub mod level;` to `src/plugins/mod.rs`
- [x] **2.8** Import `LevelPlugin` in `src/main.rs` and add it to the `add_plugins(...)` call

*Checkpoint: compiles cleanly, all existing tests pass. `LevelPlugin` is registered but does nothing.*

---

### 3. AsteroidPlugin: split reset handler, add spawn-level handler

> Separate despawn (on `ResetGameEvent`) from spawn (on `SpawnLevelEvent`) in `AsteroidPlugin`. Requires Group 1 complete.

- [x] **3.1** In `src/plugins/asteroid.rs`, rename `handle_asteroid_reset` to reflect its new despawn-only role (keep the name or rename to `despawn_asteroids_on_reset` — either is fine, pick one and be consistent). Remove the `spawn_asteroids_into` call from this function; keep only the entity despawn loop.
- [x] **3.2** Add new system `fn handle_spawn_level` in `src/plugins/asteroid.rs`:
  - Parameters: `EventReader<SpawnLevelEvent>`, `Commands`, `ResMut<Assets<Mesh>>`, `ResMut<Assets<ColorMaterial>>`, `Query<&Window, With<PrimaryWindow>>`, `ResMut<Level>`
  - For each event: call `spawn_asteroids_into`, then set `level.active = true`
- [x] **3.3** Register `handle_spawn_level` in `AsteroidPlugin::build` under `add_systems(Update, handle_spawn_level)`
- [x] **3.4** Remove the `Startup` registration of `spawn_asteroids` from `AsteroidPlugin::build` — initial spawn is now driven by `LevelPlugin` via `SpawnLevelEvent` on game start

*Checkpoint: compiles cleanly. Note: the game will start with an empty asteroid field until Group 4 wires up `on_reset_game` and `insert_level_resource`. That is expected at this stage.*

---

### 4. LevelPlugin: implement all systems

> Fill in the stub systems from Group 2. Requires Groups 1–3 complete.

- [x] **4.1** Implement `insert_level_resource`:
  - `commands.insert_resource(Level { number: 1, active: false })`
  - Also fire initial `SpawnLevelEvent { count: ASTEROID_INITIAL_COUNT }` — wait, this runs at `Startup` and `LevelPlugin` hasn't entered `Playing` yet. Instead, rely on `on_reset_game` for initial spawn triggered by `handle_attract_input`. No spawn at Startup.
  - Body: `commands.insert_resource(Level { number: 1, active: false });`

- [x] **4.2** Implement `fn asteroid_count_for_level(level: u32) -> usize` as a free function (not a system) in `src/plugins/level.rs`:
  ```rust
  fn asteroid_count_for_level(level: u32) -> usize {
      let count = ASTEROID_INITIAL_COUNT + (level as usize - 1) * ASTEROID_COUNT_INCREMENT;
      count.min(ASTEROID_MAX_COUNT)
  }
  ```

- [x] **4.3** Implement `detect_level_clear`:
  - Parameters: `asteroids: Query<(), With<Asteroid>>`, `mut level: ResMut<Level>`, `mut next_state: ResMut<NextState<GameState>>`
  - If `level.active && asteroids.is_empty()`: set `level.active = false`, increment `level.number`, `next_state.set(GameState::LevelTransition)`
  - Register with `.run_if(in_state(GameState::Playing))`

- [x] **4.4** Implement `start_level_transition`:
  - Parameters: `mut commands: Commands`
  - Body: `commands.insert_resource(LevelTransitionTimer(Timer::from_seconds(LEVEL_TRANSITION_SECS, TimerMode::Once)))`
  - Register with `add_systems(OnEnter(GameState::LevelTransition), start_level_transition)`

- [x] **4.5** Implement `tick_level_transition`:
  - Parameters: `mut commands: Commands`, `time: Res<Time>`, `mut timer: ResMut<LevelTransitionTimer>`, `mut spawn_events: EventWriter<SpawnLevelEvent>`, `mut next_state: ResMut<NextState<GameState>>`, `level: Res<Level>`
  - Tick timer. If finished: remove `LevelTransitionTimer`, fire `SpawnLevelEvent { count: asteroid_count_for_level(level.number) }`, `next_state.set(GameState::Playing)`
  - Register with `.run_if(in_state(GameState::LevelTransition))`

- [x] **4.6** Implement `on_reset_game`:
  - Parameters: `mut events: EventReader<ResetGameEvent>`, `mut level: ResMut<Level>`, `mut commands: Commands`, `mut spawn_events: EventWriter<SpawnLevelEvent>`
  - For each event: reset `*level = Level { number: 1, active: false }`, `commands.remove_resource::<LevelTransitionTimer>()`, fire `SpawnLevelEvent { count: ASTEROID_INITIAL_COUNT }`
  - Register unconditionally (no state gate) — must handle reset from any state including `LevelTransition`

*Checkpoint: compiles cleanly. Asteroids spawn on game start (triggered by `ResetGameEvent` from `handle_attract_input`). Level clear detection fires and transitions to `LevelTransition`. After 5s, new asteroids spawn. Level number increments correctly. All existing tests pass.*

---

### 5. HUD additions

> Add level number display (top-center) and "Level X — Get Ready" overlay. Requires Groups 1 and 4 complete (needs `Level` resource and `LevelTransition` state).

- [x] **5.1** Add level number text entity in `spawn_hud` in `src/plugins/hud.rs`:
  - Position: top-center (`position_type: Absolute`, `top: Val::Px(10.0)`, `width: Val::Percent(100.0)`, `justify_content: JustifyContent::Center`)
  - Initial text: `"Level: 1"`
  - Components: `HudLevelText`

- [x] **5.2** Add "Level X — Get Ready" overlay entity in `spawn_overlay_text` in `src/plugins/hud.rs`:
  - Position: center-screen (e.g. `top: Val::Percent(40.0)`, full width, centered)
  - Initial text: `"Level 1 — Get Ready"` (will be updated before shown)
  - Initial visibility: `Visibility::Hidden`
  - Components: `LevelReadyText`

- [x] **5.3** Update `update_hud` in `src/plugins/hud.rs` to also read `Res<Level>` and update `HudLevelText`:
  - Add `level: Res<Level>` parameter
  - Add `level_text: Query<&mut Text, (With<HudLevelText>, Without<HudLivesText>, Without<HudScoreText>)>` query
  - Update text: `format!("Level: {}", level.number)`

- [x] **5.4** Add `show_level_ready_text` system registered on `OnEnter(GameState::LevelTransition)`:
  - Reads `Res<Level>` and `Query<&mut Text, With<LevelReadyText>>` and `Query<&mut Visibility, With<LevelReadyText>>`
  - Updates text to `format!("Level {} — Get Ready", level.number)`
  - Sets visibility to `Visibility::Visible`

- [x] **5.5** Add `hide_level_ready_text` system registered on `OnExit(GameState::LevelTransition)`:
  - Sets `LevelReadyText` visibility to `Visibility::Hidden`

*Checkpoint: compiles cleanly. Level number shows in HUD top-center. "Level X — Get Ready" overlay appears during transition and hides when transition ends. All existing tests pass.*

---

### 6. Unit tests

> Test the pure `asteroid_count_for_level` function. No Bevy app required.

- [x] **6.1** Add `#[cfg(test)] mod tests` block in `src/plugins/level.rs` with:
  - `asteroid_count_level_1_is_initial_count`: assert `asteroid_count_for_level(1) == ASTEROID_INITIAL_COUNT`
  - `asteroid_count_level_2_adds_increment`: assert `asteroid_count_for_level(2) == ASTEROID_INITIAL_COUNT + ASTEROID_COUNT_INCREMENT`
  - `asteroid_count_level_3_adds_two_increments`: assert `asteroid_count_for_level(3) == ASTEROID_INITIAL_COUNT + 2 * ASTEROID_COUNT_INCREMENT`
  - `asteroid_count_caps_at_max`: assert `asteroid_count_for_level(100) == ASTEROID_MAX_COUNT`
  - `asteroid_count_at_cap_boundary`: assert the exact level where cap is first hit (level 4: `6 + 3*2 = 12`) equals `ASTEROID_MAX_COUNT`

- [x] **6.2** Add constant validation tests to `src/config.rs`:
  - `level_constants_are_consistent`: assert `ASTEROID_INITIAL_COUNT < ASTEROID_MAX_COUNT`
  - `level_transition_secs_is_positive`: assert `LEVEL_TRANSITION_SECS > 0.0`

*Checkpoint: `cargo test` passes. All new and existing tests green.*

---

## Open Questions

None — all questions resolved during refinement.

---

## Notes for Execute

- **`detect_level_clear` system ordering:** register it without an explicit `GameSet` — it only transitions state and doesn't need to run before/after collision or movement. It runs on `Update` gated by `run_if(in_state(Playing))`.
- **Startup asteroid spawn removed:** `AsteroidPlugin` previously called `spawn_asteroids` on `Startup`. After Group 3, this is gone — the game starts with an empty field until `handle_attract_input` fires `ResetGameEvent`, which triggers `on_reset_game` → `SpawnLevelEvent` → asteroids spawn. This is correct and intentional.
- **`LevelTransitionTimer` is optional during `on_reset_game`:** `commands.remove_resource::<LevelTransitionTimer>()` is a no-op if the resource doesn't exist — safe to call unconditionally.
- **Query filter conflicts in `update_hud`:** the existing queries use `Without<HudLivesText>` / `Without<HudScoreText>` to avoid ambiguity. Add corresponding `Without<HudLevelText>` filters to each existing query, and add all three `Without` filters to the new level text query.
- **`show_level_ready_text` text + visibility:** Bevy separates `Text` and `Visibility` into different components. The `OnEnter` system needs two separate queries or a combined `(Entity, &mut Text, &mut Visibility)` query on `LevelReadyText`.
