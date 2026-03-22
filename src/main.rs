mod components;
mod config;
mod plugins;

use bevy::prelude::*;
use components::{GameSet, GameState};
use plugins::{
    asteroid::AsteroidPlugin, bullet::BulletPlugin, collision::CollisionPlugin,
    game_state::GameStatePlugin, hud::HudPlugin, ship::ShipPlugin, sound::SoundPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins((
            ShipPlugin,
            AsteroidPlugin,
            BulletPlugin,
            CollisionPlugin,
            GameStatePlugin,
            HudPlugin,
            SoundPlugin,
        ))
        .configure_sets(
            Update,
            (GameSet::Movement, GameSet::Collision, GameSet::Despawn).chain(),
        )
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
