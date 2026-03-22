# Review: Lives and Game Over — Task 5 (GameStatePlugin)

**Plan:** `plans/lives-and-game-over.md`
**Spec:** `specs/lives-and-game-over.md`
**Date:** 2026-03-22
**Scope:** Task group 5 only (`src/plugins/game_state.rs`)
**Outcome:** Passed with Notes

---

## Summary

Task 5 is correctly implemented. All seven subtasks are checked off with corresponding code.
`game_state.rs` is clean, well-structured, and follows project conventions. The only issues
are two spec-drift items from earlier tasks (plan 2.3 and 2.5) that needed recording, plus
8 expected forward-looking clippy dead-code warnings that will resolve as tasks 6–9 are
implemented.

---

## Toolchain

- `cargo fmt`: Clean
- `cargo test`: 27 passed, 0 failed
- `cargo clippy -- -D warnings`: **8 dead-code errors — all pre-existing, forward-looking** (see Notes)

---

## Playtest

Deferred — headless environment, no display. User should verify the full integration during
or after task group 6.

---

## Findings

### Blockers

None.

### Warnings

None.

### Notes

**[N1] — Clippy dead-code errors are forward-looking, not structural problems**

All 8 errors are in `src/components.rs` and `src/config.rs`, not `game_state.rs`:

| Symbol | Defined in | Consumed by |
|--------|-----------|-------------|
| `SpawnShipEvent.invincible` field | task 2.5 | task 6 (`spawn_ship_from_event`) |
| `HudLivesText`, `HudScoreText`, `GameOverText`, `PressAnyKeyText` | task 2.6 | task 9 (HudPlugin) |
| `SHIP_RADIUS` | task 2.7 | task 8 (collision) |
| `SHIP_INVINCIBILITY_SECS`, `SHIP_BLINK_INTERVAL_SECS` | task 2.7 | task 6 (ShipPlugin) |

The plan's task 5 checkpoint explicitly says "cargo check compiles cleanly. Logic is in place;
full integration requires group 6." This is the expected state. Clippy -D warnings will be clean
after task 9.

**[N2] — Spec drift: `Invincible` has two timers, spec shows one**

The spec (`Key Types`) defines:
```rust
pub struct Invincible {
    pub timer: Timer,
}
```
The plan (task 2.3) and implementation have:
```rust
pub struct Invincible {
    pub timer: Timer,
    pub blink_timer: Timer,
}
```
The `blink_timer` field is required for `tick_invincibility` to blink without external state.
This is a correct improvement. Spec updated below.

**[N3] — Spec drift: `SpawnShipEvent` was optional in spec, is now a concrete type**

The spec described `SpawnShipEvent` as an optional design choice. The plan formalized it.
The implementation depends on it as a concrete type registered by ShipPlugin. Spec updated
below.

**[N4] — `handle_attract_input` and `handle_game_over_input` are identical functions**

Both functions have identical bodies; they differ only in their `run_if` state gate. This is
two occurrences, which per project DRY rules does not yet warrant extraction. Left as-is.
If a third identical handler is added (e.g., a `Paused` state), extract a shared helper at
that point.

---

## Fixes Applied

None required for task 5 code.

### Spec Updates

Updated `specs/lives-and-game-over.md`:
- `Invincible` Key Type updated to include `blink_timer` field
- `SpawnShipEvent` added to Key Types as a concrete event type

---

## Outcome

**Passed with Notes** — task 5 is complete and correct. Forward-looking clippy warnings are
expected and tracked. Spec updated to match implementation.
