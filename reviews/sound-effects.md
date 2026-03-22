# Review: Sound Effects

**Plan:** `plans/sound-effects.md`
**Spec:** `specs/sound-effects.md`
**Date:** 2026-03-22
**Outcome:** Passed with Notes

---

## Summary

The sound-effects implementation is complete and correct. All five audio systems are in place, events are properly wired from game logic into the sound plugin, and the plugin integrates cleanly with the existing app. One runtime panic was discovered and fixed post-Execute (missing `wav` codec feature) before this review ran. The code is clean, well-structured, and idiomatic. Three spec inaccuracies were corrected as spec drift.

---

## Toolchain

- `cargo fmt`: Clean
- `cargo test`: 25 passed
- `cargo clippy`: Clean

---

## Playtest

- `cargo run`: Starts and runs. Game is playable. Terminated cleanly after manual observation — no crash, no panic.
- Sound files present in `assets/sounds/` — all six `.wav` files bundled. All sound effects confirmed playing during playtest.
- No regressions in existing gameplay behavior.

---

## Findings

### Blockers

None.

### Warnings

- **[W1]** Documentation not generated. **Action required:** run `/docs plans/sound-effects.md` after this review.

### Notes

- **[N1]** The `wav` Bevy feature was not in the original plan. It was a necessary addition discovered when `cargo run` panicked: `AudioPlugin` in Bevy 0.15 only calls `app.add_audio_source::<AudioSource>()` (which registers the asset type) when at least one codec feature — `mp3`, `flac`, `wav`, or `vorbis` — is enabled. Without it, `asset_server.load()` panics at startup. The spec's Cargo.toml section has been updated to document this requirement.

- **[N2]** The spec referenced `PlaybackSettings::DESPAWN_ON_END` but the correct constant in Bevy 0.15 is `PlaybackSettings::DESPAWN`. The spec has been corrected.

- **[N3]** The spec's module structure table listed `src/plugins/bullet.rs` as the file modified for `BulletFiredEvent`. The plan already noted this as a typo — `ship_shoot` lives in `src/plugins/ship.rs`. The spec has been corrected.

---

## Fixes Applied

### Spec Updates

- `specs/sound-effects.md` — corrected `PlaybackSettings::DESPAWN_ON_END` → `PlaybackSettings::DESPAWN` throughout (2 occurrences)
- `specs/sound-effects.md` — updated Cargo.toml change section to require both `bevy_audio` and `wav` features, with explanation of why both are needed
- `specs/sound-effects.md` — corrected module structure: `bullet.rs` → `ship.rs` for `BulletFiredEvent` sender

---

## Outcome

**Passed with Notes** — all tasks implemented, suite green, game runs without panic. No blockers. Documentation generation outstanding.

Run `/docs plans/sound-effects.md` to complete this feature.
