# Lives and Game Over

**Status:** Draft
**Created:** 2026-03-22
**Spec author:** Refined via /refine skill

---

## Summary

Add ship-asteroid collision, a three-life system, and a full game state machine (Attract → Playing → Dead → Game Over). The game launches into an attract screen with moving asteroids and no ship; pressing any key starts a fresh game. Losing all lives shows a Game Over overlay; pressing any key resets to a fresh game again. A HUD displays current lives and a score placeholder (always 0 until the score spec is implemented).

---

## Motivation

Currently nothing happens when the ship touches an asteroid. This feature closes that gap and adds the core game loop structure that all future features (score, new waves, menus) will build on.

---

## Scope

### In scope
- Ship-asteroid circle collision detection
- `GameState` enum with four variants: `Attract`, `Playing`, `Dead`, `GameOver`
- Lives resource — starts at 3, decrements on ship death
- Respawn delay (1.5 s) before the ship reappears after death
- Invincibility window (2.0 s) on respawn — ship blinks to signal it
- Attract screen: asteroids moving, no ship, "Press Any Key to Start" text
- Game Over screen: asteroids still moving, ship gone, "Game Over" + "Press Any Key to Start" text
- Full game reset on any-key press from Attract or Game Over (fresh asteroids, fresh ship, lives reset to 3)
- HUD: lives count and score placeholder (always shows 0)

### Out of scope
- Actual score tracking (future spec)
- New wave spawning when all asteroids are cleared (future spec)
- Ship explosion animation
- Sound effects
- High score

---

## Architecture

### Where it lives

| Concern | File |
|---|---|
| `GameState` enum, new components/resources | `src/components.rs` |
| New config constants | `src/config.rs` |
| State transitions, respawn timer, reset logic | `src/plugins/game_state.rs` (new) |
| HUD text entities and update systems | `src/plugins/hud.rs` (new) |
| Ship-asteroid collision | `src/plugins/collision.rs` (extended) |
| Ship spawning triggered by events | `src/plugins/ship.rs` (modified) |
| Asteroid reset triggered by events | `src/plugins/asteroid.rs` (modified) |
| Plugin registration | `src/main.rs` (modified), `src/plugins/mod.rs` (modified) |

### Key types

```rust
// --- components.rs additions ---

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Attract,
    Playing,
    Dead,
    GameOver,
}

/// Remaining lives. Decremented on ship death; game over when it reaches 0.
#[derive(Resource)]
pub struct Lives(pub u32);

/// Score placeholder — always 0 until the score spec is implemented.
#[derive(Resource)]
pub struct Score(pub u32);

/// Attached to the ship entity during the invincibility window after respawn.
/// Collision systems skip entities that carry this component.
#[derive(Component)]
pub struct Invincible {
    pub timer: Timer,        // total invincibility duration
    pub blink_timer: Timer,  // toggles visibility every SHIP_BLINK_INTERVAL_SECS (updated post-implementation)
}

/// Resource present only while in the Dead state. Counts down before respawn.
#[derive(Resource)]
pub struct RespawnTimer(pub Timer);

// Events
#[derive(Event)]
pub struct ShipDestroyedEvent;

#[derive(Event)]
pub struct ResetGameEvent;

/// Fired by GameStatePlugin (respawn path) and ShipPlugin (reset path) to centralise
/// all ship spawning in ShipPlugin. (added post-implementation)
#[derive(Event)]
pub struct SpawnShipEvent {
    pub invincible: bool,
}

// HUD marker components
#[derive(Component)]
pub struct HudLivesText;

#[derive(Component)]
pub struct HudScoreText;

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct PressAnyKeyText;
```

```rust
// --- config.rs additions ---

pub const SHIP_RADIUS: f32 = 16.0;
pub const PLAYER_STARTING_LIVES: u32 = 3;
pub const SHIP_RESPAWN_DELAY_SECS: f32 = 1.5;
pub const SHIP_INVINCIBILITY_SECS: f32 = 2.0;
pub const SHIP_BLINK_INTERVAL_SECS: f32 = 0.1;
```

