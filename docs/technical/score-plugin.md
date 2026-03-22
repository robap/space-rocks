# ScorePlugin — Technical Reference

**Source:** `src/plugins/score.rs`
**Spec:** `specs/score-system.md`
**Review:** `reviews/score-system.md`
**Last updated:** 2026-03-22

---

## Overview

`ScorePlugin` awards points when asteroids are destroyed by bullets. It owns the `AsteroidDestroyedEvent` registration, reads those events each frame, and updates the shared `Score` resource. No other system in the codebase touches event production or score accumulation — this plugin is the single owner of that pipeline.

---

## Key Types

```rust
// src/components.rs — pre-existing shared types consumed by this plugin
pub struct Score(pub u32);            // resource: current player score

pub struct AsteroidDestroyedEvent {   // event fired by CollisionPlugin
    pub size: AsteroidSize,
}
```

```rust
// src/config.rs — point values per asteroid size
pub const SCORE_LARGE: u32  = 20;
pub const SCORE_MEDIUM: u32 = 50;
pub const SCORE_SMALL: u32  = 100;
```

`ScorePlugin` introduces no new component or resource types. `Score` is initialised by `GameStatePlugin` at `Startup` and reset to `0` by `GameStatePlugin` on new game — `ScorePlugin` only reads and increments it.

---

## Architecture

```
src/plugins/
  score.rs   — ScorePlugin, points_for_size(), on_asteroid_destroyed()
```

The plugin is intentionally minimal: one public function, one private system, one `build` method.

### `points_for_size`

```rust
pub fn points_for_size(size: AsteroidSize) -> u32 {
    match size {
        AsteroidSize::Large  => SCORE_LARGE,
        AsteroidSize::Medium => SCORE_MEDIUM,
        AsteroidSize::Small  => SCORE_SMALL,
    }
}
```

Pure function with no Bevy dependencies. Exists separately from the system so it can be unit-tested without a full `App`. The `pub` visibility is intentional — the spec calls it out as the testable interface.

### `on_asteroid_destroyed`

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

Private to the module. Drains the event reader every frame; multiple events in the same frame are each scored independently.

---

## Data Flow

See: `specs/diagrams/score-system.excalidraw`

Sequence for a typical asteroid destruction:

1. `CollisionPlugin::bullet_asteroid_collision` (in `GameSet::Collision`) detects a bullet↔asteroid overlap
2. It writes `AsteroidDestroyedEvent { size }` via `EventWriter`
3. After `GameSet::Collision` completes, `on_asteroid_destroyed` runs (`.after(GameSet::Collision)`)
4. It reads the event, calls `points_for_size(event.size)`, and adds the result to `score.0`
5. `HudPlugin::update_hud` runs in the same `Update` schedule and reads `Res<Score>` to refresh the HUD text

---

## Design Decisions

- **`after(GameSet::Collision)` not `in_set(GameSet::Collision)`** — the scoring system reads events *produced* by the collision set. Running inside the set would create an ordering ambiguity with the system that writes the events. Running after guarantees all collision events are written before the score is updated.

- **`ScorePlugin` owns `add_event::<AsteroidDestroyedEvent>()`** — `CollisionPlugin` fires the event but was written before any consumer existed. Bevy requires events to be registered before they can be written or read. `ScorePlugin` is the first consumer, so it takes responsibility for registration. If `ScorePlugin` is ever removed, whoever needs the event next must pick up `add_event`.

- **`saturating_add` over `+=`** — `Score` is a `u32`. A theoretical overflow at `u32::MAX` (4,294,967,295 points) would panic in debug or wrap silently in release. `saturating_add` clamps harmlessly. This is a deliberate safety choice, not an oversight.

- **`points_for_size` is a separate function** — pulling the `match` out of the system body makes it independently unit-testable without constructing a Bevy `App` or mocking an `EventReader`. The system itself has no meaningful pure-function test surface.

- **`.run_if(in_state(GameState::Playing))`** — `AsteroidDestroyedEvent` is only ever sent from `bullet_asteroid_collision`, which runs unconditionally. The guard is defensive: if a future system sends the event outside `Playing`, the score system ignores it rather than silently accumulating points.

---

## Integration Points

| System | Plugin | Relationship |
|--------|--------|-------------|
| `bullet_asteroid_collision` | `CollisionPlugin` | Writes `AsteroidDestroyedEvent` that this plugin consumes |
| `update_hud` | `HudPlugin` | Reads `Score` resource each frame to update on-screen text |
| `insert_game_resources` | `GameStatePlugin` | Inserts `Score(0)` at `Startup` |
| `handle_attract_input` / `handle_game_over_input` | `GameStatePlugin` | Resets `score.0 = 0` when a new game starts |

---

## Known Constraints and Gotchas

- **`AsteroidDestroyedEvent` registration lives here.** If `ScorePlugin` is ever removed from the app, `CollisionPlugin` will panic at runtime on the first asteroid destruction because the event is no longer registered. The ownership of `add_event::<AsteroidDestroyedEvent>()` must move to `CollisionPlugin` (or another plugin) if `ScorePlugin` is removed.

- **Score resource is not initialised by `ScorePlugin`.** `GameStatePlugin::insert_game_resources` inserts it at `Startup`. If `GameStatePlugin` is removed or the insertion is missed, systems that access `ResMut<Score>` will panic.

- **No test for the system itself.** `on_asteroid_destroyed` is not unit-tested because it requires a Bevy `App` with registered event and resource state. The pure logic (`points_for_size`) is tested; the system wiring is validated by playtest.
