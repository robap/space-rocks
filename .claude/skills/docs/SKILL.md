---
name: docs
description: Writes and maintains two kinds of documentation after a feature is reviewed: technical docs (docs/technical/) for developers and agents, and user docs (docs/user/) for players.
---

# Docs — Documentation Skill

Produces and maintains documentation in `docs/` after a feature passes `/review`. Two audiences, two doc types, one skill.

---

## Invocation

```
/docs plans/<feature-name>.md
```

If invoked without a path, lists plans with status `Complete ✓` that are missing documentation entries, and asks which to document.

---

## Two Audiences, Two Doc Types

### Technical Documentation (`docs/technical/`)

**Audience:** You (the developer) and future Claude sessions acting as agents on this codebase.

**Purpose:** Explain how a feature works from the inside — its architecture, types, systems, data flow, and integration points. A future agent reading this doc should be able to understand the feature well enough to modify it, extend it, or debug it without reading all the source code.

**A good technical doc answers:**
- What does this feature do, and where does it live in the codebase?
- What are the key types and what do they represent?
- How does data flow through the system? (Reference diagrams)
- What are the non-obvious design decisions, and why were they made?
- What are the known constraints, invariants, or gotchas?
- How does this feature interact with other systems?

### User Documentation (`docs/user/`)

**Audience:** A person playing the game. No knowledge of Rust, game engines, or the codebase.

**Purpose:** Explain what the player can do, how to do it, and what to expect. User docs describe the game experience, not the implementation.

**A good user doc answers:**
- What is this feature from the player's perspective?
- How do I interact with it? (Controls, inputs, actions)
- What happens as a result?
- Are there rules, limits, or edge cases the player should know about?

---

## Phase 0: Read Everything

Before writing a word:

1. **Read the spec** — the spec defines intended behavior and is the source of truth for what the feature was supposed to do.
2. **Read the review** (`reviews/<name>.md`) — the review records what was actually built, any deviations from spec, and any playtest observations. The docs must reflect reality, not just the spec.
3. **Read the implementation** — scan the relevant source files to understand the actual types, function signatures, and module structure. Technical docs cite real code.
4. **Read existing docs** — check `docs/technical/` and `docs/user/` for related entries. This feature may extend something already documented. Update existing docs rather than duplicating.
5. **Check for diagrams** — the spec may reference `specs/diagrams/`. Use these in technical docs rather than redrawing.

---

## Phase 1: Write Technical Documentation

Save to `docs/technical/<feature-name>.md`.

If the feature extends an already-documented system, edit the existing file rather than creating a new one. Append a clearly labeled section.

### Technical Doc Template

```markdown
# [Feature Name] — Technical Reference

**Source:** `src/[module path]`
**Spec:** `specs/<feature-name>.md`
**Review:** `reviews/<feature-name>.md`
**Last updated:** [date]

---

## Overview

[2-4 sentences. What this feature does and why it exists in the codebase. Name the primary module(s).]

---

## Key Types

[Document the primary types introduced by this feature. Show the actual struct/enum definition with field-level comments where non-obvious. Do not copy-paste the entire source — only the types a reader needs to understand the feature.]

```rust
pub struct AsteroidField {
    /// Active asteroids currently in the game world.
    pub asteroids: Vec<Asteroid>,
    /// Seconds until next asteroid spawns.
    spawn_timer: f32,
}

pub enum AsteroidSize {
    Large,   // Splits into 2 Medium on death
    Medium,  // Splits into 2 Small on death
    Small,   // Despawned on death, no split
}
```

---

## Architecture

[How is this feature structured? Which module owns what? If the feature spans multiple files, explain the division of responsibility.]

### Module layout
```
src/
  systems/
    asteroid_spawner.rs   — spawns asteroids on a timer
    asteroid_physics.rs   — applies velocity, handles wrapping
    asteroid_death.rs     — split logic and despawn
  components/
    asteroid.rs           — Asteroid, AsteroidSize types
