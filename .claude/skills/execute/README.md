# Execute Skill

Works through a plan file task by task using strict TDD. Checks off `[ ]` boxes as it goes.

## Usage

```
/execute plans/<feature-name>.md
```

If invoked without an argument, lists available plans and asks which to execute.

## TDD Loop (per task)

1. Write test → 2. Write stub (compiles, wrong behavior) → 3. Run test (fail for behavioral reason) → 4. Implement → 5. Run test (pass) → 6. Run full suite → 7. Clippy → 8. Check off task

## Test Location

All tests live in `tests/` — never inline `#[cfg(test)]` modules in source files.

## Workflow Position

```
/refine  →  /plan  →  /execute  →  /review
```

Execute consumes plans produced by `/plan`. When all tasks are checked off, run `/review` to validate against the spec.
