use bevy::prelude::*;

use crate::config::*;
use crate::ingame::materials::InGameMaterials;
use crate::ingame::resources::dungeon::door::Door;
use crate::ingame::resources::dungeon::doors::Doors;
use crate::ingame::resources::dungeon::position::Position;
use crate::ingame::resources::dungeon::Dungeon;
use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;

use crate::ingame::dungeon::TILE_SIZE;

pub fn draw_doors(
    mut commands: Commands,
    dungeon: Res<Dungeon>,
    player_dungeon_stats: Res<PlayerDungeonStats>,
    ingame_materials: Res<InGameMaterials>,
) {
    let start_y = 0.0 + WINDOW_HEIGHT / 2.0 - TILE_SIZE / 2.0;
    let start_x = 0.0 - WINDOW_HEIGHT * RESOLUTION / 2.0 + TILE_SIZE / 2.0;

    let current_floor = dungeon.current_floor.clone();
    let current_position = current_floor.current_position;

    if player_dungeon_stats.is_changed() {
        let is_room_cleared = player_dungeon_stats.is_room_cleared;

        let mut has_left_room: bool = false;
        let mut has_right_room: bool = false;

        if current_position.column_index < current_floor.total_columns - 1 {
            let right_position = Position {
                row_index: current_position.row_index,
                column_index: current_position.column_index + 1,
            };

            let right_position_value =
                current_floor.map[right_position.row_index][right_position.column_index];

            if right_position_value != 0.0 {
                has_right_room = true;
            }
        }

        if current_position.column_index > 0 {
            let left_position = Position {
                row_index: current_position.row_index,
                column_index: current_position.column_index - 1,
            };

            let left_position_value =
                current_floor.map[left_position.row_index][left_position.column_index];

            if left_position_value != 0.0 {
                has_left_room = true;
            }
        }

        commands
            .spawn_bundle(SpriteBundle {
                ..Default::default()
            })
            .with_children(|parent| {
                for (index, door) in Door::iterator().enumerate() {
                    match door {
                        Door::Left | Door::Right => {
                            let image = if *door == Door::Left {
                                ingame_materials.dungeon_materials.wall_border_left.clone()
                            } else {
                                ingame_materials.dungeon_materials.wall_border_right.clone()
                            };

                            let x = start_x + ((index * 15) as f32) * TILE_SIZE;
                            let y = start_y - 4.0 * TILE_SIZE;

                            // is blocked only when not having a next room and current room isn't cleared

                            let is_blocked = if *door == Door::Right {
                                if has_right_room {
                                    !is_room_cleared
                                } else {
                                    true
                                }
                            } else {
                                if has_left_room {
                                    !is_room_cleared
                                } else {
                                    true
                                }
                            };

                            parent
                                .spawn_bundle(SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                        ..Default::default()
                                    },
                                    transform: Transform {
                                        translation: Vec3::new(x, y, 0.2),
                                        ..Default::default()
                                    },
                                    texture: image,
                                    visibility: Visibility {
                                        is_visible: is_blocked,
                                    },
                                    ..Default::default()
                                })
                                .insert(door.clone());
                        }
                        Door::Bottom => {
                            if current_position.row_index + 1 < current_floor.total_rows {
                                let lower_position = Position {
                                    row_index: current_position.row_index + 1,
                                    column_index: current_position.column_index,
                                };

                                let lower_position_value = current_floor.map
                                    [lower_position.row_index][lower_position.column_index];

                                if lower_position_value != 0.0 {
                                    front_door(parent, door, is_room_cleared, &ingame_materials);
                                }
                            }
                        }
                        Door::Top => {
                            if current_position.row_index > 0 {
                                let upper_position = Position {
                                    row_index: current_position.row_index - 1,
                                    column_index: current_position.column_index,
                                };

                                let upper_position_value = current_floor.map
                                    [upper_position.row_index][upper_position.column_index];

                                if upper_position_value != 0.0 {
                                    front_door(parent, door, is_room_cleared, &ingame_materials);
                                }
                            }
                        }
                    }
                }
            })
            .insert(Doors)
            .insert(Name::new("Doors"));
    }
}

fn front_door(
    parent: &mut ChildBuilder,
    door: &Door,
    is_room_cleared: bool,
    ingame_materials: &InGameMaterials,
) {
    let left_part = ingame_materials.dungeon_materials.door_left_part.clone();
    let right_part = ingame_materials.dungeon_materials.door_right_part.clone();
    let door_closed = ingame_materials.dungeon_materials.door_closed.clone();
    let door_opened = ingame_materials.dungeon_materials.door_opened.clone();

    let left_door_part_x = -96.0;
    let right_door_part_x = 96.0;

    let y = if *door == Door::Bottom { -224.0 } else { 224.0 };
    let z = if *door == Door::Bottom { 0.2 } else { 0.1 };

    parent.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE * 2.0)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(left_door_part_x, y, z),
            ..Default::default()
        },
        texture: left_part,
        ..Default::default()
    });

    parent.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE * 2.0)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(right_door_part_x, y, z),
            ..Default::default()
        },
        texture: right_part,
        ..Default::default()
    });

    parent
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE * 2.0, TILE_SIZE * 2.0)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, y, z),
                ..Default::default()
            },
            texture: if is_room_cleared {
                door_opened
            } else {
                door_closed
            },
            ..Default::default()
        })
        .insert(door.clone());
}

pub fn clear_doors(
    mut query: Query<(Entity, &Doors)>,
    mut commands: Commands,
    player_dungeon_stats: Res<PlayerDungeonStats>,
) {
    for (entity, _doors) in query.iter_mut() {
        if player_dungeon_stats.is_changed() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
