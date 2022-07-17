use bevy::prelude::*;

use crate::ingame::resources::dungeon::Dungeon;
use crate::ingame::resources::fixed::animation_state::AnimationState;
use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::ingame::resources::player::Player;

const TILE_SIZE: f32 = 64.0;

pub fn input_handle_system(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    mut player_dungeon_stats: ResMut<PlayerDungeonStats>,
    keyboard_input: Res<Input<KeyCode>>,
    mut dungeon: ResMut<Dungeon>,
    time: Res<Time>,
) {
    let (mut player_stats, mut transform) = player_query.single_mut();

    let mut is_move = false;

    player_stats.animation_state = AnimationState::Idle;

    if keyboard_input.pressed(KeyCode::W) {
        transform.translation.y += player_stats.speed * TILE_SIZE * time.delta_seconds();
        is_move = true;
    }

    if keyboard_input.pressed(KeyCode::S) {
        transform.translation.y -= player_stats.speed * TILE_SIZE * time.delta_seconds();
        is_move = true;
    }

    if keyboard_input.pressed(KeyCode::A) {
        transform.translation.x -= player_stats.speed * TILE_SIZE * time.delta_seconds();
        transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        is_move = true;
    }

    if keyboard_input.pressed(KeyCode::D) {
        transform.translation.x += player_stats.speed * TILE_SIZE * time.delta_seconds();
        transform.rotation = Quat::default();
        is_move = true;
    }

    if keyboard_input.pressed(KeyCode::H) {
        dungeon.current_floor.current_position = dungeon.current_floor.end_room_position.clone();
        player_dungeon_stats.current_room_position =
            dungeon.current_floor.end_room_position.clone();
    }

    if is_move {
        player_stats.animation_state = AnimationState::Moving;
    }
}