```

---

## Data Flow

[How does data move through this feature during a typical frame? Use a diagram if one exists, or describe it as a sequence.]

See: `specs/diagrams/<name>.excalidraw`

[Or describe in prose/steps if no diagram exists:]

1. `SpawnSystem` fires when `AsteroidField.spawn_timer` reaches 0 → creates `Asteroid` entity with random position on screen edge
2. `PhysicsSystem` applies `Asteroid.velocity` to `Transform` each frame
3. `CollisionSystem` raises `AsteroidHitEvent` when asteroid overlaps player or bullet
4. `DeathSystem` receives `AsteroidHitEvent` → splits or despawns based on `AsteroidSize`

---

## Design Decisions

[Document non-obvious choices and the reasons behind them. These are the most valuable part of the technical doc — they prevent future work from unknowingly reversing good decisions.]

- **Why `Vec<Asteroid>` instead of ECS entities?** At the scale of this game, a plain `Vec` is simpler and faster than querying ECS entities. If asteroid count ever exceeds ~200 or they need complex component composition, revisit.
- **Why split on the death system and not the collision system?** Separation of concerns — collision detects the event, death decides the consequence. Keeps both systems focused.

---

## Integration Points

[What other systems or features does this one depend on or interact with? Explicit is better than implicit.]

| System | Relationship |
|--------|-------------|
| `CollisionSystem` | Raises `AsteroidHitEvent` consumed by `DeathSystem` |
| `RenderSystem` | Reads `Transform` and `AsteroidSize` to pick sprite |
| `ScoreSystem` | Listens for `AsteroidHitEvent` to increment score |

---

## Known Constraints and Gotchas

[Anything that will surprise or trap a future developer or agent working on this feature.]

- `MAX_ASTEROID_COUNT` (`src/constants.rs`) is a hard cap enforced in `SpawnSystem`. If you raise it, profile for frame drops.
- Split children inherit parent velocity plus a perpendicular offset — see `calculate_split_velocity()` in `asteroid_death.rs` if changing split behavior.
- Wrapping behavior assumes screen dimensions are read from `ScreenBounds` resource. If screen is resized at runtime, `ScreenBounds` must be updated first.
```

---

## Phase 2: Write or Update User Documentation

User docs are maintained as **one document per major game system**, not one per feature. The player doesn't think in terms of features — they think in terms of the game.

Check `docs/user/` for an existing document this feature belongs to. Common groupings for a space game:
- `docs/user/how-to-play.md` — controls, objectives, win/lose conditions
- `docs/user/enemies-and-hazards.md` — asteroids, enemies, what they do
- `docs/user/weapons-and-powerups.md` — shooting, special abilities
- `docs/user/scoring.md` — how points work

If no appropriate document exists, create one. If one exists, add a section or expand an existing section.

### User Doc Writing Rules

- **No technical terms.** No structs, systems, modules, or Rust. Write as if explaining the game to a friend.
- **Present tense, active voice.** "Press Space to fire. Your ship shoots a bullet in the direction you're facing."
- **Describe what the player sees and does**, not how the code implements it.
- **Controls are explicit.** Don't say "use the shoot button" — say "press Space."
- **Keep it short.** Players skim. Bullets and short paragraphs over walls of text.

### User Doc Template (for a new section)

```markdown
## [Feature Name in Plain English]

[1-2 sentence description of what this is from the player's point of view.]

### How it works

[What the player sees, in the order they see it. Use plain language.]

Asteroids float across the screen in random directions. Large asteroids are slow and easy to see. Small asteroids are fast and harder to dodge.

### Controls

| Action | Key / Input |
|--------|------------|
| [action] | [key] |

### Tips

- [Player-useful tip]
- [Player-useful tip]
```

---

## Phase 3: Draw a Diagram (When Helpful)

For technical docs, if the data flow or system interaction is complex enough that prose is harder to follow than a picture, use the `excalidraw-diagram` skill to create a diagram and save it to `docs/diagrams/<feature-name>.excalidraw`. Reference it in the technical doc.

Do not create a diagram just to have one. Draw it only if it communicates something that text alone cannot.

---

## Phase 4: Update the Docs Index

Maintain `docs/INDEX.md` — a single file listing all documentation with one-line descriptions. This is what agents consult first when they need to find relevant docs quickly.

```markdown
# Documentation Index

## Technical
- [Feature Name](technical/feature-name.md) — [one line: what it covers]

## User
- [How to Play](user/how-to-play.md) — controls, objectives, basic rules
- [Enemies and Hazards](user/enemies-and-hazards.md) — asteroids and what they do
```

Add the new or updated doc to the appropriate section. Keep descriptions to one line.

---

## Quality Check Before Finishing

**Technical doc:**
- [ ] All key types documented with real definitions
- [ ] Data flow is traceable from entry to exit
- [ ] Design decisions record the *why*, not just the *what*
- [ ] Integration points named explicitly
- [ ] Gotchas documented — anything that surprised you during implementation

**User doc:**
- [ ] No technical jargon
- [ ] Controls are specific (exact key names)
- [ ] Player can learn the feature from this doc without asking further questions
- [ ] Written in present tense, active voice

**Both:**
- [ ] `docs/INDEX.md` updated
- [ ] Existing docs amended rather than duplicated where possible
- [ ] Diagrams referenced, not re-described in prose
