use bevy::prelude::*;
use chrono::prelude::*;

use crate::ingame::resources::dungeon::border_layer::BorderLayer;
use crate::ingame::resources::dungeon::Dungeon;
use crate::ingame::resources::fixed::animation_state::AnimationState;
use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::ingame::resources::player::Player;

use bevy::sprite::collide_aabb::collide;

const TILE_SIZE: f32 = 64.0;

pub fn input_handle_system(
    mut player_query: Query<(&mut Player, &mut Transform, &TextureAtlasSprite)>,
    mut wall_query: Query<(&BorderLayer, &Transform), Without<Player>>,
    mut player_dungeon_stats: ResMut<PlayerDungeonStats>,
    keyboard_input: Res<Input<KeyCode>>,
    mut dungeon: ResMut<Dungeon>,
    time: Res<Time>,
) {
    let (mut player_stats, mut transform, sprite) = player_query.single_mut();

    let mut is_move = false;

    let mut can_move_left: bool = true;
    let mut can_move_right: bool = true;
    let mut can_move_up: bool = true;
    let mut can_move_down: bool = true;

    player_stats.animation_state = AnimationState::Idle;

    for (border_layer, border_layer_transform) in wall_query.iter_mut() {
        let border_size = match *border_layer {
            BorderLayer::Top => Vec2::new(64.0, -64.0),
            BorderLayer::Bottom => Vec2::new(TILE_SIZE, 100.0),
            BorderLayer::Right => Vec2::new(TILE_SIZE, TILE_SIZE),
            BorderLayer::Left => Vec2::new(TILE_SIZE, TILE_SIZE),
        };

        if collide(
            border_layer_transform.translation,
            border_size,
            transform.translation,
            sprite.custom_size.unwrap(),
        )
        .is_some()
        {
            match *border_layer {
                BorderLayer::Top => can_move_up = false,
                BorderLayer::Bottom => can_move_down = false,
                BorderLayer::Right => can_move_right = false,
                BorderLayer::Left => can_move_left = false,
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

    if keyboard_input.pressed(KeyCode::H) {
        dungeon.current_floor.current_position = dungeon.current_floor.end_room_position.clone();
        player_dungeon_stats.current_room_position =
            dungeon.current_floor.end_room_position.clone();
    }

    if is_move {
        player_stats.animation_state = AnimationState::Moving;
    }
}
