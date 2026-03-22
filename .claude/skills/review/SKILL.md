---
name: review
description: Reviews a completed plan against its spec. Fixes what it can (refactoring, clean code, test gaps). Reports what needs human judgment. Produces reviews/<name>.md with Blocker/Warning/Note findings.
---

# Review — Implementation Validation Skill

Validates a completed implementation against its spec and plan. Fixes issues within its authority. Reports issues that require human judgment. Produces `reviews/<name>.md` as a structured record of findings and actions.

---

## Invocation

```
/review plans/<feature-name>.md
```

If invoked without a path, lists plans in `plans/` whose status is `Complete` or whose tasks are all checked off, and asks which to review.

---

## Phase 0: Read Everything

Before making any judgment, read all of:

1. **The spec** (`specs/<name>.md`) — the source of truth for intended behavior, scope, types, and constraints
2. **The plan** (`plans/<name>.md`) — the task breakdown; used to check completeness and scope
3. **All implementation files** touched by this plan — read each one in full
4. **All test files** in `tests/` related to this feature
5. **`Cargo.toml`** — check for new dependencies added during Execute
6. **Prior reviews** in `reviews/` — don't re-report issues already known and accepted

Do not form judgments while reading. Finish reading everything first, then assess.

---

## Phase 1: Run the Toolchain

Before any manual review, run the full toolchain. Failures here are automatic Blockers.

```bash
cargo fmt --check
cargo test
cargo clippy -- -D warnings
```

- `cargo fmt --check` — reports unformatted code without changing it. If it fails, run `cargo fmt`, then continue.
- `cargo test` — all tests must pass. Any failure is a Blocker.
- `cargo clippy -- -D warnings` — any warning is a Blocker.

Fix toolchain failures before proceeding to manual review. There is no point reviewing code that doesn't compile, doesn't pass tests, or has unfixed lint.

---

## Phase 1b: Playtest

After the toolchain is green, run the game:

```bash
cargo run
```

Observe and record:
- Does the feature behave as the spec describes, visually and interactively?
- Are there frame drops, stutters, or rendering glitches introduced by this feature?
- Does anything feel wrong that tests cannot capture — timing, responsiveness, visual feedback?
- Does the game crash or panic under normal use?

Record observations in the review report under a **Playtest** section. A crash or panic is a Blocker. Visual or feel issues that contradict the spec are Warnings. Subjective feel notes go under Notes for the user's acceptance judgment.

If `cargo run` cannot be executed in the current environment (headless CI, no display), note this explicitly in the report and flag the playtest as deferred to user acceptance.

---

## Phase 2: Review Dimensions

Work through each dimension in order. For each finding: classify it (Blocker / Warning / Note), then act on it or record it.

---

### Dimension 1 — Plan Completeness

**Question: Did Execute implement everything the plan asked for?**

Go through every task and subtask in the plan. For each `[x]` item, verify the corresponding code exists and does what the task description says. A checked-off task with no corresponding implementation is a Blocker.

Also check: are there any `[ ]` items still unchecked? If so, Execute did not finish. Stop the review, return the plan to Execute, and note which tasks remain.

**Findings:**
- Unchecked tasks remaining → **Blocker** (do not continue review)
- Checked task with no corresponding code → **Blocker**
- Task implemented differently than described but correctly per spec → **Note** (record the deviation)

---

### Dimension 2 — Scope

**Question: Did Execute write more than the plan asked for?**

Look for:
- Functions, types, or modules that appear in the code but correspond to no plan task and no spec requirement
- Configuration options, abstraction layers, or extensibility hooks added speculatively
- Tests that test behavior not described in the spec

Extra code is not automatically bad — sometimes it's necessary glue. But it must be justified. Unjustified additions are scope creep.

**Action:** For each piece of extra code found, determine:
- Is it genuinely necessary for the planned code to work? → **Note** (record it, keep it)
- Is it speculative or added "just in case"? → **Warning** (remove it)

Remove speculative additions. Run `cargo test` after removal to confirm nothing breaks. If tests break on removal, the tests were testing the unplanned code — remove those tests too, or escalate to Blocker if the dependency is deeper than expected.

---

### Dimension 3 — Spec Satisfaction

