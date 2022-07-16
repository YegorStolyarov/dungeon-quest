use bevy::prelude::*;

use crate::config::*;
use crate::ingame::materials::{dungeon::DungeonMaterials, InGameMaterials};
use crate::ingame::resources::dungeon::rooms::Rooms;
use crate::ingame::resources::dungeon::Dungeon;

const TILE_SIZE: f32 = 64.0;

pub fn draw_room(
    mut commands: Commands,
    dungeon: Res<Dungeon>,
    rooms: Res<Rooms>,
    ingame_materials: Res<InGameMaterials>,
) {
    if dungeon.is_changed() {
        let current_floor = dungeon.current_floor;
        let floor = dungeon.floors[current_floor].clone();

        let current_position = floor.current_position;
        let room_id = floor.map[current_position.row_index][current_position.column_index];

        let room = rooms.get_room(1.0);

        // let dungeon_materials = ingame_materials.dungeon_materials;

        for (row_index, row) in room.tilemap.iter().enumerate() {
            let start_y = 0.0 + WINDOW_HEIGHT / 2.0 - TILE_SIZE / 2.0;
            let start_x = 0.0 - WINDOW_HEIGHT * RESOLUTION / 2.0 + TILE_SIZE / 2.0;

            for (column_index, column) in row.iter().enumerate() {
                let x = start_x + column_index as f32 * TILE_SIZE;
                let y = start_y - row_index as f32 * TILE_SIZE;

                let floor_image = ingame_materials.dungeon_materials.floor.clone();

                commands.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.0),
                        ..Default::default()
                    },
                    texture: floor_image,
                    ..Default::default()
                });

                if *column != 0 {
                    let image = match column {
                        0 => ingame_materials.dungeon_materials.floor.clone(),
                        1 => ingame_materials.dungeon_materials.wall.clone(),
                        2 => ingame_materials.dungeon_materials.wall_border_mid.clone(),
                        3 => ingame_materials
                            .dungeon_materials
                            .wall_border_corner_left
                            .clone(),
                        4 => ingame_materials
                            .dungeon_materials
                            .wall_border_corner_right
                            .clone(),
                        5 => ingame_materials
                            .dungeon_materials
                            .wall_border_corner_bottom_left
                            .clone(),
                        6 => ingame_materials
                            .dungeon_materials
                            .wall_border_corner_bottom_right
                            .clone(),
                        7 => ingame_materials.dungeon_materials.wall_border_right.clone(),
                        8 => ingame_materials.dungeon_materials.wall_border_left.clone(),
                        _ => panic!("Unknow room value: {}", column),
                    };

                    commands.spawn_bundle(SpriteBundle {
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
                    });
                }
            }
        }
    }
}
