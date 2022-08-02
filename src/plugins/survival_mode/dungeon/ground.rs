use bevy::prelude::*;

use crate::config::*;
use crate::materials::ingame::InGameMaterials;
use crate::plugins::survival_mode::dungeon::{TOTAL_TILE_HEIGHT, TOTAL_TILE_WIDTH};
use crate::plugins::survival_mode::SurvivalModeData;
use crate::resources::dungeon::ground::Ground;
use crate::resources::dungeon::layer::Layer;

pub fn ground(
    mut commands: Commands,
    ingame_materials: Res<InGameMaterials>,
    mut data: ResMut<SurvivalModeData>,
) {
    let start_x = 0.0 - (TOTAL_TILE_WIDTH * TILE_SIZE / 2.0 - TILE_SIZE / 2.0);
    let start_y = 0.0 + (TOTAL_TILE_HEIGHT * TILE_SIZE / 2.0 - TILE_SIZE / 2.0);

    let ground = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(
                    TOTAL_TILE_WIDTH * TILE_SIZE,
                    TOTAL_TILE_HEIGHT * TILE_SIZE,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            let total_tile_width = TOTAL_TILE_WIDTH as usize;
            let total_tile_height = TOTAL_TILE_HEIGHT as usize;
            for row_index in 0..total_tile_height {
                for column_index in 0..total_tile_width {
                    let x = start_x + column_index as f32 * TILE_SIZE;
                    let y = start_y - row_index as f32 * TILE_SIZE;

                    let component_name = format!("Layer[{},{}]", row_index, column_index);

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
                            texture: ingame_materials.dungeon_materials.floor.clone(),
                            ..Default::default()
                        })
                        .insert(Layer)
                        .insert(Name::new(component_name));
                }
            }
        })
        .insert(Name::new("Ground"))
        .insert(Ground)
        .id();

    data.ground = Some(ground);
}
