use bevy::prelude::*;

use crate::components::*;

pub struct SoundPlugin;

#[derive(Resource)]
pub struct SoundAssets {
    pub shoot: Handle<AudioSource>,
    pub explosion_large: Handle<AudioSource>,
    pub explosion_medium: Handle<AudioSource>,
    pub explosion_small: Handle<AudioSource>,
    pub ship_explosion: Handle<AudioSource>,
    pub thruster: Handle<AudioSource>,
}

#[derive(Resource, Default)]
pub struct ThrusterSoundEntity(pub Option<Entity>);

fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SoundAssets {
        shoot: asset_server.load("sounds/shoot.wav"),
        explosion_large: asset_server.load("sounds/explosion_large.wav"),
        explosion_medium: asset_server.load("sounds/explosion_medium.wav"),
        explosion_small: asset_server.load("sounds/explosion_small.wav"),
        ship_explosion: asset_server.load("sounds/ship_explosion.wav"),
        thruster: asset_server.load("sounds/thruster.wav"),
    });
}

fn play_shoot_sound(
    mut commands: Commands,
    assets: Res<SoundAssets>,
    mut events: EventReader<BulletFiredEvent>,
) {
    for _ in events.read() {
        commands.spawn((
            AudioPlayer(assets.shoot.clone()),
            PlaybackSettings::DESPAWN_ON_END,
        ));
    }
}

fn play_asteroid_explosion(
    mut commands: Commands,
    assets: Res<SoundAssets>,
    mut events: EventReader<AsteroidDestroyedEvent>,
) {
    for event in events.read() {
        let handle = match event.size {
            AsteroidSize::Large => assets.explosion_large.clone(),
            AsteroidSize::Medium => assets.explosion_medium.clone(),
            AsteroidSize::Small => assets.explosion_small.clone(),
        };
        commands.spawn((AudioPlayer(handle), PlaybackSettings::DESPAWN_ON_END));
    }
}

fn play_ship_explosion(
    mut commands: Commands,
    assets: Res<SoundAssets>,
    mut events: EventReader<ShipDestroyedEvent>,
) {
    for _ in events.read() {
        commands.spawn((
            AudioPlayer(assets.ship_explosion.clone()),
            PlaybackSettings::DESPAWN_ON_END,
        ));
    }
}

fn manage_thruster_sound(
    mut commands: Commands,
    assets: Res<SoundAssets>,
    mut thruster_entity: ResMut<ThrusterSoundEntity>,
    thruster_query: Query<&Thruster, With<Player>>,
) {
    let is_active = thruster_query
        .get_single()
        .map(|t| t.active)
        .unwrap_or(false);

    if is_active && thruster_entity.0.is_none() {
        let entity = commands
            .spawn((
                AudioPlayer(assets.thruster.clone()),
                PlaybackSettings::LOOP,
            ))
            .id();
        thruster_entity.0 = Some(entity);
    } else if !is_active {
        if let Some(entity) = thruster_entity.0.take() {
            commands.entity(entity).despawn();
        }
    }
}

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletFiredEvent>()
            .add_event::<AsteroidDestroyedEvent>()
            .init_resource::<ThrusterSoundEntity>()
            .add_systems(Startup, load_sounds)
            .add_systems(
                Update,
                (
                    play_shoot_sound,
                    play_asteroid_explosion,
                    play_ship_explosion,
                    manage_thruster_sound,
                ),
            );
    }
}
