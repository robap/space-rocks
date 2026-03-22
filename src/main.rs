mod components;
mod config;
mod plugins;

use bevy::prelude::*;
use plugins::{
    asteroid::AsteroidPlugin, bullet::BulletPlugin, collision::CollisionPlugin, ship::ShipPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((ShipPlugin, AsteroidPlugin, BulletPlugin, CollisionPlugin))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
