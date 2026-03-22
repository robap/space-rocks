# Review: Project Bootstrap — Task 6 (BulletPlugin)

**Plan:** `plans/project-bootstrap.md` (tasks 6.1–6.4 only)
**Spec:** `specs/project-bootstrap.md`
**Date:** 2026-03-22
**Outcome:** Passed with fixes

---

## Summary

All four subtasks are complete and the implementation is minimal and correct. One spec gap was found and fixed: the spec scope explicitly lists "Screen wrapping for ship, asteroids, and bullets" but Execute omitted `wrap_bullets` from BulletPlugin — this was a plan omission, not an Execute error. The function was added during review. No other findings. Bullet systems have no extractable pure logic, so no unit tests were possible or required.

---

## Toolchain

- `cargo fmt --check`: Clean
- `cargo test`: 20 passed (no regressions; no new tests from task 6)
- `cargo clippy -- -D warnings`: Clean

---

## Playtest

- `cargo run`: Deferred — headless environment, no display available.
- Expected: Bullets fired from the ship travel in the facing direction, inherit ship velocity, wrap at screen edges, and vanish after ~1.2 seconds.

---

## Findings

### Blockers

None.

### Warnings

- **[W1]** `src/plugins/bullet.rs` — `wrap_bullets` system absent. The spec scope section explicitly includes "Screen wrapping for ship, asteroids, and bullets." The plan's task 6 description ("Move bullets each frame, tick their lifetime timers, and despawn them when expired") omitted this, so Execute was not at fault — but the behavior was missing. **Fixed:** added `wrap_bullets` system following the same pattern as `wrap_asteroids` and `wrap_ship`, registered in `BulletPlugin::build`.

### Notes

- **[N1]** Plan task 6.1 listed `crate::config::*` as a required import, but no config constants are consumed by `BulletPlugin`'s systems (the bullet lifetime duration lives in the `BulletLifetime` timer constructed at spawn time in `ship_shoot`). The import was correctly omitted. No action needed.

- **[N2]** Carry-forward from task 5 review [N3]: `#![allow(dead_code)]` in `config.rs` and `components.rs`. All constants and components are now used as of tasks 4–6. Deferring removal to after task 7 — `CollisionPlugin` may use additional items and it's cleaner to remove the suppressor once in the final task.

- **[N3]** `wrap_bullets` uses identical logic to `wrap_asteroids` and `wrap_ship`. The spec explicitly requires "Wrapping is handled in each plugin's movement system (not a shared system) to keep plugins self-contained." This triplication is intentional and spec-compliant. No change needed.

- **[N4]** `move_bullets` and `bullet_lifetime` are private functions. If task 7's `CollisionPlugin` needs ordering guarantees, the plan recommends using `GameSet` labels (defined in `main.rs`) rather than `.after(move_bullets)` — which would require making `move_bullets` public. Task 7 should use the `GameSet` approach to keep visibility minimal.

- **[N5]** `/docs` has not been run for this feature. Run `/docs plans/project-bootstrap.md` after task 7 completes to document the full MVP.

---

## Fixes Applied

### Refactoring

- `src/plugins/bullet.rs` — added `wrap_bullets` system and registered it in `BulletPlugin::build`. Added `use bevy::window::PrimaryWindow;` import. System follows the same structure as `wrap_asteroids` and `wrap_ship`.

### Tests Added

None. Bullet movement and wrapping are Bevy ECS systems with no extractable pure logic. Per spec: "Unit tests on pure logic only — no Bevy app in tests for MVP. Visual/integration testing is manual."

### Spec Updates

None required. The spec already correctly listed bullet screen wrapping in scope; the omission was in the plan, not the spec.

---

## Outcome

**Passed with fixes** — one warning fixed (missing `wrap_bullets`), toolchain clean, all plan tasks verified. Tasks 6.1–6.4 remain `[x]` as marked by Execute.

Plan status remains `In progress` — task 7 (CollisionPlugin) remains.
