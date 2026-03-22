---
name: plan
description: Reads a spec file and produces a structured Plan file (plans/*.md) with hierarchical checkbox tasks ready for the Execute skill to implement.
---

# Plan — Spec-to-Implementation-Plan Skill

Reads a specification from `specs/` and produces a detailed, executable `plans/<name>.md` file — a hierarchical task breakdown with checkbox tracking that the `/execute` skill will work through step by step.

---

## Invocation

```
/plan specs/<feature-name>.md
```

If the user invokes `/plan` without a path, list the available specs in `specs/` and ask which one to plan.

---

## Phase 1: Read Everything First

Before producing a single task, read and internalize:

1. **The spec file** passed as argument — understand scope, key types, architecture, behavior, error handling, and open questions.
2. **The codebase** — scan `Cargo.toml`, `src/`, and `build.rs` if present. Understand:
   - Existing modules and how the new feature must integrate
   - Current dependency versions (so you don't plan for a dep that isn't there)
   - Established patterns: ECS setup, state machine conventions, error types, test structure
3. **Existing plans** in `plans/` — don't re-plan anything already covered. Identify dependencies on other plans.
4. **Existing specs** in `specs/` — cross-referenced decisions that constrain task ordering.

Do not skip this phase. A plan written without reading the code will produce tasks that fight the existing architecture.

---

## Phase 2: Decompose Into Tasks

### Task Decomposition Rules

**Tasks must be implementation steps, not spec summaries.**
Bad: `- [ ] Implement asteroid splitting`
Good: `- [ ] Add \`split()\` method to \`Asteroid\` that returns \`Option<[Asteroid; 2]>\` based on size variant`

**Each leaf task (subtask) must be completable in one focused coding session** — roughly 30–90 minutes of work. If a task would take longer, break it down further.

**Tasks must be ordered for a clean build.** The codebase should compile (or at least have no new errors introduced) after each top-level task group is complete. Stub/placeholder implementations are acceptable between tasks, but broken code is not.

**Prefer bottom-up ordering for new features:**
1. Data types and structs first
2. Trait implementations second
3. Systems/logic third
4. Integration and wiring fourth
5. Tests last (or interleaved if TDD is implied by the spec)

**Make dependencies explicit.** If Task 3 requires Task 1 to be done first, say so.

### Task Granularity Guide

| Work Item | Appropriate Level |
|-----------|------------------|
| Define a new `struct` or `enum` | Subtask |
| Implement a `trait` for a type | Subtask |
| Write a single system function | Subtask |
| Wire a system into the game loop | Subtask |
| Add a new module file | Subtask |
| Write tests for one function/system | Subtask |
| Complete a subsystem (e.g. "collision detection") | Top-level task with subtasks |
| Complete a cross-cutting concern (e.g. "error handling") | Top-level task with subtasks |

---

## Phase 3: Write the Plan File

Save to `plans/<feature-name>.md`. Use the same kebab-case name as the spec.

### Plan File Format

```markdown
# Plan: [Feature Name]

**Spec:** `specs/<feature-name>.md`
**Status:** Not started
**Created:** [date]

---

## Overview

[2-4 sentences. What will be built, in what order, and why that order. Name any significant integration points or risks.]

---

## Prerequisites

- [ ] [Any task from another plan that must be complete before this one starts — with link]
- [ ] [Or "None" if there are no prerequisites]

---

## Tasks

### 1. [Task Group Name]

> [One sentence explaining what this group accomplishes and why it comes first]

- [ ] **1.1** [Concrete subtask — be specific about file, type, function, or module]
- [ ] **1.2** [Concrete subtask]
- [ ] **1.3** [Concrete subtask]

*Checkpoint: [What the codebase state looks like when this group is done — e.g., "compiles cleanly, new types are defined but not yet wired"]*

---

### 2. [Task Group Name]

> [One sentence. What this group accomplishes. May note dependency: "Requires group 1."]

- [ ] **2.1** [Concrete subtask]
  - [ ] **2.1.1** [Sub-subtask if needed — use sparingly, only for genuinely nested work]
  - [ ] **2.1.2** [Sub-subtask]
- [ ] **2.2** [Concrete subtask]
- [ ] **2.3** [Concrete subtask]

*Checkpoint: [Codebase state]*

---

### 3. [Continue for all task groups...]

---

## Open Questions

[Copy any "Open Questions" from the spec that are still unresolved and would block or alter implementation. If the plan can proceed without resolving them, note the assumption made.]

- [ ] [Question — and the assumption made if proceeding anyway]

---

## Notes for Execute

[Any implementation guidance that doesn't fit in a task description — crate versions to use, known footguns in the existing code, patterns to follow or avoid, links to relevant docs.]
```

---

## Task Writing Standards

### Be Rust-specific

Every task description should be precise enough that the implementer knows exactly what to write.

| Vague | Precise |
|-------|---------|
| Add asteroid component | Add `#[derive(Component)] pub struct Asteroid { size: AsteroidSize, velocity: Vec2 }` to `src/components/asteroid.rs` |
| Handle collision | Implement `fn check_collision(a: &Transform, b: &Transform, radius: f32) -> bool` in `src/systems/collision.rs` |
| Add tests | Write `#[test] fn asteroid_large_splits_into_two_medium()` asserting split count and size variants |
| Wire into game loop | Register `collision_system` in `App::add_systems(Update, collision_system)` in `src/main.rs` |

### Reference real paths and types

Tasks must name:
- The file to create or modify: `src/systems/asteroid.rs`
- The type, function, or impl block to add
- The trait being implemented (if any)
- The crate or module to add it to

### Stub-first for compile hygiene

When a task introduces a type that other tasks depend on, the first subtask should be to write a minimal stub that compiles. Later subtasks fill it in. This keeps the build green throughout.

Example:
```
- [ ] **1.1** Create `src/systems/physics.rs` with stub `pub fn physics_system()` that compiles (empty body)
- [ ] **1.2** Add `physics_system` to `mod.rs` and register in `App`
- [ ] **1.3** Implement velocity integration in `physics_system`
- [ ] **1.4** Implement boundary wrapping in `physics_system`
```

### Test tasks belong at the end of their group

Don't put a test task before the code it tests. Tests are the last subtask in each group, or gathered into a dedicated final group for integration tests.

---

## Checkpoint Annotations

Every top-level task group ends with a `*Checkpoint*` line. This tells the Execute skill what to verify before moving to the next group:

- **Compile status**: "compiles cleanly" / "compiles with warnings" / "stubs in place, won't link yet"
- **Test status**: "all existing tests pass" / "new unit tests pass" / "no tests yet"
- **Observable behavior**: "nothing visible yet" / "asteroids render but don't move" / "full feature working"

Checkpoints are how the Execute skill knows when a group is truly done, not just superficially checked off.

---

## Quality Check Before Saving

Before saving the plan, verify:

- [ ] Every task is a concrete action, not a spec summary
- [ ] Tasks are ordered so the build is always in a valid (if incomplete) state
- [ ] Each top-level group has a checkpoint
- [ ] File paths, type names, and function signatures are named where known
- [ ] Open questions from the spec are carried forward (not silently dropped)
- [ ] The plan is complete enough that Execute could work through it without reading the spec
