use bevy::prelude::*;

use crate::config::*;
use crate::materials::ingame::InGameMaterials;
use crate::plugins::classic_mode::dungeon::{TOTAL_TILE_HEIGHT, TOTAL_TILE_WIDTH};
use crate::plugins::classic_mode::ClassicModeData;
use crate::resources::dungeon::end_point::EndPoint;
use crate::resources::dungeon::Dungeon;

pub fn end_point(
    mut commands: Commands,
    ingame_materials: Res<InGameMaterials>,
    mut data: ResMut<ClassicModeData>,
) {
    let start_x = 0.0 - WINDOW_HEIGHT * RESOLUTION / 2.0 + TILE_SIZE / 2.0;
    let start_y = 0.0 + WINDOW_HEIGHT / 2.0 - TILE_SIZE / 2.0;

    let center_row = TOTAL_TILE_HEIGHT / 2;
    let center_column = TOTAL_TILE_WIDTH / 2;

    let end_point = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(
                    start_x + center_column as f32 * TILE_SIZE,
                    start_y - center_row as f32 * TILE_SIZE,
                    0.1,
                ),
                ..Default::default()
            },
            texture: ingame_materials.dungeon_materials.ladder.clone(),
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(EndPoint)
        .insert(Name::new("EndPoint"))
        .id();

    data.end_point = Some(end_point);
}

pub fn end_point_handle_system(
    mut query: Query<(&mut Visibility, &mut Handle<Image>), With<EndPoint>>,
    ingame_materials: Res<InGameMaterials>,
    dungeon: Res<Dungeon>,
) {
    for (mut visibility, mut handle_image) in query.iter_mut() {
        let current_position = dungeon.current_floor.current_position;
        let end_room_position = dungeon.current_floor.end_room_position;

        if end_room_position == current_position {
            *visibility = Visibility::Visible;
            if dungeon.current_floor.is_last_floor {
                *handle_image = ingame_materials.dungeon_materials.treasure.clone();
            } else {
                *handle_image = ingame_materials.dungeon_materials.ladder.clone();
            }
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
