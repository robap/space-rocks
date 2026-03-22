# Review Skill

Validates a completed plan implementation against its spec. Fixes what it can. Reports what needs human judgment. Produces `reviews/<name>.md`.

## Usage

```
/review plans/<feature-name>.md
```

## What it checks

1. Plan completeness — did Execute cover every task?
2. Scope — did Execute write more than asked?
3. Spec satisfaction — does the implementation meet the objectives?
4. Security — unsafe blocks, panics in hot paths, unbounded collections, dependency advisories
5. Clean code — cross-feature naming, function size, duplication, visibility
6. Test quality — coverage gaps, behavior vs. implementation coupling, naming
7. Macro-level structural refactor — module boundaries, shared abstractions, layer violations
8. API surface — is public visibility minimal and intentional?
9. Spec drift — update the spec to match what was actually built
10. Dependency audit — new crates justified and clean?

## Output

- `reviews/<name>.md` — structured findings (Blocker / Warning / Note) + record of all fixes applied
- Fixes applied directly: refactoring, clean code, test gaps, formatting
- Blockers escalated: unimplemented spec requirements, security decisions, ambiguous spec

## Workflow Position

```
/refine  →  /plan  →  /execute  →  /review
```

Review is the final gate. Plan status is updated to `Complete ✓` only after all Blockers are resolved.
