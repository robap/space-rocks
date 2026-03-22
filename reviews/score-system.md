# Review: Score System

**Plan:** `plans/score-system.md`
**Spec:** `specs/score-system.md`
**Date:** 2026-03-22
**Outcome:** Passed

---

## Summary

The implementation is minimal, correct, and exactly what the spec describes. All three files were touched as planned, the critical missing `add_event::<AsteroidDestroyedEvent>()` registration was included, and `saturating_add` is used correctly. No regressions, no scope creep, no security concerns. The feature is production-ready within the MVP constraints.

---

## Toolchain

- `cargo fmt`: Clean
- `cargo test`: 28 passed (3 new tests for `points_for_size`)
- `cargo clippy`: Clean

---

## Playtest

- `cargo run`: Deferred — headless environment, no display available
- Visual behavior: Deferred to user acceptance
- Performance: No concerns — system iterates at most a handful of events per frame
- Crashes/panics: None expected; no panic-capable code paths in the new system

**For user acceptance:** Launch the game, press any key to start, shoot asteroids, and confirm:
- Large asteroid hit → score increases by 20
- Medium asteroid hit → score increases by 50
- Small asteroid hit → score increases by 100
- Starting a new game from the game-over screen resets the score to 0
- Destroying two asteroids in one frame scores both correctly

---

## Findings

### Blockers

None.

### Warnings

- **[W1]** Documentation not yet produced. Run `/docs plans/score-system.md` to generate technical and user-facing docs for this feature.

### Notes

- **[N1]** `points_for_size` is declared `pub`. Since this is a single binary crate with no external consumers, `pub` and `pub(crate)` are equivalent. The spec explicitly wrote it as `pub fn`, so this matches intent and requires no change.
- **[N2]** The edge cases listed in the spec (multiple collisions per frame, score overflow via `saturating_add`, events outside `Playing` state) are handled in the system but have no dedicated tests. This is correct per the spec's testing strategy, which scopes unit tests to `points_for_size` only. The system-level behaviors require a full Bevy app to test and are deferred to integration testing via playtest.

---

## Fixes Applied

None required.

---

## Outcome

**Passed** — all plan tasks implemented and verified, no blockers, suite green, clippy clean.
