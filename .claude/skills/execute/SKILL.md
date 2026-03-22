---
name: execute
description: Implements a plan file task by task using TDD. Writes tests first, stubs to compile, verifies behavioral failure, then implements. Checks off plan tasks as it goes.
---

# Execute — Plan-to-Code Skill

Works through a `plans/<name>.md` file task by task, implementing each one using test-driven development. Checks off `[ ]` boxes in the plan as work is completed.

---

## Invocation

```
/execute plans/<feature-name>.md
```

If invoked without a path, lists available plans in `plans/` and asks which to execute.

---

## Phase 0: Read Before Touching Anything

Before writing a single line of code:

1. **Read the plan file** — understand the full task tree, ordering, checkpoints, and any notes for Execute.
2. **Read the spec** — the plan references it. Understand the intended behavior, types, and error handling strategy. The spec is the source of truth when plan and code diverge.
3. **Read the codebase** — understand the existing module structure, naming conventions, error types, and patterns. Code written by this skill must be indistinguishable in style from code already in the project.
4. **Check what's already done** — any `[x]` tasks in the plan are already complete. Start from the first unchecked `[ ]`.

Do not skip this phase. Writing code without reading the existing codebase produces integration failures and style inconsistencies.

---

## Phase 1: The TDD Loop

For every leaf task (subtask), execute this loop in full. No shortcuts.

### Step 1 — Write the Test

Write the test for the behavior this task implements **before writing any implementation code**.

- Save it to `tests/` (see Test Organization below)
- The test must be specific to the behavior described in the task, not a generic smoke test
- Use the naming convention: `test_<what_it_does_in_plain_english>` (see Naming below)
- Structure with Arrange / Act / Assert — one clear setup, one action, one assertion cluster
- The test should read like a specification of the behavior

At this point the test **will not compile** because the types/functions it calls don't exist yet. That is expected. Move to Step 2.

### Step 2 — Write the Stub

Write the minimum code needed to make the test **compile but not pass**:

- Define the types, structs, enums, and function signatures the test references
- Function bodies should be stubs: `todo!()`, `unimplemented!()`, or a default/wrong return value
- Do not implement any logic yet
- The goal is a green compiler with a red test

**Verify the stub is correct:**
```bash
cargo build
```
Must compile without errors. Warnings are acceptable at this stage if they'll be resolved by the implementation.

### Step 3 — Run the Test, Verify Behavioral Failure

```bash
cargo test <test_name>
```

Read the failure output carefully. The test must fail for a **behavioral reason** — wrong return value, wrong state, assertion mismatch — not a compile error or a `todo!()` / `unimplemented!()` panic.

**If the failure is `todo!()` or `unimplemented!()`:** Replace the stub body with a default/wrong return value that compiles. For example, a function returning `bool` should return `false`. A function returning `Vec<T>` should return `vec![]`. The test should then fail because the behavior is wrong, not because execution was aborted.

**If the failure is a compile error:** Fix the stub. Do not proceed to Step 4 with a compile error.

**Only move to Step 4 once the test fails with a meaningful assertion message.**

### Step 4 — Implement the Behavior

Now write the actual implementation. Apply all clean code principles (see below). The implementation should be exactly what's needed to make this test pass — nothing more.

Resist the urge to implement adjacent functionality "while you're in there." Each task is its own focused implementation.

### Step 5 — Run the Test, Verify Pass

```bash
cargo test <test_name>
```

The test must pass. If it doesn't:

**→ Diagnose before acting.** Read the failure. Ask: is the test correct, or is the implementation correct? (See Failure Triage below.)

Do not move to Step 6 until this test passes cleanly.

### Step 6 — Run the Full Suite

```bash
cargo test
```

All previously passing tests must still pass. If a pre-existing test now fails, the implementation has a regression. Fix the regression before proceeding — do not suppress or delete tests to make this green.

### Step 7 — Refactor

With tests green, look at the code just written and ask: *is this as clean as it can be?*

This is the **Red → Green → Refactor** cycle closing. Do not skip it.

**What to look for:**

- **Functions doing more than one thing.** If a function has a blank-line-separated section that could be named and extracted, extract it. The extracted function's name becomes documentation.
- **Functions longer than ~20–25 lines.** Length is a smell that multiple responsibilities are present.
- **Duplicated logic.** If the same pattern appears twice within this task's new code, extract it now before it appears a third time elsewhere.
- **Names that no longer fit.** Sometimes a name made sense before implementation; once the code exists, a better name is obvious. Rename now.
- **Magic values that should be constants.** Any literal that acquired meaning during implementation.
- **Logic that belongs elsewhere.** A helper that would be more at home in a different module.

**How to refactor safely:**

1. Make one structural change at a time — extract one function, rename one thing
2. Run `cargo test` after each change
3. A failing test after a refactor means either the refactor introduced a bug (most likely — revert and retry) or the original code had an untested assumption that the refactor exposed (investigate before proceeding)
4. Never change behavior during refactor — if you spot a behavioral improvement, note it as a future task and keep moving

**When to stop:**

