use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::Rng;
use std::time::Duration;

use crate::components::invinsible_cooldown::InvisibleCooldownComponent;
use crate::components::monster::MonsterComponent;
use crate::components::player::PlayerComponent;
use crate::components::player_animation::PlayerAnimation;
use crate::components::player_list_effects::PlayerListEffectsComponent;
use crate::config::*;
use crate::plugins::player::{PLAYER_SIZE_HEIGHT, PLAYER_SIZE_WIDTH};
use crate::resources::animation_state::AnimationState;
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

pub fn monsters_collision_check(
    mut player_query: Query<(
        &mut PlayerComponent,
        &mut PlayerAnimation,
        &mut PlayerListEffectsComponent,
        &mut InvisibleCooldownComponent,
        &Transform,
    )>,
    monsters_query: Query<(&MonsterComponent, &Transform), Without<PlayerComponent>>,
) {
    let (
        mut player,
        mut player_animation,
        mut player_list_effects,
        mut invinsible_cooldown,
        player_transform,
    ) = player_query.single_mut();
    let player_size = Vec2::new(PLAYER_SIZE_WIDTH, PLAYER_SIZE_HEIGHT);
    let player_position = player_transform.translation;

    if !invinsible_cooldown.duration.finished() {
        return;
    }

    for (monster_component, transform) in monsters_query.iter() {
        let monster_size = Vec2::new(monster_component.width, monster_component.height);
        let monster_position = transform.translation;
        if collide(player_position, player_size, monster_position, monster_size).is_some() {
            let damage = monster_component.damage;

            player.current_health_points = if damage > player.current_health_points {
                0.0
            } else {
                player.current_health_points - damage
            };

            let debuff_effect = monster_component.trigger_effect;
            let trigger_chance = monster_component.trigger_chance;

            if debuff_effect != None && trigger_chance != 0.0 {
                let mut rng = rand::thread_rng();
                if rng.gen_range(0.0..1.0) < trigger_chance {
                    player_list_effects.activate(debuff_effect.unwrap());
                }
            }

            invinsible_cooldown.duration = Timer::new(Duration::from_secs_f32(2.0), false);
            invinsible_cooldown.hurt_duration = Timer::new(Duration::from_secs_f32(0.3), false);
            player_animation.animation_state = AnimationState::Hit;
            break;
        }
    }
}
