use bevy::prelude::*;

use crate::config::*;
use crate::ingame::materials::InGameMaterials;
use crate::ingame::resources::dungeon::ladder::Ladder;
use crate::ingame::resources::dungeon::Dungeon;

use crate::ingame::dungeon::{TILE_SIZE, TOTAL_TILE_HEIGHT, TOTAL_TILE_WIDTH};

pub fn draw_layer(mut commands: Commands, ingame_materials: Res<InGameMaterials>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(WINDOW_HEIGHT * RESOLUTION, WINDOW_HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            let start_y = 0.0 + WINDOW_HEIGHT / 2.0 - TILE_SIZE / 2.0;
            let start_x = 0.0 - WINDOW_HEIGHT * RESOLUTION / 2.0 + TILE_SIZE / 2.0;

            let center_row = TOTAL_TILE_HEIGHT / 2;
            let center_column = TOTAL_TILE_WIDTH / 2;

            for row_index in 0..TOTAL_TILE_HEIGHT {
                for column_index in 0..TOTAL_TILE_WIDTH {
                    let x = start_x + column_index as f32 * TILE_SIZE;
                    let y = start_y - row_index as f32 * TILE_SIZE;

                    let floor_image = ingame_materials.dungeon_materials.floor.clone();

                    if row_index >= 1 {
                        if column_index > 0 && column_index < 15 {
                            if row_index == center_row && column_index == center_column {
                                parent
                                    .spawn_bundle(SpriteBundle {
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
                                    })
                                    .insert(Ladder);
                            } else {
                                parent.spawn_bundle(SpriteBundle {
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
                            }
                        }
                    }
                }
            }
        });
}

pub fn change_floor_to_ladder(
    mut query: Query<(&Ladder, &mut Handle<Image>)>,
    ingame_materials: Res<InGameMaterials>,
    dungeon: Res<Dungeon>,
) {
    for (_ladder, mut handle_image) in query.iter_mut() {
        let current_position = dungeon.current_floor.current_position;
        let end_room_position = dungeon.current_floor.end_room_position;

        if end_room_position == current_position {
            *handle_image = ingame_materials.dungeon_materials.ladder.clone();
        }
    }
}
