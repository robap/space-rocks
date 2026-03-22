# Review: Project Bootstrap — Task 4 (AsteroidPlugin)

**Plan:** `plans/project-bootstrap.md` (tasks 4.1–4.6 only)
**Spec:** `specs/project-bootstrap.md`
**Date:** 2026-03-22
**Outcome:** Passed with fixes

---

## Summary

All six subtasks are complete and the implementation is clean. Asteroids spawn at random screen edges, drift with angular rotation, and wrap correctly. The `wrap_position` helper is correctly extracted and well-tested. One warning was fixed: angular velocity bounds `-1.5..1.5` were magic numbers in `spawn_asteroids` — extracted to `ASTEROID_MIN_ANGULAR_VELOCITY` and `ASTEROID_MAX_ANGULAR_VELOCITY` in `config.rs`. No structural issues found.

---

## Toolchain

- `cargo fmt --check`: Clean
- `cargo test`: 11 passed (no regressions; 5 new tests from task 4)
- `cargo clippy -- -D warnings`: Clean

---

## Playtest

- `cargo run`: Deferred — headless environment, no display available.
- Expected: 6 grey circles spawning at screen edges, drifting across the window, wrapping to the opposite side when they exit. No ship or collision yet.

---

## Findings

### Blockers

None.

### Warnings

- **[W1]** `src/plugins/asteroid.rs:35` — angular velocity range `gen_range(-1.5_f32..1.5_f32)` used magic literals not present in `config.rs`. Every tunable constant must live in config per project conventions. **Fixed:** extracted to `ASTEROID_MIN_ANGULAR_VELOCITY: f32 = -1.5` and `ASTEROID_MAX_ANGULAR_VELOCITY: f32 = 1.5` in `config.rs`.

### Notes

- **[N1]** `random_edge_position` uses `_ =>` as a catch-all on a `0u8..4` range. The `_` arm handles only `3` (right edge) — this is effectively exhaustive. The pattern is acceptable here since the range is a bare integer literal with no enum backing. No change needed.

- **[N2]** `cargo audit` not available in this environment. `rand = "0.8"` is widely used, actively maintained, and has no known advisories at time of writing. Low risk.

- **[N3]** Carry-forward from task 1 review [N1]: `bevy_input` not listed explicitly in `Cargo.toml` features. Task 5 (ShipPlugin) will require it for `ButtonInput<KeyCode>`. If the build fails, add `"bevy_input"` to the feature list. Low risk — it is likely pulled in transitively by `bevy_winit`.

- **[N4]** Carry-forward from task 2 review [N1]: `#![allow(dead_code)]` remains in `config.rs`. `ASTEROID_MIN_ANGULAR_VELOCITY` and `ASTEROID_MAX_ANGULAR_VELOCITY` are now in use from `asteroid.rs`, but other constants are still suppressed. Still appropriate to defer removal until task 7.

---

## Fixes Applied

### Refactoring

- `src/config.rs` — added `ASTEROID_MIN_ANGULAR_VELOCITY: f32 = -1.5` and `ASTEROID_MAX_ANGULAR_VELOCITY: f32 = 1.5` to replace magic literals in `spawn_asteroids`.

### Tests Added

None beyond what Execute already wrote (5 tests for `wrap_position` covering all four boundary directions plus the in-bounds case — complete coverage).

### Spec Updates

None required. The spec's Behavior section says asteroids spawn with "random velocity and angular velocity" without specifying a range; the config constants are an implementation detail. No spec change needed.

---

## Outcome

**Passed with fixes** — one warning fixed, toolchain clean, all plan tasks verified. Tasks 4.1–4.6 remain `[x]` as marked by Execute.

Plan status remains `In progress` — tasks 5–7 remain.
