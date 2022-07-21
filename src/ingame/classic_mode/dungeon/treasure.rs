use bevy::prelude::*;

use crate::config::*;
use crate::ingame::classic_mode::ClassicModeData;
use crate::ingame::materials::InGameMaterials;
use crate::ingame::resources::dungeon::treasure::Treasure;
use crate::ingame::resources::dungeon::Dungeon;

const START_Y: f32 = 0.0 + WINDOW_HEIGHT / 2.0 - TILE_SIZE / 2.0;
const START_X: f32 = 0.0 - WINDOW_HEIGHT * RESOLUTION / 2.0 + TILE_SIZE / 2.0;

pub fn treasure(
    mut commands: Commands,
    ingame_materials: Res<InGameMaterials>,
    mut data: ResMut<ClassicModeData>,
) {
    let center_row = TOTAL_TILE_HEIGHT / 2;
    let center_column = TOTAL_TILE_WIDTH / 2;

    let treasure = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(
                    START_X + center_column as f32 * TILE_SIZE,
                    START_Y - center_row as f32 * TILE_SIZE,
                    0.15,
                ),
                ..Default::default()
            },
            texture: ingame_materials.dungeon_materials.treasure.clone(),
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(Treasure)
        .insert(Name::new("Treasure"))
        .id();

    data.treasure = Some(treasure);
}

pub fn treasure_system(mut query: Query<(&Treasure, &mut Visibility)>, dungeon: Res<Dungeon>) {
    for (_treasure, mut visibility) in query.iter_mut() {
        let current_position = dungeon.current_floor.current_position;
        let end_room_position = dungeon.current_floor.end_room_position;

        if end_room_position == current_position && dungeon.current_floor.is_last_floor {
            visibility.is_visible = true;
        } else {
            visibility.is_visible = false;
        }
    }
}