**Question: Does the implementation fulfill the spec's stated objectives?**

This is the most important dimension. Cross-reference every behavioral requirement in the spec against the implementation:

- Each requirement in the spec's **Behavior** section → is it implemented?
- Each type in the spec's **Key Types** section → does the implementation match the intended shape?
- Each error handling strategy → is it followed?
- Each edge case listed in the spec → is there a test for it? Is it handled correctly?
- Performance constraints (if any) → any obvious violations?

**Findings:**
- Spec requirement not implemented → **Blocker**
- Spec type implemented with different fields/variants (without clear justification) → **Blocker**
- Error handling strategy not followed (e.g., `.unwrap()` in production code) → **Blocker**
- Edge case with no test coverage → **Warning** (write the missing test)
- Implementation improves on spec in a clear way → **Note** (record as spec drift — spec should be updated)

---

### Dimension 4 — Security

**Question: Are there security problems in the implementation?**

For a Rust game project, the relevant surface is narrower than a networked service, but still real. Check for:

- **Unsafe blocks** — is any `unsafe` code present? Is it justified, minimal, and accompanied by a safety comment explaining the invariant?
- **Integer overflow** — arithmetic on game values (positions, velocities, scores) that could panic in debug or wrap silently in release. Use checked arithmetic or saturating arithmetic where overflow is possible.
- **Unbounded collections** — `Vec` or `HashMap` that grows without a cap. In a game loop, unbounded growth causes frame drops or OOM. Every collection that can grow at runtime needs a documented upper bound or eviction policy.
- **Panicking code in the game loop** — `.unwrap()`, `.expect()`, `index[]` access, integer division. Any panic in a game loop crashes the game. Production paths must handle failure gracefully.
- **File I/O or serialization** (if present) — user-controlled input that gets deserialized or used in a path must be validated.
- **Dependency vulnerabilities** — if new crates were added, run `cargo audit` if available. Note any crates that are unmaintained or have known advisories.

**Findings:**
- Unjustified `unsafe` block → **Blocker**
- Panic-capable code in game loop hot path → **Blocker**
- Unbounded collection with no documented cap → **Warning**
- Overflow-susceptible arithmetic on untrusted or unbounded values → **Warning**
- `cargo audit` advisory on a new dependency → **Blocker**

**Action:** Fix Blockers. Add bounds checks, replace panicking calls with handled alternatives, document `unsafe` invariants. For Warnings, fix or document a deliberate acceptance with reasoning.

---

### Dimension 5 — Clean Code

**Question: Do the clean code principles from Execute hold across the whole feature?**

The per-task refactor in Execute catches local issues. Review catches cross-feature patterns that only emerge once everything is built together. Look for:

**Naming drift** — names that made sense for individual tasks but are inconsistent or misleading when seen together. E.g., `spawn_asteroid` in one module and `create_rock` in another for the same concept.

**Function length and responsibility** — scan every function introduced by this feature. Any function over ~25 lines or with multiple distinct logical sections needs to be broken down. Do it now.

**Repeated patterns across modules** — logic that appears in two or more places that wasn't visible as duplication during task-level work. Extract the shared concept into a named function or type.

**Constants that are still literals** — magic numbers that survived the per-task pass.

**Comment quality** — comments that restate the code, comment-out dead code, or are just noise. Delete them.

**Visibility** — every `pub` item. Ask: should this be `pub(crate)`? Should it be private? Minimize the public surface.

**Action:** Fix all of these directly. Run `cargo test` after each structural change to confirm no regressions. These are not findings to report — they are things to fix silently and note in the review as "Refactored: [description]" under a dedicated section.

---

### Dimension 6 — Test Quality

**Question: Are the tests good tests?**

Passing tests are necessary but not sufficient. Review the test suite for:

**Coverage gaps** — spec behaviors and edge cases with no test. Write missing tests.

**Testing implementation instead of behavior** — a test that breaks if you rename a private function (but not if you change what the function does) is testing the wrong thing. Tests must be behavior-anchored.

**Test names that don't describe behavior** — rename tests that don't clearly communicate what they prove. A test suite should read like a specification.

**Redundant tests** — multiple tests that exercise the exact same behavior path with no meaningful variation. Consolidate.

