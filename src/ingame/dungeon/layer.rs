use bevy::prelude::*;

use crate::config::*;
use crate::ingame::materials::InGameMaterials;
use crate::ingame::resources::dungeon::ground::Ground;
use crate::ingame::resources::dungeon::ladder::Ladder;
use crate::ingame::resources::dungeon::layer::Layer;
use crate::ingame::resources::dungeon::Dungeon;

use crate::ingame::dungeon::{TILE_SIZE, TOTAL_TILE_HEIGHT, TOTAL_TILE_WIDTH};

const START_Y: f32 = 0.0 + WINDOW_HEIGHT / 2.0 - TILE_SIZE / 2.0;
const START_X: f32 = 0.0 - WINDOW_HEIGHT * RESOLUTION / 2.0 + TILE_SIZE / 2.0;

pub fn layer(mut commands: Commands, ingame_materials: Res<InGameMaterials>) {
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
            let center_row = TOTAL_TILE_HEIGHT / 2;
            let center_column = TOTAL_TILE_WIDTH / 2;

            for row_index in 0..TOTAL_TILE_HEIGHT {
                for column_index in 0..TOTAL_TILE_WIDTH {
                    let floor_image = ingame_materials.dungeon_materials.floor.clone();

                    if row_index >= 1 {
                        if column_index > 0 && column_index < 15 {
                            if row_index == center_row && column_index == center_column {
                                ladder(parent, row_index, column_index, floor_image.clone());
                            } else {
                                block_layer(
                                    parent,
                                    row_index,
                                    column_index,
                                    Some(floor_image.clone()),
                                );
                            }
                        } else {
                            block_layer(parent, row_index, column_index, None);
                        }
                    }
                }
            }
        })
        .insert(Name::new("Ground"))
        .insert(Ground);
}

fn block_layer(
    parent: &mut ChildBuilder,
    row_index: usize,
    column_index: usize,
    floor_image: Option<Handle<Image>>,
) {
    let layer = if row_index == 1 {
        Layer::BorderTop
    } else if row_index == 8 {
        Layer::BorderBottom
    } else if column_index == 0 {
        Layer::BorderLeft
    } else if column_index == 15 {
        Layer::BorderRight
    } else {
        Layer::None
    };

    let x = START_X + column_index as f32 * TILE_SIZE;
    let y = START_Y - row_index as f32 * TILE_SIZE;

    let component_name = if layer == Layer::None {
        "Layer"
    } else {
        "BorderLayer"
    };

    match floor_image {
        None => {
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        color: Color::NONE,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(layer)
                .insert(Name::new(component_name));
        }
        _ => {
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
                    texture: floor_image.unwrap(),
                    ..Default::default()
                })
                .insert(layer)
                .insert(Name::new(component_name));
        }
    }
}

fn ladder(
    parent: &mut ChildBuilder,
    row_index: usize,
    column_index: usize,
    floor_image: Handle<Image>,
) {
    parent
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(
                    START_X + column_index as f32 * TILE_SIZE,
                    START_Y - row_index as f32 * TILE_SIZE,
                    0.0,
                ),
                ..Default::default()
            },
            texture: floor_image,
            ..Default::default()
        })
        .insert(Layer::None)
        .insert(Name::new("Ladder"))
        .insert(Ladder);
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
            if dungeon.current_floor.is_last_floor {
                *handle_image = ingame_materials.dungeon_materials.floor.clone();
            } else {
                *handle_image = ingame_materials.dungeon_materials.ladder.clone();
            }
        } else {
            *handle_image = ingame_materials.dungeon_materials.floor.clone();
        }
    }
}