Stop when every function does one thing, every name is honest, and you would be comfortable having another developer read this code cold. The goal is not perfection — it is clarity.

### Step 8 — Format and Lint

```bash
cargo fmt
cargo clippy -- -D warnings
```

Run `cargo fmt` first — always. Formatting is not a style preference, it is a hard rule. Never commit unformatted code.

Then run clippy. Zero warnings. If clippy flags something, fix it — don't `#[allow(...)]` it unless the spec explicitly calls for an exception, and document why if so.

### Step 9 — Check Off the Task

Edit the plan file, changing the task's `[ ]` to `[x]`.

Then move to the next task and repeat from Step 1.

---

## Test Organization

All tests live in the `tests/` directory at the crate root. **No `#[cfg(test)]` modules inside source files.**

```
src/
  lib.rs
  systems/
    physics.rs
  components/
    asteroid.rs
tests/
  physics_tests.rs
  asteroid_tests.rs
  collision_tests.rs
```

### Accessing Internals

`tests/` can only access `pub` and `pub(crate)` items. When a test needs to exercise internal behavior:

- Prefer testing through the public API — if behavior is visible externally, test it externally
- If internal state must be verified, expose it with `pub(crate)` — not `pub` — and document why
- Never make something `pub` solely for testing

### One Test File Per Module Under Test

`tests/asteroid_tests.rs` tests `src/components/asteroid.rs`. Keep the mapping 1:1. For cross-cutting integration tests (e.g. "asteroid spawns, moves, then splits"), use `tests/integration/` with a descriptive name.

---

## Failure Triage

When a test fails after an implementation attempt, stop and reason before acting.

### The Three Questions