**Brittle setup** — tests with excessive setup that makes them hard to read or maintain. Extract setup helpers with clear names.

**Tests that pass trivially** — a test that would pass even with an empty implementation is not testing anything. `assert!(true)` or an assertion that can never fail.

**Action:** Write missing tests, rename tests, remove redundant tests, extract setup helpers. Run `cargo test` after changes. Record written and removed tests in the review report.

---

### Dimension 7 — Macro-Level Structural Refactor

**Question: Now that the whole feature exists, is the structure right?**

This is the cross-cutting refactor pass that Execute cannot do mid-task. Look at the feature as a whole:

**Module boundaries** — do the module divisions make sense? Did a module grow into two distinct responsibilities during implementation? Should two modules be merged because they're always used together?

**Shared abstractions** — did two systems evolve independently that would benefit from a shared trait or type? This is only visible after both are written.

**Layer violations** — is game logic leaking into rendering code? Is data manipulation happening in a system that should only coordinate? Are the right things in the right places?

**Type design** — do the types feel right to use? Is anything awkward to construct or pattern-match? Awkward types usually indicate a design mismatch with the domain.

**Action:** Make structural changes, one at a time. After each change, run `cargo test`. A failing test after a structural refactor means either a bug was introduced (revert, retry) or the test was coupled to structure rather than behavior (fix the test). Record significant restructuring in the review as "Restructured: [description]".

---

### Dimension 8 — API Surface

**Question: Is the public API minimal and intentional?**

Audit every `pub` item added by this feature:

- Is it `pub` because external code genuinely needs it, or because it was convenient during implementation?
- Could it be `pub(crate)` and still serve its purpose?
- Is anything missing from the public API that the spec implies should be there?
- Does the API feel natural to use? Would a new developer understand what to call and in what order?

**Action:** Tighten visibility wherever possible. Add any missing public items. Record visibility changes in the review.

---

### Dimension 9 — Spec Drift

**Question: Did the implementation improve on the spec in ways that should be captured?**

Sometimes implementation reveals that the spec was slightly wrong — a type needs an extra field, a behavior needs a subtlety, a constraint was overly restrictive. If the implementation is better than the spec, the spec is now wrong.

Record all spec drift found. After the review is complete, update the spec to match reality. A spec that doesn't match the code is a lie waiting to mislead the next feature.

**Action:** Update `specs/<name>.md` to reflect what was actually built. Mark each update clearly with a note like `(updated post-implementation)` so the history is visible.

---

### Dimension 10 — Dependency Audit

**Question: Were any new dependencies added, and are they justified?**

Check `Cargo.toml` for crates added during Execute that weren't present before this feature. For each new dependency:

- Is it actually used? (`cargo +nightly udeps` if available, or manual check)
- Could the need have been met by an already-present crate or std?
- Is it actively maintained?
- Run `cargo audit` if available

**Findings:**
- Unused dependency → **Warning** (remove it)
- Need met by existing crate → **Warning** (remove the new dep, use the existing one)
- Known advisory → **Blocker**
- Unmaintained crate → **Note**

---

### Dimension 11 — Documentation

**Question: Has `/docs` been run for this feature?**

Check `docs/technical/` and `docs/user/` for entries covering this feature.

- If `/docs` has not been run yet, this is a **Warning** — note it in the report and remind the user to run `/docs plans/<name>.md` after review passes.
- If docs exist, spot-check them: does the technical doc accurately describe what was built? Does the user doc reflect the actual player-facing behavior?
- Outdated or inaccurate docs are a **Warning** — flag for correction in the next `/docs` pass.

Documentation is not a Blocker for review passing, but its absence must be recorded. A feature is not fully shipped until docs exist.

---

## Phase 3: Write the Review Report

Save to `reviews/<feature-name>.md`.

