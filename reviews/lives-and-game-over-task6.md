# Review: Lives and Game Over — Task 6 (ShipPlugin)

**Plan:** `plans/lives-and-game-over.md` (Task 6 only)
**Spec:** `specs/lives-and-game-over.md`
**Date:** 2026-03-22
**Outcome:** Passed with fixes

---

## Summary

Task 6 correctly removes the Startup ship spawn, registers `SpawnShipEvent`, adds `spawn_ship_from_event` and `handle_ship_reset` systems, renames `spawn_ship` to `spawn_ship_entity` with the `invincible: bool` parameter, and gates all five movement systems under `.run_if(in_state(GameState::Playing))`. All five subtasks are implemented. The toolchain passes cleanly (27 tests, no format issues). The only clippy failures are five pre-existing dead-code warnings from Tasks 2/5 stubs that Tasks 8–9 will resolve; none are regressions introduced by Task 6. One warning-level design note was identified and documented.

---

## Toolchain

- `cargo fmt`: Clean — no changes needed.
- `cargo test`: 27/27 passed. All pre-existing tests for `clamp_to_max_speed` and `wrap_position` continue to pass.
- `cargo clippy`: 5 errors under `-D warnings`, all pre-existing dead-code stubs from Tasks 2 and 5:
  - `HudLivesText` (components.rs:39) — used in Task 9
  - `HudScoreText` (components.rs:42) — used in Task 9
  - `GameOverText` (components.rs:45) — used in Task 9
  - `PressAnyKeyText` (components.rs:48) — used in Task 9
  - `SHIP_RADIUS` (config.rs:14) — used in Task 8

  None of these are regressions from Task 6. They are blockers for `cargo clippy -- -D warnings` until Tasks 8 and 9 are complete.

## Playtest

- `cargo run`: Cannot run — headless environment. Deferred to user acceptance.

---

## Findings

### Blockers

None introduced by Task 6.

### Warnings

**W1 — `handle_ship_reset`: multiple `SpawnShipEvent`s if `ResetGameEvent` fires more than once per frame**

`handle_ship_reset` iterates over all `ResetGameEvent`s and sends one `SpawnShipEvent { invincible: false }` per event. If two `ResetGameEvent`s arrive in the same frame (e.g., duplicate key-press processing or a future code path), the second iteration will find no `Player` entity (already despawned) so no second despawn occurs, but it will still send a second `SpawnShipEvent`, resulting in two ships being spawned simultaneously.

In practice, `handle_attract_input` and `handle_game_over_input` each fire at most one `ResetGameEvent` per frame, so this path is not reachable in normal play today. However, it is an implicit assumption that could break silently when Task 7 adds asteroid reset logic or if a second caller of `ResetGameEvent` is added.

Recommendation for a future tightening pass: consume the iterator with `.read().last().is_some()` (send exactly one spawn event if any reset events were present) rather than one-per-event. Not required to unblock Tasks 7–10.

### Notes

**N1 — Plan subtask 6.2 wording vs implementation**

The plan says to rename `spawn_ship` to `fn spawn_ship_entity(commands, meshes, materials)` and add `invincible: bool`. The implementation uses `&mut ResMut<Assets<Mesh>>` and `&mut ResMut<Assets<ColorMaterial>>` as references to the `ResMut` smart pointers, which is the correct Bevy pattern for passing asset resources into a non-system helper. The signature matches the intent; the plan's abbreviated parameter list is not misleading.

**N2 — No new pure-logic functions requiring unit tests**

Task 6 introduced no new pure functions. `spawn_ship_entity`, `spawn_ship_from_event`, and `handle_ship_reset` all require ECS types and cannot be unit-tested without a Bevy app. The existing tests for `clamp_to_max_speed` and `wrap_position` are unaffected. No test gaps exist for Task 6 specifically.

**N3 — `handle_ship_reset` uses `despawn` (not `despawn_recursive`)**

Correct per plan notes: the ship entity has no children at this stage. If a thruster-flame child is added in a future spec, this will need updating to `despawn_recursive`. Noted as a future caution, not a defect.

**N4 — Movement systems grouped in a single `add_systems` call**

All five movement systems (`ship_rotation`, `ship_thrust`, `ship_movement`, `wrap_ship`, `ship_shoot`) are grouped in a single `add_systems` call with shared `.in_set(GameSet::Movement).run_if(in_state(GameState::Playing))`. This is clean and correct — the condition applies to all five at once.

---

## Fixes Applied

### Refactoring

None required. Code was clean on review.

### Tests Added

None. Task 6 introduced no new pure-logic functions amenable to unit testing.

---

## Outcome

Task 6 is complete and correct. All five subtasks (6.1–6.5) are implemented as specified. The toolchain passes (formatting clean, 27 tests green). The five clippy dead-code errors are pre-existing stubs from Tasks 2/5 and are not regressions. One warning-level design note (W1) documents a latent multi-event edge case in `handle_ship_reset` that does not affect current functionality but should be considered if additional `ResetGameEvent` sources are added in later tasks. The plan remains In Progress; Tasks 7–10 are not yet implemented.
