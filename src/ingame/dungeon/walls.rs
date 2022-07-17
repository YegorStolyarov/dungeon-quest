use bevy::prelude::*;

use crate::config::*;
use crate::ingame::materials::InGameMaterials;
use crate::ingame::resources::dungeon::position::Position;
use crate::ingame::resources::dungeon::rooms::Rooms;
use crate::ingame::resources::dungeon::walls::Walls;
use crate::ingame::resources::dungeon::Dungeon;
use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;

use crate::ingame::dungeon::TILE_SIZE;

pub fn draw_walls(
    mut commands: Commands,
    dungeon: Res<Dungeon>,
    rooms: Res<Rooms>,
    player_dungeon_stats: Res<PlayerDungeonStats>,
    ingame_materials: Res<InGameMaterials>,
) {
    if player_dungeon_stats.is_changed() {
        let start_y = 0.0 + WINDOW_HEIGHT / 2.0 - TILE_SIZE / 2.0;
        let start_x = 0.0 - WINDOW_HEIGHT * RESOLUTION / 2.0 + TILE_SIZE / 2.0;

        let current_floor = dungeon.current_floor.clone();
        let current_position = current_floor.current_position;

        // let room_id = current_floor.map[current_position.row_index][current_position.column_index];

        let mut build_wall_top: bool = true;
        let mut build_wall_bottom: bool = true;

        if current_position.row_index > 0 {
            let upper_position = Position {
                row_index: current_position.row_index - 1,
                column_index: current_position.column_index,
            };

            if current_floor.map[upper_position.row_index][upper_position.column_index] != 0.0 {
                build_wall_top = false;
            }
        }

        if current_position.row_index < current_floor.total_columns - 1 {
            let lower_position = Position {
                row_index: current_position.row_index + 1,
                column_index: current_position.column_index,
            };
            if current_floor.map[lower_position.row_index][lower_position.column_index] != 0.0 {
                build_wall_bottom = false;
            }
        }

        let room = rooms.get_room(1.0);

        commands
            .spawn_bundle(SpriteBundle {
                ..Default::default()
            })
            .with_children(|parent| {
                for (row_index, row) in room.tilemap.iter().enumerate() {
                    for (column_index, column) in row.iter().enumerate() {
                        let custom_size = Vec2::new(TILE_SIZE, TILE_SIZE);

                        let x = start_x + column_index as f32 * custom_size.x;
                        let y = start_y - row_index as f32 * custom_size.y;

                        if *column != 0 {
                            let image = match column {
                                1 => ingame_materials.dungeon_materials.wall.clone(),
                                2 => ingame_materials.dungeon_materials.wall_border_mid.clone(),
                                3 => ingame_materials
                                    .dungeon_materials
                                    .wall_border_corner_top_left
                                    .clone(),
                                4 => ingame_materials
                                    .dungeon_materials
                                    .wall_border_corner_top_right
                                    .clone(),
                                5 => ingame_materials.dungeon_materials.wall_left.clone(),
                                6 => ingame_materials.dungeon_materials.wall_right.clone(),
                                7 => ingame_materials.dungeon_materials.wall_border_left.clone(),
                                8 => ingame_materials.dungeon_materials.wall_border_right.clone(),
                                _ => panic!("Unknow room value: {}", column),
                            };

                            parent
                                .spawn_bundle(SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: Some(custom_size),
                                        ..Default::default()
                                    },
                                    transform: Transform {
                                        translation: Vec3::new(x, y, 0.1),
                                        ..Default::default()
                                    },
                                    texture: image,
                                    ..Default::default()
                                })
                                .insert(Name::new("Wall"));
                        }

                        let image = if row_index == 0 || row_index == 7 {
                            ingame_materials.dungeon_materials.wall_border_mid.clone()
                        } else {
                            ingame_materials.dungeon_materials.wall.clone()
                        };

                        if build_wall_top && row_index <= 1 {
                            if *column == 0 {
                                parent
                                    .spawn_bundle(SpriteBundle {
                                        sprite: Sprite {
                                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                            ..Default::default()
                                        },
                                        transform: Transform {
                                            translation: Vec3::new(x, y, 0.1),
                                            ..Default::default()
                                        },
                                        texture: image.clone(),
                                        ..Default::default()
                                    })
                                    .insert(Name::new("Wall"));
                            }
                        }
                        if build_wall_bottom && row_index >= 7 {
                            if *column == 0 {
                                parent
                                    .spawn_bundle(SpriteBundle {
                                        sprite: Sprite {
                                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                            ..Default::default()
                                        },
                                        transform: Transform {
                                            translation: Vec3::new(x, y, 0.1),
                                            ..Default::default()
                                        },
                                        texture: image,
                                        ..Default::default()
                                    })
                                    .insert(Name::new("Wall"));
                            }
                        }
                    }
                }
            })
            .insert(Walls)
            .insert(Name::new("Walls"));
    }
}
pub fn clear_walls(
    mut query: Query<(Entity, &Walls)>,
    mut commands: Commands,
    player_dungeon_stats: Res<PlayerDungeonStats>,
) {
    for (entity, _walls) in query.iter_mut() {
        if player_dungeon_stats.is_changed() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
