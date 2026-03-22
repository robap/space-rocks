use bevy::prelude::*;
use rand::Rng;
use std::collections::HashSet;

use crate::components::*;
use crate::config::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bullet_asteroid_collision.in_set(GameSet::Collision))
            .add_systems(
                Update,
                ship_asteroid_collision
                    .in_set(GameSet::Collision)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn bullet_asteroid_collision(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    asteroids: Query<(Entity, &Transform, &Asteroid)>,
    mut asteroid_destroyed: EventWriter<AsteroidDestroyedEvent>,
) {
    let mut hit_asteroids: HashSet<Entity> = HashSet::new();

    for (bullet_entity, bullet_transform) in &bullets {
        for (asteroid_entity, asteroid_transform, asteroid) in &asteroids {
            if hit_asteroids.contains(&asteroid_entity) {
                continue;
            }
            let bullet_pos = bullet_transform.translation.truncate();
            let asteroid_pos = asteroid_transform.translation.truncate();
            if circles_are_colliding(
                bullet_pos,
                asteroid_pos,
                BULLET_RADIUS,
                asteroid.size.radius(),
            ) {
                commands.entity(bullet_entity).despawn();
                commands.entity(asteroid_entity).despawn();
                hit_asteroids.insert(asteroid_entity);
                asteroid_destroyed.send(AsteroidDestroyedEvent { size: asteroid.size });
                spawn_split_asteroids(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    asteroid_transform.translation,
                    asteroid.size,
                );
                break;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn ship_asteroid_collision(
    mut commands: Commands,
    mut destroyed: EventWriter<ShipDestroyedEvent>,
    ship: Query<(Entity, &Transform), (With<Player>, Without<Invincible>)>,
    asteroids: Query<(&Transform, &Asteroid)>,
) {
    let Ok((ship_entity, ship_transform)) = ship.get_single() else {
        return;
    };
    let ship_pos = ship_transform.translation.truncate();

    for (asteroid_transform, asteroid) in &asteroids {
        let asteroid_pos = asteroid_transform.translation.truncate();
        if circles_are_colliding(ship_pos, asteroid_pos, SHIP_RADIUS, asteroid.size.radius()) {
            commands.entity(ship_entity).despawn();
            destroyed.send(ShipDestroyedEvent);
            return;
        }
    }
}

fn spawn_split_asteroids(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
    size: AsteroidSize,
) {
    let Some(child_size) = size.split() else {
        return;
    };

    let mut rng = rand::thread_rng();
    let base_angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let split_offset = std::f32::consts::FRAC_PI_6; // ±30°

    for angle_offset in [split_offset, -split_offset] {
        let angle = base_angle + angle_offset;
        let speed = rng.gen_range(ASTEROID_MIN_SPEED..ASTEROID_MAX_SPEED);
        let vel = Vec2::new(angle.cos(), angle.sin()) * speed;
        let ang_vel = rng.gen_range(ASTEROID_MIN_ANGULAR_VELOCITY..ASTEROID_MAX_ANGULAR_VELOCITY);

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(child_size.radius()))),
            MeshMaterial2d(materials.add(Color::srgb(0.6, 0.6, 0.6))),
            Transform::from_translation(position),
            Asteroid { size: child_size },
            Velocity(vel),
            AngularVelocity(ang_vel),
        ));
    }
}

fn circles_are_colliding(pos_a: Vec2, pos_b: Vec2, radius_a: f32, radius_b: f32) -> bool {
    pos_a.distance(pos_b) < radius_a + radius_b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlapping_circles_are_colliding() {
        assert!(circles_are_colliding(
            Vec2::ZERO,
            Vec2::new(5.0, 0.0),
            4.0,
            4.0
        ));
    }

    #[test]
    fn test_touching_circles_are_not_colliding() {
        assert!(!circles_are_colliding(
            Vec2::ZERO,
            Vec2::new(8.0, 0.0),
            4.0,
            4.0
        ));
    }

    #[test]
    fn test_separated_circles_are_not_colliding() {
        assert!(!circles_are_colliding(
            Vec2::ZERO,
            Vec2::new(10.0, 0.0),
            4.0,
            4.0
        ));
    }

    #[test]
    fn test_overlapping_circles_at_diagonal_are_colliding() {
        // 3-4-5 right triangle: distance = 5.0, sum of radii = 6.0 → collision
        assert!(circles_are_colliding(
            Vec2::ZERO,
            Vec2::new(3.0, 4.0),
            3.0,
            3.0
        ));
    }
}
