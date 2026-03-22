# SoundPlugin — Technical Reference

**Source:** `src/plugins/sound.rs`
**Spec:** `specs/sound-effects.md`
**Review:** `reviews/sound-effects.md`
**Last updated:** 2026-03-22

---

## Overview

`SoundPlugin` adds arcade sound effects to the game: one-shot sounds for shooting, asteroid explosions (per size), and ship destruction, plus a looping thruster sound that starts and stops with the thrust key. Audio is loaded at startup via `AssetServer` and played by spawning temporary `AudioPlayer` entities. The plugin owns all audio resources and systems.

---

## Key Types

```rust
// Loaded once at startup. Holds handles to all six sound assets.
#[derive(Resource)]
pub struct SoundAssets {
    pub shoot: Handle<AudioSource>,
    pub explosion_large: Handle<AudioSource>,
    pub explosion_medium: Handle<AudioSource>,
    pub explosion_small: Handle<AudioSource>,
    pub ship_explosion: Handle<AudioSource>,
    pub thruster: Handle<AudioSource>,
}

// Tracks the entity carrying the looping thruster AudioPlayer.
// None when the thruster is silent; Some(entity) when it is playing.
#[derive(Resource, Default)]
pub struct ThrusterSoundEntity(pub Option<Entity>);
```

Events used (defined in `src/components.rs`, registered by `SoundPlugin`):

```rust
#[derive(Event)]
pub struct BulletFiredEvent;            // sent by ship_shoot in ship.rs

#[derive(Event)]
pub struct AsteroidDestroyedEvent {
    pub size: AsteroidSize,             // selects which explosion sound to play
}                                       // sent by bullet_asteroid_collision in collision.rs
```

---

## Architecture

All audio logic lives in one file. Five systems cover the four sound triggers:

```
src/plugins/sound.rs
  load_sounds              — Startup: loads all .wav files, inserts SoundAssets resource
  play_shoot_sound         — Update: fires a one-shot sound for each BulletFiredEvent
  play_asteroid_explosion  — Update: fires a size-matched one-shot for each AsteroidDestroyedEvent
  play_ship_explosion      — Update: fires a one-shot sound for each ShipDestroyedEvent
  manage_thruster_sound    — Update: starts/stops a looping entity based on Thruster.active
```

---

## Data Flow

### One-shot sounds

```
BulletFiredEvent (ship.rs)
  └─► play_shoot_sound
        └─► commands.spawn((AudioPlayer(assets.shoot.clone()), PlaybackSettings::DESPAWN))
              └─► Bevy AudioPlugin plays the sound, then despawns the entity automatically
```

Same pattern for asteroid explosions (`AsteroidDestroyedEvent`) and ship destruction (`ShipDestroyedEvent`). Asteroid explosion picks its handle via an exhaustive `match` on `AsteroidSize`.

### Looping thruster

```
manage_thruster_sound (runs every Update frame)
  reads: Thruster.active via Query<&Thruster, With<Player>>
         ThrusterSoundEntity (current loop entity, if any)

  if active && no entity playing:
    spawn (AudioPlayer(thruster_handle), PlaybackSettings::LOOP)
    store returned Entity in ThrusterSoundEntity

  if !active && entity playing:
    despawn the entity
    clear ThrusterSoundEntity
```

`unwrap_or(false)` on the thruster query means "ship not present" is treated identically to "thruster key not held". This correctly stops the loop when the ship is despawned mid-thrust.

---

## Integration Points

| System / Plugin | Relationship |
|-----------------|--------------|
| `ShipPlugin` (`ship_shoot`) | Sends `BulletFiredEvent` after spawning each bullet |
| `CollisionPlugin` (`bullet_asteroid_collision`) | Sends `AsteroidDestroyedEvent { size }` after despawning each asteroid |
| `GameStatePlugin` | Sends `ShipDestroyedEvent` — consumed by `play_ship_explosion` |
| `Bevy AudioPlugin` (via `DefaultPlugins`) | Processes `AudioPlayer` + `PlaybackSettings` components; auto-despawns `DESPAWN` entities |

Sound systems are unordered relative to `GameSet` — they only react to events and component state, they do not move or despawn game entities.

---

## Design Decisions

- **Events decouple audio from game logic.** `ship_shoot` and `bullet_asteroid_collision` send events; `SoundPlugin` listens. Neither side needs to know the other exists. Adding, removing, or replacing audio has zero impact on game logic.

- **Looping thruster uses an entity, not a flag.** Bevy's audio system requires an entity with `AudioPlayer` to play continuously. `ThrusterSoundEntity` tracks it so we can despawn it to stop. A boolean flag alone isn't enough.

- **`PlaybackSettings::DESPAWN` for one-shots.** Bevy removes the entity automatically when playback finishes. No cleanup system needed.

- **`SoundAssets` is inserted at `Startup`, not `Default`-initialized.** The handles require `AssetServer`, so they can't be constructed without it. Any `Update` system that tries to access `Res<SoundAssets>` before `Startup` completes will panic — but Bevy's schedule guarantees `Startup` runs first.

---

## Known Constraints and Gotchas

- **Both `bevy_audio` and `wav` features are required in `Cargo.toml`.** `bevy_audio` alone is insufficient: `AudioPlugin` only registers `AudioSource` (and thus allows `asset_server.load()`) when at least one codec feature is also present. Without `wav`, `load_sounds` panics at startup.

- **Sound files are bundled** in `assets/sounds/` — all six `.wav` files ship with the game. If a file is ever missing, Bevy logs a warning and plays nothing rather than crashing.

- **`manage_thruster_sound` runs unconditionally** (no `run_if` guard). It's O(1) per frame and handles the "no ship" case gracefully, so a state filter adds no benefit.

- **No volume control.** `GlobalVolume` defaults to 1.0. Changing master volume would go in `SoundPlugin::build` via `.insert_resource(GlobalVolume::new(v))`.
