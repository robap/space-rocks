# Plan: Score System

**Spec:** `specs/score-system.md`
**Status:** Complete ✓
**Created:** 2026-03-22

---

## Overview

Three files change: `src/config.rs` gets three point constants, `src/plugins/score.rs` is created with the `ScorePlugin` and its one system, and `src/main.rs` wires the plugin in. `src/plugins/mod.rs` gets a one-line module declaration. All supporting infrastructure (`Score`, `AsteroidDestroyedEvent`, `HudPlugin`, `GameStatePlugin` reset) already exists — this plan only adds what is missing.

A critical integration detail: `AsteroidDestroyedEvent` is written by `CollisionPlugin` but is not yet registered with `app.add_event`. The game will panic on the first asteroid destruction until `ScorePlugin` calls `app.add_event::<AsteroidDestroyedEvent>()` in its `build` method.

---

## Prerequisites

- None. All required types (`Score`, `AsteroidDestroyedEvent`, `AsteroidSize`, `GameSet`, `GameState`) are already defined and the codebase compiles.

---

## Tasks

### 1. Add Point Constants to `src/config.rs`

> Add the three scoring constants so `ScorePlugin` can depend on them without magic numbers.

- [x] **1.1** Add to the bottom of `src/config.rs`:
  ```rust
  pub const SCORE_LARGE: u32 = 20;
  pub const SCORE_MEDIUM: u32 = 50;
  pub const SCORE_SMALL: u32 = 100;
  ```

*Checkpoint: `cargo check` passes cleanly. No behaviour change yet — constants are unused.*

---

### 2. Create `src/plugins/score.rs`

> Create the new plugin file with all logic, wiring, and tests in one place. Requires group 1.

- [x] **2.1** Create `src/plugins/score.rs` with the plugin struct and a stub `build` that compiles:
  ```rust
  use bevy::prelude::*;
  use crate::components::*;
  use crate::config::*;

  pub struct ScorePlugin;

  impl Plugin for ScorePlugin {
      fn build(&self, app: &mut App) {}
  }
  ```

- [x] **2.2** Add `pub fn points_for_size(size: AsteroidSize) -> u32` below the plugin impl:
  ```rust
  pub fn points_for_size(size: AsteroidSize) -> u32 {
      match size {
          AsteroidSize::Large  => SCORE_LARGE,
          AsteroidSize::Medium => SCORE_MEDIUM,
          AsteroidSize::Small  => SCORE_SMALL,
      }
  }
  ```

- [x] **2.3** Add `fn on_asteroid_destroyed` system below `points_for_size`:
  ```rust
  fn on_asteroid_destroyed(
      mut events: EventReader<AsteroidDestroyedEvent>,
      mut score: ResMut<Score>,
  ) {
      for event in events.read() {
          score.0 = score.0.saturating_add(points_for_size(event.size));
      }
  }
  ```

- [x] **2.4** Fill in `ScorePlugin::build` to register the event and system:
  ```rust
  fn build(&self, app: &mut App) {
      app.add_event::<AsteroidDestroyedEvent>()
          .add_systems(
              Update,
              on_asteroid_destroyed
                  .after(GameSet::Collision)
                  .run_if(in_state(GameState::Playing)),
          );
  }
  ```

- [x] **2.5** Add unit tests at the bottom of the file:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn large_asteroid_scores_20() {
          assert_eq!(points_for_size(AsteroidSize::Large), 20);
      }

      #[test]
      fn medium_asteroid_scores_50() {
          assert_eq!(points_for_size(AsteroidSize::Medium), 50);
      }

      #[test]
      fn small_asteroid_scores_100() {
          assert_eq!(points_for_size(AsteroidSize::Small), 100);
      }
  }
  ```

*Checkpoint: `cargo test` passes — three new tests green, all existing tests still pass.*

---

### 3. Wire `ScorePlugin` into the App

> Expose the module and add the plugin to the app. Requires group 2.

- [x] **3.1** Add `pub mod score;` to `src/plugins/mod.rs` (alongside the other module declarations).

- [x] **3.2** In `src/main.rs`, add `score::ScorePlugin` to the `use plugins::{...}` import block.

- [x] **3.3** In `src/main.rs`, add `ScorePlugin` to the `add_plugins((...))` call alongside the existing plugins.

*Checkpoint: `cargo run` compiles and launches. Shooting an asteroid increments the score shown in the top-right HUD. Destroying a large asteroid shows +20, medium +50, small +100. Score resets to 0 when starting a new game.*

---

## Open Questions

None. All decisions resolved in the spec.

---

## Notes for Execute

- `after(GameSet::Collision)` is the correct ordering constraint — not `in_set(GameSet::Collision)`. The scoring system reads events produced by the collision set, so it must run after it, not inside it.
- `AsteroidDestroyedEvent` must be registered here in `ScorePlugin` because no other plugin currently calls `app.add_event::<AsteroidDestroyedEvent>()`. Until this plan is executed, the first asteroid destruction will cause a Bevy panic at runtime.
- `saturating_add` is intentional — prevents `u32` overflow for an extremely long session. Do not change to `+=`.
- The `Score` resource is initialised by `GameStatePlugin::insert_game_resources` at `Startup` and reset to `0` by both `handle_attract_input` and `handle_game_over_input` — `ScorePlugin` does not need to touch resource initialisation.
