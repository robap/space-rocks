# Review: Project Bootstrap — Task 2 (Shared Types)

**Plan:** `plans/project-bootstrap.md` (tasks 2.1–2.7 only)
**Spec:** `specs/project-bootstrap.md`
**Date:** 2026-03-22
**Outcome:** Passed with Notes

---

## Summary

All shared types and constants are defined correctly. The implementation matches the spec's Key Types section precisely in structure and values. The four plan-specified tests pass. Two coverage gaps in `radius()` (Medium and Small variants untested) were added during review. One minor spec drift: `Debug` was added to `AsteroidSize`'s derive list — a sensible improvement, spec updated to match.

---

## Toolchain

- `cargo fmt --check`: Clean
- `cargo test`: 4 passed (6 after review added 2 missing tests)
- `cargo clippy -- -D warnings`: Clean

---

## Playtest

Not applicable — task 2 adds no systems or rendering. No `cargo run` behavioral change expected.

---

## Findings

### Blockers

None.

### Warnings

None.

### Notes

- **[N1]** `#![allow(dead_code)]` suppresses the dead_code lint at module level in both `components.rs` and `config.rs`. This is the correct call for this intermediate stage — the types are `pub` API that will be consumed by plugin tasks 4–7. However, once those types are in use, the `#![allow(dead_code)]` becomes a silent suppressor that could mask real dead code added later. **Action for task 7 review:** verify these `#![allow(dead_code)]` attributes can be removed once all plugins are implemented.

- **[N2]** `AsteroidSize` derives `Debug` but the spec's Key Types block only listed `Clone, Copy, PartialEq, Eq`. `Debug` is a sensible addition (useful in tests and panics) and is not harmful. Spec updated to match.

---

## Fixes Applied

### Tests Added

- `components::tests::radius_medium` — `AsteroidSize::Medium.radius()` returned 24.0; was untested despite `radius()` being a branching function with three variants.
- `components::tests::radius_small` — `AsteroidSize::Small.radius()` returns 12.0; same rationale.

### Spec Updates

- `specs/project-bootstrap.md` line 95 — updated `AsteroidSize` derive list to include `Debug`, annotated `(Debug added post-implementation)`.

---

## Outcome

**Passed with Notes** — toolchain clean, all plan tasks verified, two coverage gaps closed, one note recorded for follow-up at the task 7 review boundary. Plan tasks 2.1–2.7 remain `[x]` as marked.
