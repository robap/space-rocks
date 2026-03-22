# Plan: Thrust Enhancements

**Spec:** `specs/thrust-enhancements.md`
**Status:** Complete

---

## Overview

Config-only change. Two constants in `src/config.rs`:
- `SHIP_MAX_SPEED`: 400 → 800
- `SHIP_DRAG`: 0.98 → 0.99

Two unit tests added to `src/config.rs` `#[cfg(test)]` block (project convention per CLAUDE.md).

---

## Tasks

- [x] **1. Write tests for the new constant values**
  - Add `test_ship_max_speed_is_double_baseline` to `src/config.rs` `#[cfg(test)]`
  - Add `test_ship_drag_is_less_than_original` to `src/config.rs` `#[cfg(test)]`
  - Tests must fail against current values (400.0 and 0.98)

- [x] **2. Update `SHIP_MAX_SPEED` to 800.0**
  - `src/config.rs` line 3

- [x] **3. Update `SHIP_DRAG` to 0.99**
  - `src/config.rs` line 4

- [x] **4. Verify all tests pass and suite is clean**
  - `cargo test`
  - `cargo fmt`
  - `cargo clippy -- -D warnings`

---

## Notes for Execute

- Tests live in `#[cfg(test)]` inside `src/config.rs` per CLAUDE.md — not in `tests/`
- No system changes, no new types, no new files
- The "stub" phase is trivially satisfied: existing wrong values are the stub
