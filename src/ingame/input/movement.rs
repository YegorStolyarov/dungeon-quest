use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::config::*;
use crate::ingame::resources::dungeon::layer::Layer;
use crate::ingame::resources::fixed::animation_state::AnimationState;
use crate::ingame::resources::player::Player;

pub fn player_movement_handle_system(
    mut player_query: Query<(&mut Player, &mut Transform, &TextureAtlasSprite)>,
    mut layer_block_query: Query<(&Layer, &Transform), Without<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player_stats, mut transform, sprite) = player_query.single_mut();

    let mut is_move = false;

    let mut can_move_left: bool = true;
    let mut can_move_right: bool = true;
    let mut can_move_up: bool = true;
    let mut can_move_down: bool = true;

    player_stats.animation_state = AnimationState::Idle;

    for (layer_block_type, layer_block_transform) in layer_block_query.iter_mut() {
        let layer_block_size = match *layer_block_type {
            Layer::BorderTop => Vec2::new(64.0, -64.0),
            Layer::BorderBottom => Vec2::new(TILE_SIZE, 103.0),
            Layer::BorderRight => Vec2::new(TILE_SIZE, TILE_SIZE),
            Layer::BorderLeft => Vec2::new(TILE_SIZE, TILE_SIZE),
            Layer::None => Vec2::new(0.0, 0.0),
        };

        if *layer_block_type == Layer::None {
            continue;
        }

        if collide(
            layer_block_transform.translation,
            layer_block_size,
            transform.translation,
            sprite.custom_size.unwrap(),
        )
        .is_some()
        {
            match *layer_block_type {
                Layer::BorderTop => can_move_up = false,
                Layer::BorderBottom => can_move_down = false,
                Layer::BorderRight => can_move_right = false,
                Layer::BorderLeft => can_move_left = false,
                Layer::None => (),
            }
        }
    }

    if keyboard_input.pressed(KeyCode::W) && can_move_up {
        transform.translation.y += player_stats.speed * TILE_SIZE * time.delta_seconds();
        is_move = true;
    }

    if keyboard_input.pressed(KeyCode::S) && can_move_down {
        transform.translation.y -= player_stats.speed * TILE_SIZE * time.delta_seconds();
        is_move = true;
    }

    if keyboard_input.pressed(KeyCode::A) && can_move_left {
        transform.translation.x -= player_stats.speed * TILE_SIZE * time.delta_seconds();
        transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        is_move = true;
    }

    if keyboard_input.pressed(KeyCode::D) && can_move_right {
        transform.translation.x += player_stats.speed * TILE_SIZE * time.delta_seconds();
        transform.rotation = Quat::default();
        is_move = true;
    }

    if is_move {
        player_stats.animation_state = AnimationState::Moving;
    }
}
