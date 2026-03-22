---
name: refine
description: Interactive specification refinement skill. Takes a rough idea and produces a polished spec (specs/*.md) via iterative Q&A and collaborative Excalidraw diagrams. Built for Rust projects.
---

# Refine — Idea-to-Specification Skill

Transform a partially-formed idea into a complete, actionable specification through structured dialogue and visual collaboration. The output is a markdown spec file in `specs/` plus supporting Excalidraw diagrams.

---

## How This Skill Works

1. **User provides a rough prompt** — incomplete ideas, half-baked features, vague desires. All welcome.
2. **You read the codebase** — understand what already exists so questions are grounded in reality.
3. **Iterative Q&A loop** — ask exactly **one question at a time**. Never a list. Never multiple questions in one turn.
4. **Visual collaboration** — use the `excalidraw-diagram` skill frequently to draw diagrams. The user can edit them and share back. Treat diagrams as a bidirectional communication channel.
5. **Spec output** — when enough is known, produce `specs/<feature-name>.md` plus save all diagrams to `specs/diagrams/`.

---

## Phase 1: Orient (Before Asking Anything)

When invoked, immediately:

1. **Check for bootstrap condition** — does `Cargo.toml` exist? If not, this is the project's first refinement. The spec produced by this session must cover both the project foundation and the first feature. That means the Q&A must resolve: game framework choice, workspace structure, initial `Cargo.toml` dependencies, and the conventions that all future specs will build on. A `CLAUDE.md` establishing project-wide rules should be part of the output alongside the spec.

2. **Read the codebase** — scan `Cargo.toml`, `src/`, and any existing specs in `specs/`. Understand:
   - What crates/modules exist
   - What the game loop looks like (if present)
   - What patterns are already established (ECS, state machines, event systems, etc.)
   - What's missing that the user's idea might need

3. **Read existing specs** — don't re-specify what's already decided. Check `specs/` for prior decisions that constrain this one.

4. **Consult `ROADMAP.md`** — if it exists, read it before asking anything.
   - Is the feature the user described already on the roadmap? If so, note it — the spec will fulfill that item.
   - Does the user's idea depend on roadmap items not yet completed? Flag this early in the conversation.
   - If `ROADMAP.md` does not exist yet, it will be created as part of this session's output.

5. **Form your mental model** — before asking anything, summarize (internally) what you understand about the idea and what the biggest unknowns are. Order the unknowns by importance: structural/architectural questions first, detail questions last.

6. **Draw an initial diagram** — use the `excalidraw-diagram` skill to sketch your current understanding of the idea. This anchors the conversation. Label it clearly as "initial understanding — subject to change."

7. **State your understanding** — in 2-4 sentences, tell the user what you think they're describing. Be concrete. Then ask your first question.

---

## Phase 2: The Q&A Loop

### Core Rule
**Ask exactly ONE question per turn.** No lists. No "and also...". One question.

### Question Priority Order

Ask questions in this order of importance. Skip any that are already clear from context.

#### Tier 1 — Structural (ask first)
These questions determine the architecture. Get these right before anything else.

1. **Scope**: What is the minimum useful version of this feature? What's explicitly out of scope?
2. **Rust architecture fit**: Where does this live — a new crate, new module, new ECS component/system, new state? How does it integrate with what exists?
3. **Ownership model**: What owns the data? Who mutates it? Who reads it? Are there lifetime concerns?
4. **Concurrency**: Does this need to work across threads? Is there any async involved?

#### Tier 2 — Behavioral (ask second)
These questions define what the feature does.

5. **Core loop**: What happens every frame/tick/event? Draw a sequence diagram if non-trivial.
6. **Input/output**: What data goes in? What comes out? What are the types?
7. **Failure modes**: What can go wrong? What's the expected behavior on error? (Rust errors, not exceptions)
8. **State transitions**: What states can this be in? What triggers transitions? Draw a state machine if helpful.

#### Tier 3 — Detail (ask last)
These questions fill in specifics.

9. **Performance constraints**: Any FPS targets, memory budgets, or throughput requirements?
10. **External integration**: Does this touch any external systems — audio, physics engine, network?
11. **Testing strategy**: Unit tests, integration tests, or property-based tests? Any tricky invariants to test?
12. **Edge cases**: Boundary conditions the user has already thought about.

### How to Ask Good Questions

- **Be specific to Rust and the codebase.** Not "how will this handle errors?" but "should this return `Result<T, GameError>` or panic on invariant violations?"
- **Offer concrete options when helpful.** "Should the asteroid field be an ECS component attached to each asteroid, or a separate resource that the physics system queries?"
- **Reference the existing code.** "You already have a `PhysicsBody` component — should collision detection extend that, or live in a separate system?"
- **One level of depth per question.** Don't ask about both the data model AND the rendering in the same question.

---

## Phase 3: Visual Collaboration

The `excalidraw-diagram` skill is your primary communication tool. Use it aggressively.

### When to Draw

| Situation | Draw it |
|-----------|---------|
| User describes a flow with multiple steps | Sequence or flowchart diagram |
| Feature has distinct states | State machine diagram |
| Multiple systems interact | Component/dependency diagram |
| Data structure is non-trivial | Type/relationship diagram (show actual Rust types) |
| You and the user are talking past each other | Draw your understanding, ask "is this it?" |
| User edits a diagram you drew | Read the new diagram carefully — treat edits as answers |

### Drawing for Rust Projects

When diagramming Rust-specific concepts:
- **Show real types**: use `Vec<Asteroid>`, `Option<PlayerId>`, `Arc<Mutex<T>>` — not "list of things"
- **Show ownership**: use arrows labeled `&`, `&mut`, or owned to show borrow relationships
- **Show module boundaries**: use section containers for `mod` boundaries and crate boundaries
- **Show trait implementations**: when a type implements a key trait (`Component`, `System`, `Resource`), annotate it
- **State machines**: show `enum` variants as states, transitions as labeled arrows with the method/event that triggers them
- **ECS concepts**: show entities, components, and systems as distinct visual categories

### Bidirectional Diagram Protocol

After you draw a diagram:
1. Tell the user: "Here's my current understanding — please edit the diagram if anything is wrong."
2. Save the diagram to `specs/diagrams/<name>.excalidraw` immediately.
3. When the user makes edits or describes changes, update the diagram and note what changed.
4. Treat diagram edits as authoritative answers, same as text answers.

---

## Phase 4: Knowing When to Stop

Stop asking questions and write the spec when:

- All Tier 1 (structural) questions are answered
- The majority of Tier 2 (behavioral) questions are answered
- You could hand this spec to another developer (or another Claude session) and they could implement it without guessing

If the user says "that's enough" or "write the spec", write it regardless of how many questions remain. Use "TBD" for genuinely open items.

### Telling the User You're Ready

Before writing the spec, say: "I think I have enough to write a solid spec. Here's what I'm still uncertain about: [list]. Should I write it now, or do you want to resolve those first?"

---

## Phase 5: Writing the Specification

Save to `specs/<feature-name>.md`. Use kebab-case for the filename.

### Spec Template

```markdown
# [Feature Name]

**Status:** Draft
**Created:** [date]
**Spec author:** Refined via /refine skill

---

## Summary

[1-3 sentences. What is this feature and why does it exist?]

---

## Motivation

[Why is this needed? What problem does it solve in the game? What's the player/dev experience without it?]

---

## Scope

### In scope
- [Bullet list of what this spec covers]

### Out of scope
- [Explicitly excluded items — prevents scope creep during implementation]

---

## Architecture

### Where it lives

[Module path, crate, new file, or extension of existing code. Be specific: `src/systems/collision.rs`, not "the physics module".]

### Key types

```rust
// Show the actual Rust types — structs, enums, traits
// Use comments to explain non-obvious fields
// This is the most important section for implementation

pub struct AsteroidField {
    pub asteroids: Vec<Asteroid>,
    pub spawn_rate: f32,      // asteroids per second
    pub max_count: usize,
}

pub enum AsteroidSize {
    Large,   // splits into 2 Medium on death
    Medium,  // splits into 2 Small on death
    Small,   // destroyed on death
}
```

### Systems / ECS integration (if applicable)

[Which systems does this add/modify? What's the system ordering? What resources/components does each system read and write?]

### Module structure

[If this spans multiple files, show the intended layout]

---

## Behavior

### Core loop

[What happens every frame? Every tick? On every relevant event?]

### State machine (if applicable)

[Reference the diagram in specs/diagrams/ — don't duplicate what's already visual]
See: `specs/diagrams/<name>.excalidraw`

### Input/Output

| Input | Type | Source |
|-------|------|--------|
| [name] | `[Rust type]` | [where it comes from] |

| Output | Type | Destination |
|--------|------|-------------|
| [name] | `[Rust type]` | [where it goes] |

### Error handling

[What `Result` types are used? What errors are recoverable vs. fatal? What's the strategy — propagate, log, panic?]

---

## Edge Cases

- [Known tricky case and how to handle it]
- [Boundary condition]
- [Failure scenario]

---

## Performance Considerations

[Any budget constraints — frame time, memory, allocation frequency. If none, say "No specific constraints identified."]

---

## Testing Strategy

[Unit tests? Integration tests? What invariants must hold? Any fuzzing or property-based testing?]

```rust
// Key test cases to implement
#[test]
fn asteroid_splits_on_death() { ... }
```

---

## Open Questions

- [TBD item 1]
- [TBD item 2]

---

## Diagrams

[List all diagrams saved to specs/diagrams/ with a one-line description of each]

- `specs/diagrams/<name>.excalidraw` — [what it shows]
```

### Spec Writing Rules

- **Show real Rust types** in the Key Types section — this is the most valuable part of the spec for the implementer.
- **Reference diagrams** rather than re-describing them in text. The diagrams are first-class artifacts.
- **Be concrete about module paths** — `src/systems/asteroid.rs` not "the asteroid system file."
- **State what's TBD** explicitly. Ambiguity hidden in vague prose is worse than acknowledged uncertainty.
- **Keep the spec honest** — only write what was actually decided in the Q&A. Don't invent answers.

---

## Invocation Summary

When `/refine` is invoked:

1. Check bootstrap condition (no `Cargo.toml` → first-run mode)
2. Read codebase (`Cargo.toml`, `src/`, `specs/`)
3. Read `ROADMAP.md` if it exists
4. State your understanding in 2-4 sentences
5. Draw an initial diagram with `excalidraw-diagram`
6. Ask question #1 (Tier 1, most important unknown)
7. Loop: answer → update diagram if needed → next question
8. When ready: announce readiness, get confirmation or "write it"
9. Write `specs/<name>.md` and save all diagrams to `specs/diagrams/`
10. Update `ROADMAP.md` — add the new feature if not already present, or mark it as `In progress`
11. Tell the user the spec is written and what skills come next (`/plan`, `/execute`, `/review`, `/docs`)

---

## Tone and Style

- **Collaborative, not interrogative.** You're a co-designer, not an interviewer.
- **Be opinionated about Rust.** If the user proposes something that fights the borrow checker or violates common Rust patterns, say so and suggest the idiomatic alternative.
- **Use the diagrams to build shared understanding.** When something is hard to explain in words, reach for a diagram first.
- **Short turns.** Each turn: one short statement or summary + one question (or the completed spec). No long monologues mid-conversation.
