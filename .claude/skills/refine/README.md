# Refine Skill

An interactive specification-refinement skill for this Rust project. Turns rough ideas into structured, actionable specs through iterative Q&A and collaborative Excalidraw diagrams.

## Usage

```
/refine
```

Then describe your idea — as vague or incomplete as you like. Claude will read the codebase, draw an initial diagram of its understanding, and ask one question at a time to sharpen the spec.

## Output

- `specs/<feature-name>.md` — the specification
- `specs/diagrams/*.excalidraw` — all diagrams produced during refinement

## Dependencies

- `excalidraw-diagram` skill (already installed) — used heavily during refinement

## Workflow Position

```
/refine  →  /plan  →  /execute  →  /review
```

Refine produces specs. Plan turns specs into implementation plans. Execute implements them. Review validates the result.
