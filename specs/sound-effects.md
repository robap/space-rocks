# Sound Effects

**Status:** Draft
**Created:** 2026-03-22
**Spec author:** Refined via /refine skill

---

## Summary

Add retro arcade sound effects to Space Rocks using Bevy's built-in audio system. Sounds cover the four core game actions: shooting, asteroid explosions (per size), ship destruction, and looping thruster.

---

## Motivation

The game is visually functional but silent. Sound effects complete the arcade feel and give the player moment-to-moment feedback on their actions. This is the first audio feature in the project.

---

## Scope

### In scope
- Re-enable `bevy_audio` in `Cargo.toml`
- New `SoundPlugin` in `src/plugins/sound.rs`
- One-shot sounds: bullet fired, asteroid explosion (3 size variants), ship destroyed
- Looping thruster sound that starts and stops with the thrust key
- New `BulletFiredEvent` and `AsteroidDestroyedEvent` to decouple audio from game logic
- Update `README.md` with Linux `libasound2-dev` build prerequisite
- List the required sound files for the player to generate (no bundled audio assets)

### Out of scope
- Background music
- Volume control / audio settings
- Positional / spatial audio
- Sound on game state transitions (attract, game over)
- Procedural audio generation

---

## Sound Files

These six files are included in `assets/sounds/` (generated via [jsfxr](https://sfxr.me)):

| File | Category hint in jsfxr |
|------|------------------------|
| `shoot.wav` | Shoot / Laser |
| `explosion_large.wav` | Explosion |
| `explosion_medium.wav` | Explosion (shorter / higher pitch) |
| `explosion_small.wav` | Explosion (shortest / highest pitch) |
| `ship_explosion.wav` | Explosion (distinct from asteroid sounds) |
| `thruster.wav` | Powerup / Pickup (loopable — trim to a short seamless loop) |

---

## Architecture

### Where it lives

`src/plugins/sound.rs` — a new `SoundPlugin`. Registered in `src/plugins/mod.rs` and added to the app in `src/main.rs`.

### Key types

```rust
// Loaded once at startup, stored as a resource
#[derive(Resource)]
pub struct SoundAssets {
    pub shoot: Handle<AudioSource>,
    pub explosion_large: Handle<AudioSource>,
    pub explosion_medium: Handle<AudioSource>,
    pub explosion_small: Handle<AudioSource>,
    pub ship_explosion: Handle<AudioSource>,
    pub thruster: Handle<AudioSource>,
}

// Tracks the entity carrying the looping thruster AudioPlayer so it can be despawned
#[derive(Resource, Default)]
pub struct ThrusterSoundEntity(pub Option<Entity>);

// New event sent by ship_shoot
#[derive(Event)]
pub struct BulletFiredEvent;

// New event sent by bullet_asteroid_collision, carries the size of the asteroid that was hit
#[derive(Event)]
pub struct AsteroidDestroyedEvent {
    pub size: AsteroidSize,
}
```

### New events

Two events are added to `src/components.rs` to decouple audio from game logic:

- `BulletFiredEvent` — sent by `ship_shoot` in `bullet.rs` each time a bullet is spawned
- `AsteroidDestroyedEvent { size: AsteroidSize }` — sent by `bullet_asteroid_collision` in `collision.rs` each time an asteroid is hit (one event per asteroid, regardless of whether it splits)

### Systems / ECS integration

| System | Reads | Writes | Trigger |
|--------|-------|--------|---------|
| `load_sounds` | `AssetServer` | `SoundAssets` (insert) | `Startup` |
| `play_shoot_sound` | `SoundAssets`, `BulletFiredEvent` | `Commands` (spawn audio entity) | `Update` |
| `play_asteroid_explosion` | `SoundAssets`, `AsteroidDestroyedEvent` | `Commands` (spawn audio entity) | `Update` |
| `play_ship_explosion` | `SoundAssets`, `ShipDestroyedEvent` | `Commands` (spawn audio entity) | `Update` |
| `manage_thruster_sound` | `SoundAssets`, `ThrusterSoundEntity`, `Query<&Thruster>` | `Commands` (spawn/despawn), `ThrusterSoundEntity` (mutate) | `Update` |

All sound systems run in `Update` with no ordering constraints relative to `GameSet` — they only react to events and component state, they do not move or despawn game entities.

### Module structure

```
src/plugins/sound.rs   — SoundPlugin, all 5 systems above
src/components.rs      — add BulletFiredEvent, AsteroidDestroyedEvent
src/plugins/ship.rs    — send BulletFiredEvent in ship_shoot (updated post-implementation: spec had wrong filename)
src/plugins/collision.rs — send AsteroidDestroyedEvent in bullet_asteroid_collision
```

---

## Behavior

### One-shot sounds

Spawn a temporary entity with `AudioPlayer(handle.clone())` and `PlaybackSettings::DESPAWN`. Bevy removes the entity automatically when playback finishes.

```rust
commands.spawn((
    AudioPlayer(assets.shoot.clone()),
    PlaybackSettings::DESPAWN,
));
```

### Thruster loop

The `manage_thruster_sound` system polls the `Thruster` component each frame:

- If `thruster.active == true` and `ThrusterSoundEntity(None)`: spawn a looping audio entity, store its `Entity` in `ThrusterSoundEntity`.
- If `thruster.active == false` and `ThrusterSoundEntity(Some(entity))`: despawn the entity, set `ThrusterSoundEntity(None)`.
- Otherwise: do nothing.

```rust
commands.spawn((
    AudioPlayer(assets.thruster.clone()),
    PlaybackSettings::LOOP,
));
```

### Asteroid explosion sound selection

```rust
let handle = match event.size {
    AsteroidSize::Large  => assets.explosion_large.clone(),
    AsteroidSize::Medium => assets.explosion_medium.clone(),
    AsteroidSize::Small  => assets.explosion_small.clone(),
};
```

---

## Cargo.toml change

Add `bevy_audio` and `wav` to the enabled features (updated post-implementation: `bevy_audio` alone is insufficient — `AudioSource` is only registered by `AudioPlugin` when at least one codec feature is also enabled):

```toml
bevy = { version = "0.15", default-features = false, features = [
    "bevy_audio",       // ← add this
    "wav",              // ← also required: enables AudioSource registration
    "bevy_core_pipeline",
    ...
] }
```

Remove the comment explaining why it was disabled.

---

## README update

Add a **Build Prerequisites** section noting that Linux requires:

```
sudo apt install libasound2-dev
```

macOS and Windows have no additional prerequisites.

---

## Edge Cases

- **No `Thruster` component present** (ship is despawned): `manage_thruster_sound` gets no query result; if `ThrusterSoundEntity` still holds an entity, despawn it. Prevents a ghost thruster sound looping after the ship dies.
- **Multiple `BulletFiredEvent` in one frame**: each fires its own sound — correct behaviour, rapid fire sounds normal.
- **Sound assets not yet loaded**: `AssetServer::load` returns a handle immediately; Bevy streams the asset. If a sound fires before the asset loads, Bevy queues playback — no special handling needed.

---

## Performance Considerations

No specific constraints. One-shot entities are despawned automatically. Six asset handles are small. The thruster system is O(1) per frame.

---

## Testing Strategy

No unit tests for audio systems — they are pure side effects (spawning entities, playing sounds). Manual play testing covers all cases:

- Fire bullet → hear shoot sound
- Shoot large asteroid → hear large explosion; two medium asteroids appear
- Shoot medium asteroid → hear medium explosion; two small asteroids appear
- Shoot small asteroid → hear small explosion; nothing spawns
- Hold thrust → thruster loop plays; release → loop stops
- Ship hit by asteroid → hear ship explosion

---

## Open Questions

None.

---

## Diagrams

- `specs/diagrams/sound-effects-initial.excalidraw` — initial understanding: trigger points → SoundPlugin → audio output
