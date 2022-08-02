use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::components::player::PlayerComponent;
use crate::config::*;
use crate::plugins::player::{PLAYER_SIZE_HEIGHT, PLAYER_SIZE_WIDTH};
use crate::resources::dungeon::block_type::BlockType;
use crate::resources::player::player_available_movement::PlayerAvailableMovement;

pub fn wall_collision_check(
    player_position: Vec3,
    block_type_query: &Query<(&BlockType, &Transform), Without<PlayerComponent>>,
) -> PlayerAvailableMovement {
    let mut player_available_movement = PlayerAvailableMovement {
        can_move_up: true,
        can_move_down: true,
        can_move_left: true,
        can_move_right: true,
    };

    let player_size = Vec2::new(PLAYER_SIZE_WIDTH, PLAYER_SIZE_HEIGHT);

    for (block_type, block_transform) in block_type_query.iter() {
        let block_position = match *block_type {
            BlockType::WallTop => block_transform.translation + Vec3::new(0.0, 64.0, 0.0),
            _ => block_transform.translation,
        };

        let block_size = match *block_type {
            BlockType::WallBottom => Vec2::new(TILE_SIZE, TILE_SIZE),
            BlockType::WallTop => Vec2::new(TILE_SIZE, TILE_SIZE),
            BlockType::WallLeft => Vec2::new(TILE_SIZE, TILE_SIZE),
            BlockType::WallRight => Vec2::new(TILE_SIZE, TILE_SIZE),
            BlockType::None => Vec2::new(0.0, 0.0),
        };

        if *block_type == BlockType::None {
            continue;
        }

        if collide(player_position, player_size, block_position, block_size).is_some() {
            match *block_type {
                BlockType::WallTop => player_available_movement.can_move_up = false,
                BlockType::WallBottom => player_available_movement.can_move_down = false,
                BlockType::WallLeft => player_available_movement.can_move_left = false,
                BlockType::WallRight => player_available_movement.can_move_right = false,
                BlockType::None => {}
            }
        }
    }
    player_available_movement
}
