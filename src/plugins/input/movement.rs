use bevy::prelude::*;

use crate::components::player::PlayerComponent;
use crate::config::*;
use crate::plugins::player::collisions::wall_collision_check;
use crate::resources::animation_state::AnimationState;
use crate::resources::dungeon::block_type::BlockType;
use crate::resources::player::player_animation::PlayerAnimation;

pub fn player_movement_handle_system(
    mut player_query: Query<(&PlayerComponent, &mut PlayerAnimation, &mut Transform)>,
    block_type_query: Query<(&BlockType, &Transform), Without<PlayerComponent>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player_stats, mut player_animation, mut transform) = player_query.single_mut();

    let mut delta = Vec3::new(0.0, 0.0, 0.0);

    let player_position = transform.translation;
    player_animation.animation_state = AnimationState::Idle;

    let player_availalbe_movement = wall_collision_check(player_position, &block_type_query);

    if keyboard_input.pressed(KeyCode::W) {
        if player_availalbe_movement.can_move_up {
            delta.y += player_stats.speed * TILE_SIZE * time.delta_seconds();
        }
    }

    if keyboard_input.pressed(KeyCode::S) {
        if player_availalbe_movement.can_move_down {
            delta.y -= player_stats.speed * TILE_SIZE * time.delta_seconds();
        }
    }

    if keyboard_input.pressed(KeyCode::A) {
        if player_availalbe_movement.can_move_left {
            delta.x -= player_stats.speed * TILE_SIZE * time.delta_seconds();
        }
    }

    if keyboard_input.pressed(KeyCode::D) {
        if player_availalbe_movement.can_move_right {
            delta.x += player_stats.speed * TILE_SIZE * time.delta_seconds();
        }
    }

    transform.translation += delta;

    if delta.x < 0.0 {
        transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
    } else if delta.x > 0.0 {
        transform.rotation = Quat::default();
    }

    if delta != Vec3::ZERO {
        player_animation.animation_state = AnimationState::Moving;
    }
}
