use bevy::prelude::*;

use crate::config::*;
use crate::materials::ingame::InGameMaterials;
use crate::plugins::classic_mode::dungeon::{TOTAL_TILE_HEIGHT, TOTAL_TILE_WIDTH};
use crate::plugins::classic_mode::ClassicModeData;
use crate::resources::dungeon::ground::Ground;
use crate::resources::dungeon::layer::Layer;

pub fn ground(
    mut commands: Commands,
    ingame_materials: Res<InGameMaterials>,
    mut data: ResMut<ClassicModeData>,
) {
    let start_y: f32 = 0.0 + WINDOW_HEIGHT / 2.0 - TILE_SIZE / 2.0;
    let start_x: f32 = 0.0 - WINDOW_HEIGHT * RESOLUTION / 2.0 + TILE_SIZE / 2.0;

    let ground = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(WINDOW_HEIGHT * RESOLUTION, WINDOW_HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            for row_index in 0..TOTAL_TILE_HEIGHT {
                for column_index in 0..TOTAL_TILE_WIDTH {
                    if row_index >= 1 && column_index > 0 && column_index < 15 {
                        let x = start_x + column_index as f32 * TILE_SIZE;
                        let y = start_y - row_index as f32 * TILE_SIZE;

                        parent
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                    ..Default::default()
                                },
                                transform: Transform {
                                    translation: Vec3::new(x, y, 0.0),
                                    ..Default::default()
                                },
                                texture: ingame_materials.dungeon_materials.floor.clone(),
                                ..Default::default()
                            })
                            .insert(Layer)
                            .insert(Name::new("Layer"));
                    }
                }
            }
        })
        .insert(Name::new("Ground"))
        .insert(Ground)
        .id();

    data.ground = Some(ground);
}
