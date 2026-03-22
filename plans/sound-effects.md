# Plan: Sound Effects

**Spec:** `specs/sound-effects.md`
**Status:** Not started
**Created:** 2026-03-22

---

## Overview

Add `bevy_audio` to the dependency features, introduce two new game events (`BulletFiredEvent`, `AsteroidDestroyedEvent`), wire those events into the existing `ship_shoot` and `bullet_asteroid_collision` systems, then build a new `SoundPlugin` with five systems that react to events and component state to play sounds. Order: infrastructure first, then events, then senders, then the plugin, then app wiring. The codebase must compile cleanly after each group.

**Note:** `ship_shoot` lives in `src/plugins/ship.rs`, not `bullet.rs` — the spec module list has a typo. All tasks below use the correct file.

---

## Prerequisites

- None

---

## Tasks

### 1. Infrastructure — Cargo.toml and README

> Enable `bevy_audio` in the Bevy feature list and document the Linux system dependency in the README. No Rust changes yet.

- [x] **1.1** In `Cargo.toml`, add `"bevy_audio"` to the `bevy` features list and remove the comment that explained why it was disabled.
- [x] **1.2** In `README.md`, add a **Build Prerequisites** section (before the Running section) documenting that Linux requires `sudo apt install libasound2-dev` before building, and that macOS and Windows have no additional prerequisites.

*Checkpoint: `cargo build` compiles cleanly with audio support enabled. No code changes yet.*

---

### 2. New events in components.rs

> Add `BulletFiredEvent` and `AsteroidDestroyedEvent` to the shared types file. They are defined here so both senders and the listener can import from `crate::components`.

- [x] **2.1** In `src/components.rs`, add:
  ```rust
  #[derive(Event)]
  pub struct BulletFiredEvent;
  ```
- [x] **2.2** In `src/components.rs`, add:
  ```rust
  #[derive(Event)]
  pub struct AsteroidDestroyedEvent {
      pub size: AsteroidSize,
  }
  ```

*Checkpoint: `cargo build` compiles cleanly. Events are defined but not yet registered or sent.*

---

### 3. Send events from existing systems

> Wire the new events into the two existing systems that generate the relevant game actions. No new files — only small parameter additions.

- [x] **3.1** In `src/plugins/ship.rs`, update `ship_shoot` to accept an `EventWriter<BulletFiredEvent>` parameter and call `bullet_fired.send(BulletFiredEvent)` immediately after spawning the bullet entity.
- [x] **3.2** In `src/plugins/collision.rs`, update `bullet_asteroid_collision` to accept an `EventWriter<AsteroidDestroyedEvent>` parameter and call `asteroid_destroyed.send(AsteroidDestroyedEvent { size: asteroid.size })` immediately after despawning the asteroid entity (before calling `spawn_split_asteroids`).

*Checkpoint: `cargo build` compiles cleanly. Events are sent but not yet registered — the app won't run correctly until Task 4 registers them, but the build must be clean.*

---

### 4. SoundPlugin

> Create `src/plugins/sound.rs` with all resources, asset loading, and five audio systems. This is the core of the feature.

- [x] **4.1** Create `src/plugins/sound.rs` with a stub `pub struct SoundPlugin;` and an empty `impl Plugin for SoundPlugin { fn build(&self, _app: &mut App) {} }`. Confirm it compiles.

- [x] **4.2** Define `SoundAssets` and `ThrusterSoundEntity` resources in `sound.rs`:
  ```rust
  #[derive(Resource)]
  pub struct SoundAssets {
      pub shoot: Handle<AudioSource>,
      pub explosion_large: Handle<AudioSource>,
      pub explosion_medium: Handle<AudioSource>,
      pub explosion_small: Handle<AudioSource>,
      pub ship_explosion: Handle<AudioSource>,
      pub thruster: Handle<AudioSource>,
  }

  #[derive(Resource, Default)]
  pub struct ThrusterSoundEntity(pub Option<Entity>);
  ```

- [x] **4.3** Implement `load_sounds` startup system in `sound.rs` that loads all six `.wav` files from `assets/sounds/` via `AssetServer` and inserts `SoundAssets` as a resource:
  ```rust
  fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
      commands.insert_resource(SoundAssets {
          shoot:            asset_server.load("sounds/shoot.wav"),
          explosion_large:  asset_server.load("sounds/explosion_large.wav"),
          explosion_medium: asset_server.load("sounds/explosion_medium.wav"),
          explosion_small:  asset_server.load("sounds/explosion_small.wav"),
          ship_explosion:   asset_server.load("sounds/ship_explosion.wav"),
          thruster:         asset_server.load("sounds/thruster.wav"),
      });
  }
  ```

