# Thrust Enhancements

**Status:** Draft
**Created:** 2026-03-22
**Spec author:** Refined via /refine skill

---

## Summary

Tune the ship's movement feel by doubling max speed and halving the friction coefficient, resulting in a faster top speed and a longer, more satisfying drift when coasting. Reverse thrust is explicitly out of scope for this iteration.

---

## Motivation

The current ship feels sluggish at top speed and decelerates too aggressively when not thrusting. Doubling `SHIP_MAX_SPEED` gives the player more dangerous momentum to manage. Reducing drag lets the ship coast longer, rewarding skilled players who use inertia tactically rather than fighting it.

---

## Scope

### In scope
- Increase `SHIP_MAX_SPEED` from 400 to 800
- Decrease drag from 0.98 to 0.99 (starting point — expected to require tuning)
- Both changes are config-only: no system logic changes required

### Out of scope
- Reverse thrust (deferred to a future spec)
- Changes to `SHIP_THRUST` acceleration rate
- Changes to rotation speed
- Any UI feedback for current speed

---

## Architecture

### Where it lives

`src/config.rs` only. No system code changes.

### Key types

No new types. The two constants to change:

```rust
// Before
pub const SHIP_MAX_SPEED: f32 = 400.0;
pub const SHIP_DRAG: f32 = 0.98;

// After (starting values — SHIP_DRAG expected to need tuning)
pub const SHIP_MAX_SPEED: f32 = 800.0;
pub const SHIP_DRAG: f32 = 0.99;
```

### Systems / ECS integration

No system changes. The existing `ship_thrust` system in `src/plugins/ship.rs` already:
1. Clamps to `SHIP_MAX_SPEED` when thrusting
2. Applies `SHIP_DRAG` unconditionally every frame (both thrusting and coasting)

Both behaviours continue unchanged — only the constant values shift.

---

## Behavior

### Core loop

Each frame in `ship_thrust`:
```
if thrusting:
    velocity += forward * SHIP_THRUST * dt
    velocity = clamp(velocity, SHIP_MAX_SPEED)   // now 800
velocity *= SHIP_DRAG                            // now 0.99
```

The drag model is exponential decay. At 60 fps:
- `SHIP_DRAG = 0.98` → ship at 800 px/s reaches ~1 px/s in ~5.5 seconds
- `SHIP_DRAG = 0.99` → ship at 800 px/s reaches ~1 px/s in ~11 seconds

Expect tuning to land `SHIP_DRAG` somewhere in the range `0.990–0.995`.

### Tuning guidance

`SHIP_DRAG` is the primary feel lever. Adjust in increments of 0.002 and playtest:
- Too slidey → decrease toward 0.98
- Still too sticky → increase toward 0.995

`SHIP_MAX_SPEED` at 800 is a hard target. Only adjust if collision feel breaks (ship outruns asteroids entirely) or the wrapping feel becomes disorienting.

---

## Edge Cases

- **Bullet speed**: `BULLET_SPEED = 500` is relative to the ship velocity (`bullet_vel = ship_vel + forward * BULLET_SPEED`). At max ship speed of 800, forward-fired bullets can reach ~1300 px/s. The existing `BULLET_LIFETIME = 1.2s` may need a small reduction if bullets exit the screen before crossing it — but this is a play-feel decision, not a correctness issue.
- **Asteroid relative speed**: Asteroids top out at 120 px/s. At ship max speed 800, the player can completely outrun all asteroids. This is intentional and consistent with the original Asteroids arcade game.

---

## Performance Considerations

No performance impact. Two constant value changes.

---

## Testing Strategy

Unit tests in `src/config.rs` (existing `#[cfg(test)]` block) — add:

```rust
#[test]
fn ship_max_speed_is_double_baseline() {
    // Documents the intent: max speed should be ~2x the original 400
    assert!(SHIP_MAX_SPEED >= 700.0 && SHIP_MAX_SPEED <= 900.0);
}

#[test]
fn ship_drag_is_less_than_original() {
    // Original was 0.98; new value must be less friction (closer to 1.0)
    assert!(SHIP_DRAG > 0.98 && SHIP_DRAG < 1.0);
}
```

Primary validation is manual playtesting — drift duration and top speed feel are subjective.

---

## Open Questions

- Final tuned value of `SHIP_DRAG` — start at 0.99, adjust through play
- Whether `BULLET_LIFETIME` needs a small trim after playtesting at new speeds

---

## Diagrams

- `specs/diagrams/thrust-enhancements.excalidraw` — ship velocity state machine showing Thrusting, Coasting, and the deferred Reverse Thrust state; annotated with per-frame formulas and drag application point
