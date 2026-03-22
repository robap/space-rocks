# Review: Project Bootstrap — Task 5 (ShipPlugin)

**Plan:** `plans/project-bootstrap.md` (tasks 5.1–5.8 only)
**Spec:** `specs/project-bootstrap.md`
**Date:** 2026-03-22
**Outcome:** Passed with fixes

---

## Summary

All eight subtasks are complete and the implementation is clean. The ship spawns at centre, rotates with A/D or arrow keys, thrusts with W or up arrow, wraps at screen edges, and fires bullets on Space. Pure logic (velocity clamping and screen wrapping) is correctly extracted into private helpers with inline tests. Two warnings were fixed: `22.0` and `3.0` magic literals in `ship_shoot` were extracted to `BULLET_SPAWN_OFFSET` and `BULLET_RADIUS` in `config.rs`. `BULLET_RADIUS` is particularly important — it will be referenced again in `CollisionPlugin` (task 7). Two missing `wrap_position` tests (top and bottom edges) were added.

---

## Toolchain

- `cargo fmt --check`: Clean
- `cargo test`: 20 passed (no regressions; 9 new tests from task 5)
- `cargo clippy -- -D warnings`: Clean

---

## Playtest

- `cargo run`: Deferred — headless environment, no display available.
- Expected: Ship appears as a blue-white triangle at screen centre. A/D or arrow keys rotate; W or up arrow thrusts forward with momentum and drag; ship wraps at all four edges; Space fires yellow dots that travel in the ship's facing direction and inherit ship velocity.

---

## Findings

### Blockers

None.

### Warnings

- **[W1]** `src/plugins/ship.rs:139` — `22.0` (bullet spawn offset) is a magic literal with no named constant. This is a tunable gameplay value; CLAUDE.md requires all such values to live in `config.rs`. **Fixed:** extracted to `BULLET_SPAWN_OFFSET: f32 = 22.0` in `config.rs`.

- **[W2]** `src/plugins/ship.rs:143` — `3.0` (bullet mesh radius) is a magic literal. The same value will be used in `CollisionPlugin` (task 7.2: "bullet radius + 3.0") — having it as an unnamed literal in two places creates a consistency hazard. **Fixed:** extracted to `BULLET_RADIUS: f32 = 3.0` in `config.rs`. Task 7 must reference this constant.

### Notes

- **[N1]** `wrap_position` is duplicated between `asteroid.rs` and `ship.rs`. The spec explicitly requires "Wrapping is handled in each plugin's movement system (not a shared system) to keep plugins self-contained." This duplication is intentional and spec-compliant. No change needed.

- **[N2]** Triangle mesh vertices (`0.0, 20.0`, `-12.0, -14.0`, `12.0, -14.0`) remain as literals in `spawn_ship`. These are rendering geometry, not tunable gameplay constants. CLAUDE.md's "no magic numbers" convention targets gameplay values in systems; visual mesh proportions are a borderline case. Left as-is — acceptable for MVP.

- **[N3]** Carry-forward from task 4 review [N4]: `#![allow(dead_code)]` in `config.rs` and `components.rs`. All constants and components are now in use as of task 5, so these suppressors are technically no longer needed. Deferring removal to task 7 per prior review guidance — removing them now risks premature noise if any constant appears unused until `CollisionPlugin` and `BulletPlugin` are complete.

- **[N4]** `cargo audit` not available in this environment. No new dependencies were added for task 5.

- **[N5]** `/docs` has not been run for this feature. Run `/docs plans/project-bootstrap.md` after all tasks are complete (task 7) to document the full MVP.

---

## Fixes Applied

### Refactoring

- `src/config.rs` — added `BULLET_RADIUS: f32 = 3.0` and `BULLET_SPAWN_OFFSET: f32 = 22.0` to replace magic literals in `ship_shoot`. `BULLET_RADIUS` must be reused in `collision.rs` (task 7) to avoid value drift.
- `src/plugins/ship.rs` — replaced `22.0` with `BULLET_SPAWN_OFFSET` and `3.0` with `BULLET_RADIUS` in `ship_shoot`.

### Tests Added

- `plugins::ship::tests::test_wrap_position_past_top_edge_wraps_to_bottom` — top-edge wrap was untested in `ship.rs` (asteroid.rs covered it for its own copy; ship's independent copy needed its own coverage)
- `plugins::ship::tests::test_wrap_position_past_bottom_edge_wraps_to_top` — same rationale

### Spec Updates

None required. Task 5 implements the spec exactly.

---

## Outcome

**Passed with fixes** — two warnings fixed, two tests added, toolchain clean, all plan tasks verified. Tasks 5.1–5.8 remain `[x]` as marked by Execute.

Plan status remains `In progress` — tasks 6–7 remain.
