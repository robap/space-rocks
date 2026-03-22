# How to Play Space Rocks

Space Rocks is a classic Asteroids-style game. You pilot a ship through a field of drifting rocks. Shoot them before they hit you — but watch out, they break apart when you do.

---

## Your Ship

Your ship starts at the centre of the screen, pointing upward. It moves like a real spacecraft: you build up speed by thrusting, and you drift until drag slows you down. There are no brakes.

### Controls

| Action | Keys |
|--------|------|
| Rotate left | `A` or `←` |
| Rotate right | `D` or `→` |
| Thrust forward | `W` or `↑` |
| Fire | `Space` |

### Movement tips

- **Point first, then thrust.** Rotate to face the direction you want to go, then press thrust. Your ship will drift in that direction.
- **You can't stop instantly.** Let off thrust early and drift to a stop, or thrust in the opposite direction to slow down.
- **You wrap around.** Fly off the right edge and you'll appear on the left. Same for top and bottom.

---

## Shooting

Press `Space` to fire a bullet in the direction your ship is facing. Bullets travel fast and disappear after about a second if they don't hit anything.

- Bullets inherit your ship's speed — if you're flying fast, your bullets travel faster in that direction.
- You can fire as fast as you can press Space.

---

## Asteroids

Large grey asteroids drift across the screen at random. They wrap around the edges just like your ship.

- **Shoot a large asteroid** → it splits into two medium ones.
- **Shoot a medium asteroid** → it splits into two small ones.
- **Shoot a small asteroid** → it's destroyed completely.

The smaller pieces move faster and spread apart when they split, so clearing a field gets busier before it gets quieter.

---

## Scoring

Every asteroid you destroy earns points. Smaller asteroids are harder to hit, so they're worth more.

| Asteroid | Points |
|----------|--------|
| Large | 20 |
| Medium | 50 |
| Small | 100 |

Your current score is shown in the **top-right corner** of the screen. It resets to zero when you start a new game.

---

## Sound

Every action has a sound. You'll hear:

- A **laser shot** each time you fire.
- An **explosion** when an asteroid is destroyed — large asteroids make a bigger boom than small ones.
- A **thruster hum** while you're holding the thrust key. It stops the moment you let go.
- A **ship explosion** if an asteroid hits you.

Sound files are included in `assets/sounds/` and play automatically.
