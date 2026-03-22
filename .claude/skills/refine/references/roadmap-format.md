# ROADMAP.md Format Reference

The `ROADMAP.md` file lives at the project root. It is the authoritative source of what needs to be built, in what order, and what has been completed.

---

## Format

```markdown
# Roadmap: [Game Name]

> [One sentence describing the game and its goal]

---

## Milestone 1: [Name — e.g. "Playable Foundation"]

> [What this milestone delivers — what the game can do when this milestone is done]

- [x] [Feature name] — `specs/feature-name.md`
- [ ] [Feature name] — `specs/feature-name.md`
- [ ] [Feature name] — *(not yet specced)*

---

## Milestone 2: [Name — e.g. "Core Gameplay Loop"]

> [What this milestone delivers]

- [ ] [Feature name] — *(not yet specced)*
- [ ] [Feature name] — *(not yet specced)*

---

## Backlog

> Features identified but not yet assigned to a milestone.

- [ ] [Feature name] — [brief description]
- [ ] [Feature name] — [brief description]

---

## Completed

> Moved here after `/review` passes and `[x]` is marked in the milestone above.

- [x] [Feature name] — `reviews/feature-name.md`
```

---

## Rules

**Status markers:**
- `[ ]` — not started (no spec yet, or spec written but not planned)
- `[~]` — in progress (spec written, plan exists, execute underway)
- `[x]` — complete (review passed)

**Spec links:** Once a spec is written by `/refine`, update the roadmap entry to link it: `— \`specs/feature-name.md\``

**Milestone ordering matters.** Items in Milestone 1 must be completable without any item in Milestone 2. If a dependency is discovered during `/refine` or `/plan`, reorder the milestones.

**The Backlog** holds features that are known but not yet prioritized or milestoned. `/refine` adds unplanned ideas here by default.

**Completed section** is a record, not a list to act on. It exists so the history of what was built is visible at a glance.

---

## Who updates ROADMAP.md

| Skill | Action |
|-------|--------|
| `/refine` | Adds new features; marks in-progress items `[~]`; creates ROADMAP.md on first run |
| `/review` | Marks completed items `[x]`; moves them to Completed section |
| User | Prioritizes backlog; assigns items to milestones; adds milestone descriptions |
