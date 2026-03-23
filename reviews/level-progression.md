# Review: Level Progression

**Plan:** `plans/level-progression.md`
**Spec:** `specs/level-progression.md`
**Date:** 2026-03-22
**Outcome:** Passed with fixes

---

## Summary

Clean, well-structured implementation. The plugin-per-feature architecture was followed correctly — `LevelPlugin` owns all level state, `AsteroidPlugin` correctly delegates spawn responsibility to the new event, and the HUD changes are minimal and focused. One spec-mandated error handling requirement (graceful degradation in `tick_level_transition`) was missed by Execute and fixed here. One visibility over-exposure (`pub(crate)` on a private helper) was tightened. Everything else was correct on first pass.

---

## Toolchain

- `cargo fmt`: Clean
- `cargo test`: 37 passed
- `cargo clippy`: Clean

---

## Playtest

- `cargo run`: Could not run — no display available (headless environment, X11 unavailable)
- Visual behavior: Deferred to user acceptance
- Performance: Deferred to user acceptance
- Crashes/panics: None observed in toolchain

**User acceptance checklist:**
- [ ] Level number shows top-center in HUD throughout gameplay
- [ ] Clearing all asteroids pauses game and shows "Level 2 — Get Ready" (etc.)
- [ ] Overlay disappears and new asteroids spawn after ~5 seconds
- [ ] Level 4+ waves spawn 12 asteroids (cap confirmed)
- [ ] Resetting from GameOver starts at Level 1 with 6 asteroids immediately (no transition delay)
- [ ] Ship/bullet systems are frozen during transition (player cannot shoot or move)

---

## Findings

### Blockers
None.

### Warnings

- **[W1 — Fixed]** `src/plugins/level.rs:56` — `tick_level_transition` took `ResMut<LevelTransitionTimer>` as a required parameter. Bevy panics if the resource is absent. The spec explicitly requires: *"If `LevelTransitionTimer` is missing during `tick_level_transition` (invariant violation), use `warn!` and return early — do not panic."* **Fixed:** changed parameter to `Option<ResMut<LevelTransitionTimer>>` with `warn!` + early return on `None`.

### Notes

- **[N1 — Fixed]** `src/plugins/level.rs:32` — `asteroid_count_for_level` was `pub(crate)` despite only being used within `level.rs`. The `#[cfg(test)]` module inside the same file can access private items, so `pub(crate)` was unnecessary visibility exposure. **Fixed:** changed to private `fn`.

- **[N2]** No docs written for this feature (`docs/technical/level-plugin.md`, `docs/user/how-to-play.md` update not done). Run `/docs plans/level-progression.md` to complete documentation.

---

## Fixes Applied

### Spec Compliance
- `tick_level_transition` now uses `Option<ResMut<LevelTransitionTimer>>` with `warn!` + early return, matching the spec's stated error handling strategy for invariant violations.

### API Surface
- `asteroid_count_for_level` visibility tightened from `pub(crate)` to private. Tests within the same file access it without issue.

---

## Outcome

**Passed with fixes** — both fixes applied, suite green, zero warnings. ROADMAP updated.

Run `/docs plans/level-progression.md` to write technical and user documentation for this feature.
