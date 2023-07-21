use bevy::prelude::*;

use crate::config::*;
use crate::materials::ingame::InGameMaterials;
use crate::plugins::survival_mode::dungeon::{TOTAL_TILE_HEIGHT, TOTAL_TILE_WIDTH};
use crate::plugins::survival_mode::SurvivalModeData;
use crate::resources::dungeon::block_type::BlockType;
use crate::resources::dungeon::rooms::Rooms;
use crate::resources::dungeon::wall::Wall;
use crate::resources::dungeon::wall_type::WallType;
use crate::resources::dungeon::walls::Walls;

pub fn walls(
    mut commands: Commands,
    rooms: Res<Rooms>,
    ingame_materials: Res<InGameMaterials>,
    mut data: ResMut<SurvivalModeData>,
) {
    let room = rooms.get_room(0.0);

    let walls = commands
        .spawn(SpriteBundle {
            ..Default::default()
        })
        .with_children(|parent| {
            for (row_index, row) in room.tilemap.iter().enumerate() {
                for (column_index, column) in row.iter().enumerate() {
                    if *column != 0 {
                        wall(parent, row_index, column_index, *column, &ingame_materials);
                    }
                }
            }
        })
        .insert(Walls)
        .insert(Name::new("Walls"))
        .id();

    data.walls = Some(walls);
}

fn wall(
    parent: &mut ChildBuilder,
    row_index: usize,
    column_index: usize,
    value: i32,
    ingame_materials: &InGameMaterials,
) {
    let start_x = 0.0 - TOTAL_TILE_WIDTH * TILE_SIZE / 2.0 - TILE_SIZE / 2.0;
    let start_y = 0.0 + (TOTAL_TILE_HEIGHT * TILE_SIZE / 2.0 + TILE_SIZE / 2.0);

    let block_type = match value.abs() {
        1 => {
            if row_index == 1 {
                BlockType::WallTop
            } else {
                BlockType::WallBottom
            }
        }
        7 => BlockType::WallLeft,
        8 => BlockType::WallRight,
        _ => BlockType::None,
    };

    let x = start_x + column_index as f32 * TILE_SIZE;
    let y = start_y - row_index as f32 * TILE_SIZE;

    let image = match value {
        -1 | 1 => ingame_materials.dungeon_materials.wall.clone(),
        -2 | 2 => ingame_materials.dungeon_materials.wall_border_mid.clone(),
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
        -7 | 7 => ingame_materials.dungeon_materials.wall_border_left.clone(),
        -8 | 8 => ingame_materials.dungeon_materials.wall_border_right.clone(),
        _ => panic!("Unknow room value: {}", value),
    };

    let component_name = if value < 0 {
        "TemporaryWall"
    } else {
        "PermanentWall"
    };

    let z = if block_type == BlockType::WallTop {
        0.1
    } else {
        0.2
    };

    parent
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(x, y, z),
                ..Default::default()
            },
            texture: image,
            ..Default::default()
        })
        .insert(Wall {
            wall_type: if value < 0 {
                WallType::Temporary
            } else {
                WallType::Permanent
            },
            row_index,
            column_index,
            value,
        })
        .insert(block_type)
        .insert(Name::new(component_name));
}
