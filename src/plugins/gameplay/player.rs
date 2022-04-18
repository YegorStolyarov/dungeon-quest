use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::config::*;

pub struct PlayerEntity {
    _p: Entity,
}

#[derive(Component)]
pub struct AnimationTimer(Timer);

#[derive(Component, Inspectable)]
pub struct Player;

#[derive(Component, Inspectable)]
pub struct PlayerStats {
    movement_speed: f32,
    movement_state: PlayerMovementState,
}

#[derive(Inspectable)]
enum PlayerMovementState {
    Idle,    // 0 - 3
    Running, // 4 - 7
    Hit,     // 8
}

impl PlayerStats {
    pub fn new() -> Self {
        PlayerStats {
            movement_speed: 3.0,
            movement_state: PlayerMovementState::Idle,
        }
    }
}

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/heros/lizard.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 28.0), 9, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .insert(Player)
        .insert(PlayerStats::new())
        .insert(Timer::from_seconds(0.1, true))
        .id();

    commands.insert_resource(PlayerEntity { _p: player });
}

pub fn player_animation_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &PlayerStats,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (player_stats, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            match player_stats.movement_state {
                PlayerMovementState::Idle => {
                    let min_index = 0;
                    let max_index = 3;
                    if sprite.index > max_index || sprite.index < min_index {
                        sprite.index = min_index;
                    } else {
                        sprite.index += 1;
                    }
                }
                PlayerMovementState::Running => {
                    let min_index = 4;
                    let max_index = 7;
                    if sprite.index > max_index || sprite.index < min_index {
                        sprite.index = min_index;
                    } else {
                        sprite.index += 1;
                    }
                }
                PlayerMovementState::Hit => {
                    sprite.index = texture_atlas.textures.len() - 1;
                }
            }
        }
    }
}

pub fn player_movement_system(
    mut player_query: Query<(&mut PlayerStats, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player_stats, mut transform) = player_query.single_mut();

    let mut is_move = false;

    player_stats.movement_state = PlayerMovementState::Idle;

    if keyboard_input.pressed(KeyCode::W) {
        transform.translation.y += player_stats.movement_speed * TILE_SIZE * time.delta_seconds();
        is_move = true;
    }

    if keyboard_input.pressed(KeyCode::S) {
        transform.translation.y -= player_stats.movement_speed * TILE_SIZE * time.delta_seconds();
        is_move = true;
    }

    if keyboard_input.pressed(KeyCode::A) {
        transform.translation.x -= player_stats.movement_speed * TILE_SIZE * time.delta_seconds();
        transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        is_move = true;
    }

    if keyboard_input.pressed(KeyCode::D) {
        transform.translation.x += player_stats.movement_speed * TILE_SIZE * time.delta_seconds();
        transform.rotation = Quat::default();
        is_move = true;
    }

    if is_move {
        player_stats.movement_state = PlayerMovementState::Running;
    }
}

pub fn cleanup_player(mut commands: Commands, player_entity: Res<PlayerEntity>) {
    commands.entity(player_entity._p).despawn_recursive();
}