1. **Is the test testing the right thing?** Re-read the spec. Does the test accurately describe the intended behavior? If the test is wrong (wrong expected value, wrong setup, tests a behavior the spec doesn't require), fix the test.

2. **Is the implementation wrong?** Does the code fail to implement what the spec says? If so, fix the implementation.

3. **Is the spec ambiguous?** If neither the test nor the implementation is clearly wrong, the spec has a gap. Stop, note the ambiguity explicitly (as a comment in the plan's Open Questions), and make a documented assumption. Do not silently pick one interpretation.

### Never Do This

- Don't change the expected value in an assertion just to make a test pass without understanding why the value is different.
- Don't delete a failing test.
- Don't `#[ignore]` a failing test to move forward (unless the plan explicitly defers it and you note it).
- Don't blindly implement until a test passes by trial and error — understand the failure first.

---

## Clean Code Principles

These apply to every line of code written by this skill.

### Naming

**Names are documentation.** The code must read like prose. A reader should understand what a function does from its name alone without reading its body.

| Bad | Good |
|-----|------|
| `fn upd(a: &mut Ast)` | `fn apply_velocity(asteroid: &mut Asteroid)` |
| `let x = 3` | `let spawn_delay_seconds = 3` |
| `fn chk(a: &Asteroid, b: &Asteroid) -> bool` | `fn asteroids_are_colliding(a: &Asteroid, b: &Asteroid) -> bool` |
| `let flag = true` | `let is_visible = true` |
| `data`, `info`, `manager`, `handler` | Specific nouns: `asteroid_field`, `collision_event`, `spawn_timer` |

**Rules:**
- No single-letter names except loop indices (`i`, `j`) and well-established math variables (`x`, `y`, `dt`)
- No abbreviations unless they are domain-standard (e.g. `fps`, `hp`, `dt` for delta time)
- Booleans are named as questions: `is_alive`, `has_collided`, `can_split`
- Functions are named as verb phrases: `split_asteroid`, `apply_gravity`, `despawn_off_screen`
- Types are named as nouns: `AsteroidField`, `CollisionEvent`, `SpawnConfig`

### Constants Over Magic Values

Every literal that represents a domain concept must be a named constant.

```rust
// Bad
if velocity.length() > 500.0 { ... }
let timer = Timer::from_seconds(3.0, TimerMode::Once);

// Good
const MAX_ASTEROID_SPEED: f32 = 500.0;
const RESPAWN_DELAY_SECONDS: f32 = 3.0;

if velocity.length() > MAX_ASTEROID_SPEED { ... }
let timer = Timer::from_seconds(RESPAWN_DELAY_SECONDS, TimerMode::Once);
```

Constants live at the top of the module they belong to, or in a `constants.rs` if they're shared across modules. Never scatter literals through function bodies.

### Functions Do One Thing

A function should have one reason to exist and one reason to change.

**Length guide:** If a function body doesn't fit comfortably on one screen (~30 lines), it's probably doing more than one thing. Extract the distinct sub-operations into named helpers.

**Signs a function is doing too much:**
- It has `and` or `or` in the name: `update_and_render`, `check_and_apply`
- It has sections with blank lines separating unrelated logic
- It takes more than ~4 parameters (consider a config struct)
- It has nested conditionals more than 2 levels deep

**Prefer early returns** over nested conditions:
```rust
// Bad
fn process(asteroid: Option<&Asteroid>) -> f32 {
    if let Some(a) = asteroid {
        if a.is_alive {
            return a.speed * 2.0;
        } else {
            return 0.0;
        }
    } else {
        return 0.0;
    }
}

// Good
fn process(asteroid: Option<&Asteroid>) -> f32 {
    let Some(asteroid) = asteroid else { return 0.0 };
    if !asteroid.is_alive { return 0.0 }
    asteroid.speed * 2.0
}
```

### Comments: Rare and Meaningful

The code explains *what*. Comments explain *why* — and only when the why is non-obvious.

**Write a comment when:**
- A non-obvious algorithm or formula is used (link to the reference or explain the math)
- A workaround exists for a known external bug or limitation
- An intentional deviation from what the reader would expect
- A safety invariant must hold for correctness

**Never write a comment when:**
- The code is self-explanatory
- The comment just restates what the function does (`// increment counter` above `count += 1`)
- The information is already in the function/variable name
- You're commenting out dead code (delete it instead)

### No Dead Code

If code isn't used, delete it. Don't comment it out, don't mark it `#[allow(dead_code)]`. Rust's compiler will tell you what's unused. Trust it.

Exception: `pub` items that are part of a library's public API but not exercised internally.

### Error Handling

Follow the strategy defined in the spec. In the absence of spec guidance:

- Use `?` to propagate errors — don't `unwrap()` in production code
- `.unwrap()` and `.expect()` are only acceptable in tests and in clearly-documented invariant assertions
- If using `.expect()` in production, the message must explain the invariant: `"asteroid pool must be initialized before systems run"`
- Prefer typed errors over `Box<dyn Error>` for errors that callers need to handle
- Infallible operations return plain values, not `Result`

### Enums Over Booleans

Boolean parameters are unreadable at call sites:

```rust
// Bad — what does `true` mean here?
spawn_asteroid(position, true);

// Good
spawn_asteroid(position, AsteroidVariant::Large);
```

If a function takes a boolean that controls its behavior, replace it with an enum variant.

### Exhaustive Pattern Matching

Always use `match` over `if let` chains when handling enum variants. Never use `_ =>` as a catch-all unless the remaining variants genuinely don't matter and you've consciously decided that. Exhaustive matching means the compiler catches missing cases when new variants are added.

```rust
// Bad — new AsteroidSize variants will silently fall through
match size {
    AsteroidSize::Large => split_into_medium(),
    _ => despawn(),
}

// Good — compiler error if a new variant is added
match size {
    AsteroidSize::Large => split_into_medium(),
    AsteroidSize::Medium => split_into_small(),
    AsteroidSize::Small => despawn(),
}
```

### DRY — But Not Prematurely

Extract duplication when the same logic appears in **three or more places** and represents the same **concept**. Don't extract two similar-looking pieces of code that happen to look alike now but represent different domain concepts — they will diverge and the extraction will become a liability.

When you extract, name the abstraction after the concept, not the mechanism.

### Module Cohesion

Keep related things together. If two types are always used together and one doesn't make sense without the other, they belong in the same module. If a module is growing too large, split along conceptual lines (not line-count lines).

---

## Checkpoint Validation

At the end of each task group (as marked in the plan), verify the checkpoint:

1. Run `cargo fmt` — no formatting changes remain
2. Run `cargo test` — all tests pass
3. Run `cargo clippy -- -D warnings` — zero warnings
4. Run `cargo build` — clean compile
5. All functions introduced in this group do one thing and fit comfortably on screen
4. Confirm observable behavior matches the checkpoint description in the plan (e.g. "asteroids render but don't move")

Only mark a task group complete after the checkpoint passes. Update the plan's group header with `[x]` on each subtask and note the checkpoint as verified.

---

## Test Naming Convention

Test names must describe the behavior, not the implementation:

```rust
// Bad
fn test_asteroid_split()
fn test_fn_returns_true()
fn asteroid_test_1()

// Good
fn test_large_asteroid_splits_into_two_medium_asteroids()
fn test_small_asteroid_is_destroyed_without_splitting()
fn test_asteroid_speed_is_clamped_to_maximum()
fn test_off_screen_asteroid_wraps_to_opposite_edge()
```

Format: `test_<subject>_<condition>_<expected_outcome>` when applicable.

Group related tests using descriptive module names inside the test file:

```rust
// tests/asteroid_tests.rs

mod splitting {
    #[test]
    fn test_large_splits_into_two_medium() { ... }

    #[test]
    fn test_medium_splits_into_two_small() { ... }

    #[test]
    fn test_small_is_destroyed_on_death() { ... }
}

mod movement {
    #[test]
    fn test_velocity_is_applied_each_frame() { ... }

    #[test]
    fn test_wraps_at_screen_boundary() { ... }
}
```

---

## Completing the Plan

When all tasks are checked off:

1. Run `cargo fmt` — no formatting changes remain
2. Run the full test suite one final time: `cargo test`
3. Run clippy: `cargo clippy -- -D warnings`
3. Update the plan's **Status** field from `Not started` / `In progress` to `Complete`
4. Tell the user: the plan is complete, all tasks checked off, suite passing. Suggest running `/review plans/<name>.md` to validate the implementation against the spec.
