use crate::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_bullets, wrap_bullets).in_set(GameSet::Movement),
        )
        .add_systems(Update, bullet_lifetime.in_set(GameSet::Despawn));
    }
}

fn move_bullets(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform), With<Bullet>>) {
    for (velocity, mut transform) in &mut query {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}

fn wrap_bullets(
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<Bullet>>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;

    for mut transform in &mut query {
        if transform.translation.x > half_w {
            transform.translation.x = -half_w;
        }
        if transform.translation.x < -half_w {
            transform.translation.x = half_w;
        }
        if transform.translation.y > half_h {
            transform.translation.y = -half_h;
        }
        if transform.translation.y < -half_h {
            transform.translation.y = half_h;
        }
    }
}

fn bullet_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut BulletLifetime)>,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.0.tick(time.delta());
        if lifetime.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}
