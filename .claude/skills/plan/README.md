# Plan Skill

Reads a spec file and produces a structured, executable plan in `plans/` with hierarchical checkbox tasks.

## Usage

```
/plan specs/<feature-name>.md
```

If invoked without an argument, lists available specs and asks which to plan.

## Output

- `plans/<feature-name>.md` — the task breakdown, ready for `/execute`

## Workflow Position

```
/refine  →  /plan  →  /execute  →  /review
```

Plan consumes specs produced by `/refine`. The resulting plan file is consumed by `/execute`.
