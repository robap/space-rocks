# Level Progression

**Status:** Draft
**Created:** 2026-03-22
**Spec author:** Refined via /refine skill

---

## Summary

When all asteroids in a level are destroyed, the game pauses briefly, increments the level counter, and spawns a new, larger wave. The level number is displayed in the HUD and shown in a "Level X — Get Ready" overlay during the transition.

---

## Motivation

Currently clearing all asteroids leaves an empty screen with no progression. This feature makes clearing a level feel rewarding and creates escalating challenge through increasing asteroid counts.

---

## Scope

### In scope
- Detect when all asteroids are destroyed while a level is active
- Increment level counter on clear
- 5-second transition pause with "Level X — Get Ready" overlay text
- Spawn new asteroids when the transition ends
- Scale asteroid count: `ASTEROID_INITIAL_COUNT + (level - 1) * ASTEROID_COUNT_INCREMENT`, capped at `ASTEROID_MAX_COUNT`
- Display current level number in HUD

### Out of scope
- Asteroid speed scaling with level
- Extra lives at point thresholds
- Audio for level transition

---

## Architecture

### Where it lives

New plugin: `src/plugins/level.rs`, registered in `src/plugins/mod.rs`.

`AsteroidPlugin` gains a new event handler for `SpawnLevelEvent` and its reset handler is split (despawn-only on `ResetGameEvent`).
`HudPlugin` gains a level text element and a "Level X — Get Ready" overlay.
`GameState` gains a new `LevelTransition` variant in `src/components.rs`.

### Key types

```rust
// src/components.rs additions

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Attract,
    Playing,
    Dead,
    LevelTransition,  // new: between level clear and next level spawn
    GameOver,
}

#[derive(Resource)]
pub struct Level {
    pub number: u32,   // starts at 1; incremented on each clear
    pub active: bool,  // true once asteroids have been spawned for this level
}

#[derive(Event)]
pub struct SpawnLevelEvent {
    pub count: usize,
}

#[derive(Resource)]
pub struct LevelTransitionTimer(pub Timer);

#[derive(Component)]
pub struct HudLevelText;

#[derive(Component)]
pub struct LevelReadyText;
```

```rust
// src/config.rs additions
pub const ASTEROID_COUNT_INCREMENT: usize = 2;
pub const ASTEROID_MAX_COUNT: usize = 12;
pub const LEVEL_TRANSITION_SECS: f32 = 5.0;
```

### Systems / ECS integration

**`LevelPlugin` systems:**

| System | Schedule | Condition |
|--------|----------|-----------|
| `insert_level_resource` | Startup | — |
| `detect_level_clear` | Update | `run_if(in_state(Playing))` |
| `start_level_transition` | OnEnter(LevelTransition) | — |
| `tick_level_transition` | Update | `run_if(in_state(LevelTransition))` |
| `on_reset_game` | Update | — |

**`AsteroidPlugin` changes:**
- `handle_asteroid_reset` becomes despawn-only on `ResetGameEvent`
- New `handle_spawn_level` system reads `SpawnLevelEvent`, spawns asteroids, sets `level.active = true`

**`HudPlugin` changes:**
- Add level number text (top-center)
- Add "Level X — Get Ready" overlay (center-screen)
- Show/hide overlay via `OnEnter(LevelTransition)` / `OnExit(LevelTransition)`
- `update_hud` reads `Res<Level>` and updates the level text

### Module structure

```
src/plugins/
  level.rs    ← new
  asteroid.rs ← modified (split reset handler, add handle_spawn_level)
  hud.rs      ← modified (level text + ready overlay)
  mod.rs      ← register LevelPlugin
src/components.rs ← add Level, SpawnLevelEvent, LevelTransitionTimer,
                     HudLevelText, LevelReadyText, GameState::LevelTransition
src/config.rs ← add ASTEROID_COUNT_INCREMENT, ASTEROID_MAX_COUNT, LEVEL_TRANSITION_SECS
```

---

## Behavior

### Core loop

1. Player destroys the last asteroid.
2. `detect_level_clear` sees `asteroids.is_empty() && level.active` → sets `level.active = false`, increments `level.number`, transitions to `GameState::LevelTransition`.
3. `OnEnter(LevelTransition)`: inserts `LevelTransitionTimer(5s)`, updates "Level X — Get Ready" text content, shows overlay.
4. `tick_level_transition` ticks the timer each frame. When finished: removes the timer, fires `SpawnLevelEvent { count }`, transitions to `GameState::Playing`.
5. `AsteroidPlugin::handle_spawn_level` reads `SpawnLevelEvent`, spawns `count` asteroids, sets `level.active = true`.
6. `OnExit(LevelTransition)`: hides the overlay.