### Systems / ECS integration

#### `CollisionPlugin` (extended)

| System | Set | Runs when |
|---|---|---|
| `bullet_asteroid_collision` | `Collision` | existing, unchanged |
| `ship_asteroid_collision` | `Collision` | `state == Playing`, ship has no `Invincible` |

`ship_asteroid_collision` detects overlap between the `Player` entity and any `Asteroid` using `circles_are_colliding` (existing helper). On hit: despawn the ship entity, fire `ShipDestroyedEvent`.

#### `GameStatePlugin` (new — `src/plugins/game_state.rs`)

| System | Trigger / condition |
|---|---|
| `insert_game_resources` | `Startup` — inserts `Lives(3)` and `Score(0)` |
| `handle_attract_input` | `Update`, `state == Attract` — any key press fires `ResetGameEvent`, transitions to `Playing` |
| `handle_game_over_input` | `Update`, `state == GameOver` — same |
| `on_ship_destroyed` | `Update`, listens for `ShipDestroyedEvent` — decrements `Lives`; if `lives > 0` insert `RespawnTimer` and transition to `Dead`; else transition to `GameOver` |
| `tick_respawn_timer` | `Update`, `state == Dead` — ticks `RespawnTimer`; on finish remove timer, spawn ship with `Invincible` component, transition to `Playing` |
| `tick_invincibility` | `Update`, `state == Playing`, entity with `Invincible` — ticks timer, toggles `Visibility` every `SHIP_BLINK_INTERVAL_SECS`; on finish removes `Invincible` and restores `Visibility::Visible` |

#### `ShipPlugin` (modified)

- Remove `Startup` spawn — ship is no longer spawned at app launch.
- Listen for `ResetGameEvent`: despawn any existing `Player` entity, then spawn a fresh ship (no `Invincible`).
- `GameStatePlugin::tick_respawn_timer` spawns the respawn ship directly (with `Invincible`). Alternatively, fire a second event `SpawnShipEvent { invincible: bool }` and let `ShipPlugin` handle all spawning — choose whichever keeps the code clearest during execute.

#### `AsteroidPlugin` (modified)

- Keep `Startup` spawn for the attract screen (asteroids are present from launch).
- Listen for `ResetGameEvent`: despawn all `Asteroid` entities, then spawn a fresh set using the existing spawn logic.

#### `HudPlugin` (new — `src/plugins/hud.rs`)

| System | Trigger |
|---|---|
| `spawn_hud` | `Startup` — spawns lives text and score text as UI `Text` entities |
| `spawn_overlay_text` | `Startup` — spawns "Press Any Key to Start" and "Game Over" text; "Game Over" starts hidden |
| `update_hud` | `Update` — updates lives and score strings from `Lives` and `Score` resources |
| `show_game_over_text` | `OnEnter(GameState::GameOver)` — makes `GameOverText` visible |
| `hide_game_over_text` | `OnEnter(GameState::Playing)` — hides `GameOverText` |
| `show_press_any_key` | `OnEnter(GameState::Attract)` and `OnEnter(GameState::GameOver)` — makes `PressAnyKeyText` visible |
| `hide_press_any_key` | `OnEnter(GameState::Playing)` — hides `PressAnyKeyText` |

### Module structure

```
src/
  components.rs        — GameState, Lives, Score, Invincible, RespawnTimer, events, HUD markers
  config.rs            — SHIP_RADIUS, PLAYER_STARTING_LIVES, respawn/invincibility constants
  plugins/
    mod.rs             — expose GameStatePlugin, HudPlugin
    game_state.rs      — NEW: state transitions, respawn, reset
    hud.rs             — NEW: HUD text and overlay text
    ship.rs            — MODIFIED: remove Startup spawn, listen for ResetGameEvent
    asteroid.rs        — MODIFIED: listen for ResetGameEvent
    collision.rs       — MODIFIED: add ship_asteroid_collision system
    bullet.rs          — unchanged
  main.rs              — add GameStatePlugin, HudPlugin; register GameState with init_state
```

---

## Behavior

### Core loop

