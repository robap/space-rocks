# Review: Lives and Game Over

**Plan:** `plans/lives-and-game-over.md`
**Spec:** `specs/lives-and-game-over.md`
**Date:** 2026-03-22
**Outcome:** Passed with fixes

---

## Summary

The lives-and-game-over feature is correctly implemented across all ten task groups. The state
machine, HUD, collision system, respawn flow, and invincibility blink all match the spec. One
structural issue was found and fixed during review: `wrap_position` was duplicated across three
modules and has been extracted to `components.rs` as a shared utility. The suite is green at
25 tests with zero clippy warnings.

---

## Toolchain

- `cargo fmt`: Fixed trailing whitespace in `asteroid.rs` and `ship.rs` after refactor
- `cargo test`: 25 passed (reduced from 30; 5 duplicate wrap tests removed, 5 added to `components.rs`)
- `cargo clippy -- -D warnings`: Clean

---

## Playtest

- `cargo run`: Cannot run — headless environment. Deferred to user acceptance.
- Suggest verifying the following manually:
  - Attract screen: asteroids move, no ship, "Press Any Key to Start" visible, HUD shows "Lives: 0"
  - Pressing any key: ship spawns, asteroids reset, HUD updates to "Lives: 3"
  - Ship hitting asteroid: life decrements in HUD, ship despawns, respawn after 1.5 s
  - Respawned ship blinks for 2 s and is invincible during that window
  - After 3 deaths: "Game Over" and "Press Any Key to Start" visible
  - Pressing any key from game over: fresh game resets

---

## Findings

### Blockers

None.

### Warnings

None.

### Notes

**[N1] `insert_game_resources` correctly initializes `Lives(0)`, not `Lives(PLAYER_STARTING_LIVES)`**

Plan task 5.2 incorrectly specified `Lives(PLAYER_STARTING_LIVES)`. The attract screen has no
active game; showing a live count there is meaningless. `Lives` is set to `PLAYER_STARTING_LIVES`
by `handle_attract_input` when the player actually starts a game. `Lives(0)` on startup is the
correct design. Plan task 5.2 updated to document the rationale.

**[N2] `handle_attract_input` and `handle_game_over_input` are identical functions**

Both are identical in body; they differ only in their `run_if` state gate. Per project DRY rules,
extraction is deferred until a third identical handler is added. Carried forward from the task-5
review (N4).

**[N3] `#[allow(clippy::type_complexity)]` on `ship_asteroid_collision`**

The suppression is justified — the query type `(With<Player>, Without<Invincible>)` is the
correct idiomatic Bevy pattern and cannot be simplified without losing semantic clarity.

**[N4] `/docs` not run**

Technical and user documentation for this feature does not exist. Run `/docs plans/lives-and-game-over.md`
after this review passes to generate it.

---

## Fixes Applied

### Refactoring

**Extracted `wrap_position` to `components.rs` as a shared public function**

`wrap_position(translation: &mut Vec3, half_w: f32, half_h: f32)` existed as a private function
in both `ship.rs` and `asteroid.rs` with identical implementations. `bullet.rs` had the same
logic inlined directly in `wrap_bullets`. Three occurrences exceeds the DRY extraction threshold.

Changes made:
- Added `pub fn wrap_position` to `src/components.rs`
- Removed private `wrap_position` from `src/plugins/ship.rs`
- Removed private `wrap_position` from `src/plugins/asteroid.rs`
- Refactored `wrap_bullets` in `src/plugins/bullet.rs` to call `wrap_position`

All three modules already import `use crate::components::*`, so no additional imports were needed.

### Tests Removed

- `plugins::ship::tests::test_wrap_position_*` (5 tests) — were testing the now-removed private
  `wrap_position` in `ship.rs`. Covered by the shared function tests in `components.rs`.
- `plugins::asteroid::tests::test_wrap_position_*` (5 tests) — same reason. The entire test
  module in `asteroid.rs` was removed as it contained only these tests.

### Tests Added

- `components::tests::wrap_position_past_right_edge_wraps_to_left`
- `components::tests::wrap_position_past_left_edge_wraps_to_right`
- `components::tests::wrap_position_past_top_edge_wraps_to_bottom`
- `components::tests::wrap_position_past_bottom_edge_wraps_to_top`
- `components::tests::wrap_position_within_bounds_is_unchanged`

---

## Outcome

**Passed with fixes** — `wrap_position` duplication refactored. All tasks checked off, 25 tests
green, zero clippy warnings.

Run `/docs plans/lives-and-game-over.md` to complete documentation for this feature.