- [x] **4.4** Implement `play_shoot_sound` system in `sound.rs`. Reads `BulletFiredEvent` and `SoundAssets`; spawns `(AudioPlayer(assets.shoot.clone()), PlaybackSettings::DESPAWN_ON_END)` for each event.

- [x] **4.5** Implement `play_asteroid_explosion` system in `sound.rs`. Reads `AsteroidDestroyedEvent` and `SoundAssets`; selects the handle based on `event.size` and spawns a one-shot audio entity with `PlaybackSettings::DESPAWN_ON_END`.

- [x] **4.6** Implement `play_ship_explosion` system in `sound.rs`. Reads `ShipDestroyedEvent` and `SoundAssets`; spawns a one-shot audio entity for `assets.ship_explosion` with `PlaybackSettings::DESPAWN_ON_END`.

- [x] **4.7** Implement `manage_thruster_sound` system in `sound.rs`. Signature:
  ```rust
  fn manage_thruster_sound(
      mut commands: Commands,
      assets: Res<SoundAssets>,
      mut thruster_entity: ResMut<ThrusterSoundEntity>,
      thruster_query: Query<&Thruster, With<Player>>,
  )
  ```
  Logic:
  - `let is_active = thruster_query.get_single().map(|t| t.active).unwrap_or(false);`
  - If `is_active && thruster_entity.0.is_none()`: spawn `(AudioPlayer(assets.thruster.clone()), PlaybackSettings::LOOP)`, store the returned `Entity` in `thruster_entity.0`.
  - If `!is_active`, and `thruster_entity.0.is_some()`: despawn the stored entity, set `thruster_entity.0 = None`.
  - This handles both "key released" and "ship despawned" cases — `unwrap_or(false)` makes both stop the loop.

- [x] **4.8** Fill in `SoundPlugin::build` to register everything:
  ```rust
  fn build(&self, app: &mut App) {
      app
          .add_event::<BulletFiredEvent>()
          .add_event::<AsteroidDestroyedEvent>()
          .init_resource::<ThrusterSoundEntity>()
          .add_systems(Startup, load_sounds)
          .add_systems(Update, (
              play_shoot_sound,
              play_asteroid_explosion,
              play_ship_explosion,
              manage_thruster_sound,
          ));
  }
  ```

*Checkpoint: `cargo build` compiles cleanly. All five systems exist, two events are registered, `ThrusterSoundEntity` is initialised via `Default`. The plugin is not yet added to the app.*

---

### 5. Wire into the app

> Register the new module and add `SoundPlugin` to the Bevy app. After this group the feature is fully integrated.

- [ ] **5.1** In `src/plugins/mod.rs`, add `pub mod sound;`.
- [ ] **5.2** In `src/main.rs`, import `plugins::sound::SoundPlugin` and add `SoundPlugin` to the `add_plugins((...))` call alongside the other plugins.

*Checkpoint: `cargo build` compiles cleanly. `cargo run` launches the game. With sound files present in `assets/sounds/`, all sound effects play correctly. Without the files, the game runs silently (Bevy logs a warning for missing assets but does not panic).*

---

## Open Questions

None — all spec open questions were resolved during refinement.

---

## Notes for Execute

- **`bevy_audio` + `DefaultPlugins`**: Adding `"bevy_audio"` to the Bevy feature list is sufficient. `DefaultPlugins` already includes `AudioPlugin`; it just wasn't compiled in before.
- **`PlaybackSettings::DESPAWN_ON_END`**: This is a Bevy 0.15 constant on `PlaybackSettings`. One-shot audio entities clean themselves up automatically — no manual despawn needed.
- **`PlaybackSettings::LOOP`**: Also a Bevy 0.15 constant. The thruster entity must be manually despawned to stop looping — that's what `ThrusterSoundEntity` tracks.
- **Asset files**: The game will compile and run without the `.wav` files present. Bevy will log a warning per missing asset but won't panic. The sound features simply won't play until the files are added to `assets/sounds/`.
- **Spec typo**: The spec's module structure table lists `bullet.rs` as the file to modify for `BulletFiredEvent`. The correct file is `src/plugins/ship.rs` — that's where `ship_shoot` lives.
- **No unit tests**: Audio systems are pure side effects (entity spawning). Manual play testing per the spec's testing strategy is the verification method.
