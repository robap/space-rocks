# Space Rocks — Project Conventions

## Project
Rust + Bevy game. Asteroids-style. Built as a learning project for the Refine → Plan → Execute → Review skill pipeline.

## Skill Workflow
1. `/refine` — produce a spec in `specs/`
2. `/plan` — produce a plan in `plans/` from the spec
3. `/execute` — implement the plan task by task with TDD
4. `/review` — validate implementation against the spec

## Architecture
- Single binary crate — no Cargo workspace
- Plugin-per-feature: `ShipPlugin`, `AsteroidPlugin`, `BulletPlugin`, `CollisionPlugin`
- All shared component/resource types live in `src/components.rs`
- Tunable constants live in `src/config.rs` — no magic numbers in systems

## Rust Conventions
- No `unwrap()` in game systems — use `if let` or early return
- Invariant violations that should never happen: `panic!()` with a descriptive message
- No `Rc<RefCell<T>>` — use Bevy resources and ECS instead
- Keep systems small and focused — one concern per system function
- Prefer marker components (`struct Player;`) over boolean flags on a shared component

## Bevy Conventions
- Use `Transform` for position and rotation (do not create custom position components)
- `Velocity(Vec2)` is a custom component for linear movement
- Each plugin registers only its own systems and components
- System ordering: movement systems before collision, collision before despawn
- Use `Commands` for spawning/despawning — never mutate entity lists directly

## Testing
- Unit test pure logic (split logic, config math) — no Bevy app required
- Integration testing is manual play for MVP
- Tests live alongside the code in `#[cfg(test)]` modules

## Out of Scope (MVP)
No sound, score, lives, UI, menus, WASM, or networking.
