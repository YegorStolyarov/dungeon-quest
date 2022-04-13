use bevy::prelude::*;

use crate::config::*;

pub struct PlayerData {
    player: Entity,
}

#[derive(Component)]
pub struct Player {
    movement_speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            movement_speed: 3.0,
        }
    }
}

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player = commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("images/uuuuu.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player::new())
        .id();

    commands.insert_resource(PlayerData { player });
}

pub fn player_movement_system(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();

    if keyboard_input.pressed(KeyCode::W) {
        transform.translation.y += player.movement_speed * TILE_SIZE * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::S) {
        transform.translation.y -= player.movement_speed * TILE_SIZE * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::A) {
        transform.translation.x -= player.movement_speed * TILE_SIZE * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::D) {
        transform.translation.x += player.movement_speed * TILE_SIZE * time.delta_seconds();
    }
}

pub fn cleanup_player(mut commands: Commands, player_data: Res<PlayerData>) {
    commands.entity(player_data.player).despawn_recursive();
}
