# Score System

**Status:** Draft
**Created:** 2026-03-22
**Spec author:** Refined via /refine skill

---

## Summary

Award the player points when asteroids are destroyed by bullets. Smaller asteroids are worth more points because they are harder to hit. The score is displayed in the top-right HUD and resets when a new game begins.

---

## Motivation

Shooting asteroids currently has no reward signal. Adding a score gives players a goal to optimise for and makes the game feel complete. Almost all of the required infrastructure already exists — this feature wires up the one missing connection.

---

## Scope

### In scope
- Point constants for each `AsteroidSize` in `config.rs`
- A `ScorePlugin` with a single system that reads `AsteroidDestroyedEvent` and increments `Score`
- Unit tests for the points-per-size logic

### Out of scope
- High score persistence (separate roadmap item)
- Extra lives at score thresholds (separate roadmap item)
- Showing score on the game-over screen
- Points for any source other than bullet hits (no ship ram scoring, no enemy ships yet)

---

## Architecture

### Where it lives

New file: `src/plugins/score.rs`
Registered in: `src/plugins/mod.rs` (alongside the other plugins)
Constants added to: `src/config.rs`

### Key types

```rust
// src/config.rs — add these constants
pub const SCORE_LARGE: u32 = 20;
pub const SCORE_MEDIUM: u32 = 50;
pub const SCORE_SMALL: u32 = 100;
```

```rust
// src/plugins/score.rs
pub struct ScorePlugin;

// Pure helper — easy to unit test without a Bevy app
pub fn points_for_size(size: AsteroidSize) -> u32 {
    match size {
        AsteroidSize::Large  => SCORE_LARGE,
        AsteroidSize::Medium => SCORE_MEDIUM,
        AsteroidSize::Small  => SCORE_SMALL,
    }
}
```

The `Score(pub u32)` resource and `AsteroidDestroyedEvent` are already defined in `src/components.rs`. No new types are needed.

### Systems / ECS integration

| System | Plugin | Reads | Writes | Ordering |
|--------|--------|-------|--------|----------|
| `on_asteroid_destroyed` | `ScorePlugin` | `EventReader<AsteroidDestroyedEvent>` | `ResMut<Score>` | After `GameSet::Collision` (events fired there) |

The system runs only in `GameState::Playing` (there are no asteroid destructions outside that state, but the guard keeps it defensive).

### Module structure

```
src/plugins/
  score.rs      ← new
  collision.rs  ← unchanged (already fires AsteroidDestroyedEvent)
  hud.rs        ← unchanged (already reads Score and displays it)
  game_state.rs ← unchanged (already resets Score to 0 on game start)
```

---

## Behavior

### Core loop

Every frame, `on_asteroid_destroyed` drains `EventReader<AsteroidDestroyedEvent>`. For each event it calls `points_for_size(event.size)` and adds the result to `score.0` via `saturating_add` (prevents overflow if someone plays for a very long time).

```rust
fn on_asteroid_destroyed(
    mut events: EventReader<AsteroidDestroyedEvent>,
    mut score: ResMut<Score>,
) {
    for event in events.read() {
        score.0 = score.0.saturating_add(points_for_size(event.size));
    }
}
```

### Input / Output

| Input | Type | Source |
|-------|------|--------|
| Asteroid size at time of destruction | `AsteroidDestroyedEvent { size: AsteroidSize }` | `CollisionPlugin::bullet_asteroid_collision` |

| Output | Type | Destination |
|--------|------|-------------|
| Updated score | `Score(pub u32)` mutated in place | `HudPlugin::update_hud` reads it each frame |

### Error handling

No failure modes. Reading events is infallible; `saturating_add` prevents overflow. No `Result` types needed.

---

## Edge Cases

- **Multiple collisions in one frame**: `EventReader::read()` iterates all events — each yields its own `points_for_size` call, so two simultaneous hits score independently and correctly.
- **Score overflow**: `saturating_add` clamps at `u32::MAX` rather than wrapping. In practice unreachable, but handled defensively.
- **Events fired outside Playing state**: `AsteroidDestroyedEvent` is only ever sent from `bullet_asteroid_collision`, which runs unconditionally. The system should be gated with `.run_if(in_state(GameState::Playing))` to be safe.

---

## Performance Considerations

No specific constraints. The event reader iterates at most a handful of events per frame.

---

## Testing Strategy

Unit test the pure `points_for_size` function — no Bevy app required.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_asteroid_scores_20() {
        assert_eq!(points_for_size(AsteroidSize::Large), 20);
    }

    #[test]
    fn medium_asteroid_scores_50() {
        assert_eq!(points_for_size(AsteroidSize::Medium), 50);
    }

    #[test]
    fn small_asteroid_scores_100() {
        assert_eq!(points_for_size(AsteroidSize::Small), 100);
    }
}
```

---

## Open Questions

None. All decisions resolved.

---

## Diagrams

- `specs/diagrams/score-system.excalidraw` — full data-flow from collision system through event, scoring system, Score resource, and HUD update