```markdown
# Review: [Feature Name]

**Plan:** `plans/<feature-name>.md`
**Spec:** `specs/<feature-name>.md`
**Date:** [date]
**Outcome:** [Passed / Passed with fixes / Blocked — see findings]

---

## Summary

[2-4 sentences. Overall quality of the implementation. What was done well. What needed fixing.]

---

## Toolchain

- `cargo fmt`: [Clean / Fixed N files]
- `cargo test`: [N passed / failures fixed]
- `cargo clippy`: [Clean / N warnings fixed]

---

## Playtest

- `cargo run`: [Ran successfully / Could not run — headless environment]
- Visual behavior: [Matches spec / Issues noted below]
- Performance: [No issues observed / Issues noted below]
- Crashes/panics: [None / Details]

[Any observations for user acceptance — feel, timing, visual quality]

---

## Findings

### Blockers
[List each Blocker with: description, location (file:line), action taken or required]

- **[B1]** `src/systems/asteroid.rs:42` — `.unwrap()` on `Option` in game loop hot path. **Fixed:** replaced with early return on `None`.

### Warnings
[List each Warning with: description, location, action taken]

- **[W1]** `src/systems/physics.rs` — `MAX_SPEED` used as literal `500.0` in three places. **Fixed:** extracted to `const MAX_ASTEROID_SPEED: f32`.

### Notes
[Observations that don't require action but are worth recording]

- **[N1]** Implementation chose `Vec<Asteroid>` over the spec's suggested `HashMap<AsteroidId, Asteroid>`. This is simpler and correct for current scale. Spec updated to reflect this choice.

---

## Fixes Applied

### Refactoring
[Structural changes made during review]

- Extracted `fn calculate_split_velocity(parent: &Asteroid) -> Vec2` from `fn on_asteroid_death` — the velocity logic was a distinct sub-operation
- Merged `asteroid_spawner.rs` and `asteroid_field.rs` — both modules operated on the same data with no clean boundary between them

### Tests Added
[New tests written to close coverage gaps]

- `tests/asteroid_tests.rs::splitting::test_split_velocity_is_perpendicular_to_parent` — spec requires split children travel perpendicular to parent velocity; was untested
- `tests/asteroid_tests.rs::edge_cases::test_maximum_asteroid_count_is_not_exceeded` — unbounded spawn was possible; test now enforces the cap

### Tests Removed
[Tests deleted and why]

- `tests/asteroid_tests.rs::test_internal_state_matches_enum_ordinal` — testing implementation detail, not behavior; fragile and meaningless

### Spec Updates
[Changes made to the spec to reflect actual implementation]

- `specs/asteroid-field.md` — updated Key Types to reflect `AsteroidPool` using `Vec` not `HashMap`
- `specs/asteroid-field.md` — added `MAX_ASTEROID_COUNT` constant to performance constraints section

---

## Outcome

[One of:]

**Passed** — all blockers resolved, no outstanding warnings, suite green.
Update plan status to `Complete ✓`.

**Passed with Notes** — all blockers resolved, warnings fixed, notes recorded for future consideration.
Update plan status to `Complete ✓`.

**Blocked** — one or more blockers could not be resolved by Review alone and require human judgment.
List each blocker with what decision is needed. Plan status remains `In progress`.
```

---

## Phase 4: Update the Plan and Roadmap

If the review **Passed** or **Passed with Notes**:
1. Edit `plans/<name>.md` — update Status to `Complete ✓`
2. Edit `ROADMAP.md` — find the corresponding item and mark it `[x]`
3. If `/docs` has not been run yet, remind the user: "Run `/docs plans/<name>.md` to complete documentation for this feature."

If the review is **Blocked**: leave the plan status and roadmap item as-is. Surface the blockers clearly so the user can decide.

---

## What Review Fixes vs. Reports

| Issue Type | Review Action |
|------------|---------------|
| Unformatted code | Fix silently |
| Clippy warning | Fix silently |
| Clean code violation (naming, length, magic value) | Fix silently |
| Missing test for known edge case | Write the test |
| Redundant or poorly named test | Fix silently |
| Structural refactor opportunity | Refactor, record in report |
| Scope creep (speculative code) | Remove, record in report |
| Spec drift (implementation improved on spec) | Update spec, record in note |
| Visibility too broad | Tighten, record in report |
| Security issue (unsafe, panic in hot path) | Fix if mechanical; escalate if architectural |
| Spec requirement not implemented | **Escalate — Blocker** |
| Ambiguous spec (genuine design decision needed) | **Escalate — Blocker** |
| Security issue requiring design decision | **Escalate — Blocker** |
| Dependency advisory | **Escalate — Blocker** |
