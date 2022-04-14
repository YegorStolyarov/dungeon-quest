use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::config::*;

pub struct Player {
    entity: Entity,
}

#[derive(Component, Inspectable)]
pub struct PlayerStats {
    movement_speed: f32,
}

impl PlayerStats {
    pub fn new() -> Self {
        PlayerStats {
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
        .insert(PlayerStats::new())
        .id();

    commands.insert_resource(Player { entity: player });
}

pub fn player_movement_system(
    mut player_query: Query<(&PlayerStats, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player_stats, mut transform) = player_query.single_mut();

    if keyboard_input.pressed(KeyCode::W) {
        transform.translation.y += player_stats.movement_speed * TILE_SIZE * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::S) {
        transform.translation.y -= player_stats.movement_speed * TILE_SIZE * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::A) {
        transform.translation.x -= player_stats.movement_speed * TILE_SIZE * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::D) {
        transform.translation.x += player_stats.movement_speed * TILE_SIZE * time.delta_seconds();
    }
}

pub fn cleanup_player(mut commands: Commands, player: Res<Player>) {
    commands.entity(player.entity).despawn_recursive();
}
