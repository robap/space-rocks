# Review: Project Bootstrap — Task 7 (CollisionPlugin)

**Plan:** `plans/project-bootstrap.md` (tasks 7.1–7.4 only)
**Spec:** `specs/project-bootstrap.md`
**Date:** 2026-03-22
**Outcome:** Passed with fixes

---

## Summary

CollisionPlugin is implemented cleanly and completely. The bullet↔asteroid circle collision check is extracted as a pure function (`circles_are_colliding`) and unit-tested. The double-despawn guard uses a per-frame `HashSet<Entity>` as the plan specified. System ordering is implemented via a `GameSet { Movement, Collision, Despawn }` enum in `components.rs`, configured in `main.rs` and applied across all four plugins. Two housekeeping items carried forward from the task 6 review were resolved: `#![allow(dead_code)]` suppressors removed from `components.rs` and `config.rs`, and the spec updated to match the full set of constants and the actual `main.rs` structure.

---

## Toolchain

- `cargo fmt --check`: Clean
- `cargo test`: 24 passed (1 new test added during review)
- `cargo clippy -- -D warnings`: Clean

---

## Playtest

- `cargo run`: Deferred — headless environment, no display available.
- Expected behavior per spec: shooting a large asteroid splits it into two medium ones; medium splits to small; small is fully destroyed. Bullets consumed on hit. No double-despawn crashes.

---

## Findings

### Blockers

None.

### Warnings

None.

### Notes

- **[N1]** Deferred from task 6 review [N2]: `#![allow(dead_code)]` in `components.rs` and `config.rs` was meant to be removed after task 7. **Fixed:** suppressor and stale comments removed from both files. Zero new clippy warnings resulted.

- **[N2]** Deferred from task 6 review [N4]: `GameSet` system set approach was correctly chosen over exposing `move_bullets`/`move_asteroids` as `pub`. This keeps all movement-system functions private within their respective plugins.

- **[N3]** `/docs` has not been run for this project. Run `/docs plans/project-bootstrap.md` to produce technical and user-facing documentation for the full MVP.

---

## Fixes Applied

### Tests Added

- `src/plugins/collision.rs::tests::test_overlapping_circles_at_diagonal_are_colliding` — the three axis-aligned tests only exercised 1D distance. This 3-4-5 triangle case confirms `Vec2::distance` is used correctly for 2D collision.

### Refactoring

- `src/components.rs` — removed `#![allow(dead_code)]` suppressor and stale plan-phase comment. All types are fully used as of task 7.
- `src/config.rs` — same suppressor and comment removed.

### Spec Updates

- `specs/project-bootstrap.md` — `config.rs` listing updated to include four constants added during implementation: `BULLET_RADIUS`, `BULLET_SPAWN_OFFSET`, `ASTEROID_MIN_ANGULAR_VELOCITY`, `ASTEROID_MAX_ANGULAR_VELOCITY`.
- `specs/project-bootstrap.md` — `main.rs` structure updated to show `configure_sets`, `setup_camera`, and the `GameSet` ordering note.

---

## Outcome

**Passed with fixes** — all plan tasks verified complete, one coverage gap closed, two housekeeping deferred items resolved, spec updated to match implementation. Toolchain clean at 24 passing tests, zero warnings.

Plan status updated to `Complete ✓`. ROADMAP MVP items marked `[x]`.

Run `/docs plans/project-bootstrap.md` to complete documentation for the full MVP.
