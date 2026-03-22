use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::components::*;
use crate::config::*;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_asteroids)
            .add_systems(Update, (move_asteroids, wrap_asteroids));
    }
}

fn spawn_asteroids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;
    let mut rng = rand::thread_rng();

    for _ in 0..ASTEROID_INITIAL_COUNT {
        let (x, y) = random_edge_position(&mut rng, half_w, half_h);
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(ASTEROID_MIN_SPEED..ASTEROID_MAX_SPEED);
        let vel = Vec2::new(angle.cos(), angle.sin()) * speed;
        let ang_vel = rng.gen_range(ASTEROID_MIN_ANGULAR_VELOCITY..ASTEROID_MAX_ANGULAR_VELOCITY);

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(AsteroidSize::Large.radius()))),
            MeshMaterial2d(materials.add(Color::srgb(0.6, 0.6, 0.6))),
            Transform::from_xyz(x, y, 0.0),
            Asteroid {
                size: AsteroidSize::Large,
            },
            Velocity(vel),
            AngularVelocity(ang_vel),
        ));
    }
}

fn random_edge_position(rng: &mut impl Rng, half_w: f32, half_h: f32) -> (f32, f32) {
    match rng.gen_range(0u8..4) {
        0 => (rng.gen_range(-half_w..half_w), half_h),
        1 => (rng.gen_range(-half_w..half_w), -half_h),
        2 => (-half_w, rng.gen_range(-half_h..half_h)),
        _ => (half_w, rng.gen_range(-half_h..half_h)),
    }
}

fn move_asteroids(
    time: Res<Time>,
    mut query: Query<(&Velocity, &AngularVelocity, &mut Transform), With<Asteroid>>,
) {
    for (vel, ang_vel, mut transform) in &mut query {
        transform.translation += vel.0.extend(0.0) * time.delta_secs();
        transform.rotate_z(ang_vel.0 * time.delta_secs());
    }
}

fn wrap_asteroids(
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<Asteroid>>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;

    for mut transform in &mut query {
        wrap_position(&mut transform.translation, half_w, half_h);
    }
}

fn wrap_position(translation: &mut Vec3, half_w: f32, half_h: f32) {
    if translation.x > half_w {
        translation.x = -half_w;
    }
    if translation.x < -half_w {
        translation.x = half_w;
    }
    if translation.y > half_h {
        translation.y = -half_h;
    }
    if translation.y < -half_h {
        translation.y = half_h;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_position_past_right_edge_wraps_to_left() {
        let mut pos = Vec3::new(600.0, 0.0, 0.0);
        wrap_position(&mut pos, 500.0, 300.0);
        assert_eq!(pos.x, -500.0);
    }

    #[test]
    fn test_wrap_position_past_left_edge_wraps_to_right() {
        let mut pos = Vec3::new(-600.0, 0.0, 0.0);
        wrap_position(&mut pos, 500.0, 300.0);
        assert_eq!(pos.x, 500.0);
    }

    #[test]
    fn test_wrap_position_past_top_edge_wraps_to_bottom() {
        let mut pos = Vec3::new(0.0, 400.0, 0.0);
        wrap_position(&mut pos, 500.0, 300.0);
        assert_eq!(pos.y, -300.0);
    }

    #[test]
    fn test_wrap_position_past_bottom_edge_wraps_to_top() {
        let mut pos = Vec3::new(0.0, -400.0, 0.0);
        wrap_position(&mut pos, 500.0, 300.0);
        assert_eq!(pos.y, 300.0);
    }

    #[test]
    fn test_wrap_position_within_bounds_is_unchanged() {
        let mut pos = Vec3::new(100.0, 150.0, 0.0);
        wrap_position(&mut pos, 500.0, 300.0);
        assert_eq!(pos, Vec3::new(100.0, 150.0, 0.0));
    }
}