Every frame while `Playing`:
1. Movement systems run (ship, asteroids, bullets).
2. `bullet_asteroid_collision` checks bullets vs asteroids (existing).
3. `ship_asteroid_collision` checks the ship vs asteroids — skipped if ship has `Invincible`.
4. On hit: ship despawned, `ShipDestroyedEvent` fired.
5. `on_ship_destroyed` decrements lives, transitions to `Dead` or `GameOver`.

While `Dead`:
- Asteroids and bullets continue moving.
- `tick_respawn_timer` counts down 1.5 s.
- On expiry: spawn ship with `Invincible`, transition to `Playing`.

While `Playing` with `Invincible` on ship:
- `tick_invincibility` blinks the ship every `SHIP_BLINK_INTERVAL_SECS`.
- Collision system skips the ship.
- After 2.0 s: `Invincible` removed, ship becomes collidable again.

### State machine

See: `specs/diagrams/lives-game-over-states.excalidraw`

### Input/Output

| Input | Type | Source |
|---|---|---|
| Ship transform | `&Transform` | `Player` entity |
| Asteroid transform + size | `&Transform`, `&Asteroid` | `Asteroid` entities |
| Any key press | `ButtonInput<KeyCode>` | Bevy input resource |
| `ShipDestroyedEvent` | `EventReader` | collision system |
| `ResetGameEvent` | `EventReader` | game state system |

| Output | Type | Destination |
|---|---|---|
| `ShipDestroyedEvent` | `EventWriter` | game state system |
| `ResetGameEvent` | `EventWriter` | ship plugin, asteroid plugin |
| State transition | `NextState<GameState>` | Bevy state machine |
| `Lives` resource | `ResMut<Lives>` | HUD text |
| Despawn/spawn commands | `Commands` | ECS |

### Error handling

- All systems use `if let Ok(...)` or early return on `get_single` — no panics.
- If `ShipDestroyedEvent` fires when `Lives` is already 0, log a warning and transition to `GameOver` (defensive, should not happen in normal play).

---

## Edge Cases

- **Ship hit during invincibility**: `ship_asteroid_collision` guards with `Without<Invincible>` query filter — no special handling needed.
- **Multiple asteroids overlap ship in same frame**: `ShipDestroyedEvent` may fire more than once. `on_ship_destroyed` should be idempotent — guard against double-decrement by checking state before acting (only act when state is `Playing`).
- **Player presses key during Dead state**: ignore — `handle_attract_input` and `handle_game_over_input` are state-gated.
- **No asteroids on screen when ship respawns**: out of scope for this spec (new wave is a future feature); the player simply floats in empty space.
- **Reset while already in Attract**: harmless — `ResetGameEvent` re-despawns/respawns asteroids and resets resources.

---

## Performance Considerations

No specific constraints. The ship-asteroid collision is O(n) where n = asteroid count — at most ~18 asteroids (6 large → 12 medium → 24 small, but despawns happen along the way). No concern.

---

## Testing Strategy

Unit tests (no Bevy app required):
- Lives decrement logic is pure arithmetic — test the decrement and the `lives == 0` branch.
- `SHIP_RESPAWN_DELAY_SECS` and `SHIP_INVINCIBILITY_SECS` constants are present in config.

Integration testing is manual play:
- Ship hitting asteroid decrements life count in HUD.
- After 3 deaths, "Game Over" appears.
- Pressing any key resets to a fresh game (new asteroids, 3 lives, score 0).
- Invincibility blink is visible on respawn; ship becomes collidable after it stops.
- Attract screen shows no ship, asteroids moving; "Press Any Key to Start" visible.

```rust
// Key unit test
#[test]
fn lives_decrement_reaches_zero() {
    let mut lives = Lives(1);
    lives.0 = lives.0.saturating_sub(1);
    assert_eq!(lives.0, 0);
}
```

---

## Open Questions

- None — all decisions made.

---

## Diagrams

- `specs/diagrams/lives-game-over-states.excalidraw` — four-state game state machine (Attract, Playing, Dead/Respawning, Game Over) with all transitions labeled
