use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::*;
use crate::config::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ship).add_systems(
            Update,
            (
                ship_rotation,
                ship_thrust,
                ship_movement,
                wrap_ship,
                ship_shoot,
            ),
        );
    }
}

fn spawn_ship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let triangle = Triangle2d::new(
        Vec2::new(0.0, 20.0),
        Vec2::new(-12.0, -14.0),
        Vec2::new(12.0, -14.0),
    );
    commands.spawn((
        Mesh2d(meshes.add(triangle)),
        MeshMaterial2d(materials.add(Color::srgb(0.8, 0.9, 1.0))),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Player,
        Velocity(Vec2::ZERO),
        Thruster::default(),
    ));
}

fn ship_rotation(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = query.get_single_mut() else {
        return;
    };
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        transform.rotate_z(SHIP_ROTATION_SPEED * time.delta_secs());
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        transform.rotate_z(-SHIP_ROTATION_SPEED * time.delta_secs());
    }
}

fn ship_thrust(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Transform, &mut Thruster), With<Player>>,
) {
    let Ok((mut velocity, transform, mut thruster)) = query.get_single_mut() else {
        return;
    };
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        let forward = (transform.rotation * Vec3::Y).truncate();
        velocity.0 += forward * SHIP_THRUST * time.delta_secs();
        velocity.0 = clamp_to_max_speed(velocity.0, SHIP_MAX_SPEED);
        thruster.active = true;
    } else {
        thruster.active = false;
    }
    velocity.0 *= SHIP_DRAG;
}

fn clamp_to_max_speed(velocity: Vec2, max_speed: f32) -> Vec2 {
    if velocity.length() > max_speed {
        velocity.normalize() * max_speed
    } else {
        velocity
    }
}

fn ship_movement(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    let Ok((velocity, mut transform)) = query.get_single_mut() else {
        return;
    };
    transform.translation += velocity.0.extend(0.0) * time.delta_secs();
}

fn wrap_ship(
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;

    let Ok(mut transform) = query.get_single_mut() else {
        return;
    };
    wrap_position(&mut transform.translation, half_w, half_h);
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

fn ship_shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<(&Transform, &Velocity), With<Player>>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }
    let Ok((transform, ship_velocity)) = query.get_single() else {
        return;
    };
    let forward = (transform.rotation * Vec3::Y).truncate();
    let spawn_pos = transform.translation + (forward * BULLET_SPAWN_OFFSET).extend(0.0);
    let bullet_vel = ship_velocity.0 + forward * BULLET_SPEED;

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BULLET_RADIUS))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.5))),
        Transform::from_translation(spawn_pos),
        Bullet,
        Velocity(bullet_vel),
        BulletLifetime(Timer::from_seconds(BULLET_LIFETIME, TimerMode::Once)),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velocity_below_max_speed_is_unchanged() {
        let vel = Vec2::new(100.0, 0.0);
        assert_eq!(clamp_to_max_speed(vel, 400.0), vel);
    }

    #[test]
    fn test_velocity_at_max_speed_is_unchanged() {
        let vel = Vec2::new(400.0, 0.0);
        assert_eq!(clamp_to_max_speed(vel, 400.0), vel);
    }

    #[test]
    fn test_velocity_exceeding_max_speed_is_clamped() {
        let vel = Vec2::new(600.0, 0.0);
        let clamped = clamp_to_max_speed(vel, 400.0);
        assert!((clamped.length() - 400.0).abs() < 1e-5);
        assert!((clamped.x - 400.0).abs() < 1e-5);
    }

    #[test]
    fn test_diagonal_velocity_exceeding_max_speed_preserves_direction() {
        let vel = Vec2::new(500.0, 500.0);
        let clamped = clamp_to_max_speed(vel, 400.0);
        assert!((clamped.length() - 400.0).abs() < 1e-4);
        assert!((clamped.x - clamped.y).abs() < 1e-4);
    }

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
