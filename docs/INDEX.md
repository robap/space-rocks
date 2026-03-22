# Documentation Index

## Technical

- [Game Architecture](technical/game-architecture.md) — `main.rs`, `components.rs`, `config.rs`, `GameSet` system ordering, rendering conventions
- [ShipPlugin](technical/ship-plugin.md) — systems, types, and design decisions for player ship movement, rotation, and bullet firing
- [AsteroidPlugin](technical/asteroid-plugin.md) — asteroid spawning, movement, screen wrapping, and split data model
- [BulletPlugin](technical/bullet-plugin.md) — bullet movement, screen wrapping, lifetime countdown, and despawn
- [CollisionPlugin](technical/collision-plugin.md) — bullet↔asteroid hit detection, asteroid splitting, system set ordering
- [SoundPlugin](technical/sound-plugin.md) — audio asset loading, one-shot and looping sound systems, BulletFiredEvent and AsteroidDestroyedEvent wiring
- [ScorePlugin](technical/score-plugin.md) — point constants, AsteroidDestroyedEvent registration, score accumulation, integration with CollisionPlugin and HudPlugin

## User

- [How to Play](user/how-to-play.md) — controls, movement, shooting, and asteroid behaviour
