use bevy::prelude::*;

use crate::config::*;
use crate::ingame::classic_mode::dungeon::TOTAL_TILE_HEIGHT;
use crate::ingame::classic_mode::ClassicModeData;
use crate::ingame::materials::InGameMaterials;
use crate::ingame::resources::dungeon::rooms::Rooms;
use crate::ingame::resources::dungeon::wall::Wall;
use crate::ingame::resources::dungeon::wall_type::WallType;
use crate::ingame::resources::dungeon::walls::Walls;
use crate::ingame::resources::dungeon::Dungeon;
use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;

const START_Y: f32 = 0.0 + WINDOW_HEIGHT / 2.0 - TILE_SIZE / 2.0;
const START_X: f32 = 0.0 - WINDOW_HEIGHT * RESOLUTION / 2.0 + TILE_SIZE / 2.0;

pub fn walls(
    mut commands: Commands,
    dungeon: Res<Dungeon>,
    rooms: Res<Rooms>,
    ingame_materials: Res<InGameMaterials>,
    mut data: ResMut<ClassicModeData>,
) {
    let current_floor = dungeon.current_floor.clone();
    let current_position = current_floor.current_position;

    let room_id = current_floor.map[current_position.row_index][current_position.column_index];

    let room = rooms.get_room(if room_id == 1.0 { 1.0 } else { 1.0 });

    let walls = commands
        .spawn_bundle(SpriteBundle {
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
    let x = START_X + column_index as f32 * TILE_SIZE;
    let y = START_Y - row_index as f32 * TILE_SIZE;

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
        7 => ingame_materials.dungeon_materials.wall_border_left.clone(),
        8 => ingame_materials.dungeon_materials.wall_border_right.clone(),
        _ => panic!("Unknow room value: {}", value),
    };

    let component_name = if value < 0 { "Temporary Wall" } else { "_" };

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
        .insert(Wall {
            wall_type: if value < 0 {
                WallType::Temporary
            } else {
                WallType::Permanent
            },
            row_index,
            column_index,
        })
        .insert(Name::new(component_name));
}

pub fn temporary_walls_system(
    mut wall_query: Query<(&Wall, &mut Visibility)>,
    player_dungeon_stats: Res<PlayerDungeonStats>,
    dungeon: Res<Dungeon>,
) {
    if player_dungeon_stats.is_changed() {
        let current_floor = dungeon.current_floor.clone();
        let current_position = current_floor.current_position;
        let total_rows = current_floor.total_rows;
        let total_room_rows = TOTAL_TILE_HEIGHT;

        let has_above_room = if current_position.row_index > 0 {
            let above_room_row_index = current_position.row_index - 1;
            current_floor.map[above_room_row_index][current_position.column_index] != 0.0
        } else {
            false
        };

        let has_below_room = if current_position.row_index < total_rows - 1 {
            let below_room_row_index = current_position.row_index + 1;
            current_floor.map[below_room_row_index][current_position.column_index] != 0.0
        } else {
            false
        };

        for (wall, mut visibility) in wall_query.iter_mut() {
            if wall.wall_type == WallType::Temporary {
                if wall.row_index == 0 || wall.row_index == 1 {
                    visibility.is_visible = !has_above_room;
                }

                if wall.row_index == total_room_rows - 1 || wall.row_index == total_room_rows - 2 {
                    visibility.is_visible = !has_below_room;
                }
            }
        }
    }
}
