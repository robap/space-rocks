# Docs Skill

Writes and maintains two kinds of documentation after a feature passes `/review`.

## Usage

```
/docs plans/<feature-name>.md
```

## Output

- `docs/technical/<feature-name>.md` — developer/agent reference: types, architecture, data flow, design decisions, gotchas
- `docs/user/<system>.md` — player-facing: controls, mechanics, tips. One doc per game system, not per feature.
- `docs/INDEX.md` — kept up to date as a fast-lookup index for both agents and humans

## Workflow Position

```
/refine  →  /plan  →  /execute  →  /review  →  /docs
```

Docs is the final step. A feature is not fully shipped until both technical and user docs exist.
