use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::ingame::resources::dungeon::door::{Door, HorizontalDoor, VerticaltDoor};
use crate::ingame::resources::dungeon::position::Position;
use crate::ingame::resources::dungeon::Dungeon;
use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::ingame::resources::player::Player;

pub fn horizontal_door_interaction_handle(
    mut player_query: Query<(&mut Transform, &TextureAtlasSprite), With<Player>>,
    mut door_query: Query<
        (&Door, &Transform, &Sprite, &Visibility),
        (With<HorizontalDoor>, Without<Player>),
    >,
    mut player_dungeon_stats: ResMut<PlayerDungeonStats>,
    mut dungeon: ResMut<Dungeon>,
) {
    let (mut player_transform, player_sprite) = player_query.single_mut();
    let player_translation = player_transform.translation;
    let player_size = player_sprite.custom_size.unwrap();

    if player_dungeon_stats.is_room_cleared {
        let current_position = player_dungeon_stats.current_room_position;
        let cleared_positions = dungeon.current_floor.cleared_positions.clone();

        for (door, door_transform, door_sprite, visibility) in door_query.iter_mut() {
            if visibility.is_visible {
                continue;
            }

            let door_size = door_sprite.custom_size.unwrap();

            let door_translation = door_transform.translation;

            if collide(player_translation, player_size, door_translation, door_size).is_some() {
                let new_position = Position {
                    row_index: current_position.row_index,
                    column_index: if *door == Door::Right {
                        current_position.column_index + 1
                    } else {
                        current_position.column_index - 1
                    },
                };

                player_dungeon_stats.current_room_position = new_position;
                dungeon.current_floor.current_position = new_position;

                if cleared_positions.contains_key(&new_position) {
                    player_dungeon_stats.is_room_cleared = true;
                    let total_entered_time = cleared_positions.get(&new_position).unwrap().clone();
                    dungeon
                        .current_floor
                        .cleared_positions
                        .insert(new_position, total_entered_time + 1);
                } else {
                    player_dungeon_stats.is_room_cleared = false;
                }

                if *door == Door::Left {
                    player_transform.translation.x = (player_transform.translation.x * -1.0) - 15.0;
                } else if *door == Door::Right {
                    player_transform.translation.x = (player_transform.translation.x * -1.0) + 15.0;
                }
            }
        }
    }
}

pub fn vertical_door_interaction_handle(
    mut player_query: Query<(&mut Transform, &TextureAtlasSprite), With<Player>>,
    mut vertical_door_query: Query<(&Visibility, &Children), With<VerticaltDoor>>,
    mut door_query: Query<(&Door, &Transform), Without<Player>>,
    mut player_dungeon_stats: ResMut<PlayerDungeonStats>,
    mut dungeon: ResMut<Dungeon>,
) {
    let (mut player_transform, player_spirte) = player_query.single_mut();
    let player_translation = player_transform.translation;
    let player_size = player_spirte.custom_size.unwrap().clone();

    if player_dungeon_stats.is_room_cleared {
        let current_position = player_dungeon_stats.current_room_position;
        let cleared_positions = dungeon.current_floor.cleared_positions.clone();

        for (visibility, children) in vertical_door_query.iter_mut() {
            if !visibility.is_visible {
                continue;
            }
            for child in children.iter() {
                let result = door_query.get_mut(*child);
                if result.is_ok() {
                    let (door, door_transform) = result.unwrap();
                    let translation = door_transform.translation;

                    let size = if *door == Door::Top {
                        Vec2::new(128.0, 10.0)
                    } else {
                        Vec2::new(128.0, 36.0)
                    };

                    if collide(player_translation, player_size, translation, size).is_some() {
                        let new_position = Position {
                            row_index: if *door == Door::Bottom {
                                current_position.row_index + 1
                            } else {
                                current_position.row_index - 1
                            },
                            column_index: current_position.column_index,
                        };

                        player_dungeon_stats.current_room_position = new_position;
                        dungeon.current_floor.current_position = new_position;

                        if cleared_positions.contains_key(&new_position) {
                            player_dungeon_stats.is_room_cleared = true;
                            let cleared_position = cleared_positions.get(&new_position);
                            let total_entered_time = cleared_position.unwrap();
                            dungeon
                                .current_floor
                                .cleared_positions
                                .insert(new_position, total_entered_time + 1);
                        } else {
                            player_dungeon_stats.is_room_cleared = false;
                        }

                        if *door == Door::Top {
                            player_transform.translation.y = -130.0;
                        } else {
                            player_transform.translation.y = 130.0;
                        }
                    }
                }
            }
        }
    }
}