### Asteroid count formula

```rust
// lives in src/plugins/level.rs, unit tested
fn asteroid_count_for_level(level: u32) -> usize {
    let count = ASTEROID_INITIAL_COUNT + (level as usize - 1) * ASTEROID_COUNT_INCREMENT;
    count.min(ASTEROID_MAX_COUNT)
}
```

| Level | Count |
|-------|-------|
| 1 | 6 |
| 2 | 8 |
| 3 | 10 |
| 4+ | 12 (capped) |

### Game reset flow

On `ResetGameEvent` (new game from Attract or GameOver):
1. `AsteroidPlugin::handle_asteroid_reset` despawns all existing asteroids.
2. `LevelPlugin::on_reset_game` resets `Level { number: 1, active: false }`, removes any existing `LevelTransitionTimer`, fires `SpawnLevelEvent { count: ASTEROID_INITIAL_COUNT }`.
3. `AsteroidPlugin::handle_spawn_level` spawns 6 asteroids, sets `level.active = true`.

`on_reset_game` bypasses `LevelTransition` state entirely so the new game starts immediately without a delay.

### State transitions

```
Attract ──(any key)──► Playing ──(level clear)──► LevelTransition ──(5s)──► Playing
                          │                                                      ▲
                          ├──(ship destroyed, lives > 0)──► Dead ──(1.5s)───────┘
                          └──(ship destroyed, lives = 0)──► GameOver ──(any key)──► Playing
```

### Input/Output

| Input | Type | Source |
|-------|------|--------|
| Asteroid entities | `Query<Entity, With<Asteroid>>` | ECS |
| Level state | `Res<Level>` | LevelPlugin resource |
| Reset event | `EventReader<ResetGameEvent>` | GameStatePlugin |
| Spawn event | `EventReader<SpawnLevelEvent>` | LevelPlugin |

| Output | Type | Destination |
|--------|------|-------------|
| State transition | `NextState<GameState>` | Bevy state machine |
| Spawn event | `EventWriter<SpawnLevelEvent>` | AsteroidPlugin |
| Level mutations | `ResMut<Level>` | LevelPlugin / AsteroidPlugin |

### Error handling

- `detect_level_clear` runs every frame in `Playing`; the `active` flag prevents false triggers before first spawn and during the one-frame gap between `SpawnLevelEvent` firing and `AsteroidPlugin` processing it.
- If `LevelTransitionTimer` is missing during `tick_level_transition` (invariant violation), use `warn!` and return early — do not panic.
- `on_reset_game` must handle being called from any state including `LevelTransition`. It removes `LevelTransitionTimer` if present before transitioning.

---

## Edge Cases

- **Level clear during `Dead` state:** `detect_level_clear` only runs in `Playing`, so the last asteroid being destroyed mid-respawn delay cannot trigger a false level clear.
- **Game reset during `LevelTransition`:** `on_reset_game` runs regardless of state. It removes `LevelTransitionTimer` and fires `SpawnLevelEvent` directly, bypassing the overlay.
- **`active` flag and one-frame gap:** `SpawnLevelEvent` is fired and processed by `AsteroidPlugin` possibly one frame later. `active` remains `false` during this gap, so `detect_level_clear` cannot trigger.

---

## Performance Considerations

No specific constraints identified. `detect_level_clear` is a simple empty-query check running once per frame in `Playing` state only.

---

## Testing Strategy

Unit test the pure count formula (no Bevy app required):

```rust
#[test]
fn asteroid_count_level_1_is_initial() {
    assert_eq!(asteroid_count_for_level(1), ASTEROID_INITIAL_COUNT);
}

#[test]
fn asteroid_count_increments_by_two_per_level() {
    assert_eq!(asteroid_count_for_level(2), ASTEROID_INITIAL_COUNT + 2);
    assert_eq!(asteroid_count_for_level(3), ASTEROID_INITIAL_COUNT + 4);
}

#[test]
fn asteroid_count_caps_at_max() {
    assert_eq!(asteroid_count_for_level(100), ASTEROID_MAX_COUNT);
}
```

Integration testing: manual play.

---

## Open Questions

None — all structural and behavioral questions resolved.

---

## Diagrams

- `specs/diagrams/wave-spawn-flow.excalidraw` — initial flow diagram sketched at start of refinement
