use bevy::prelude::*;

use crate::config::*;
use crate::ingame::materials::InGameMaterials;
use crate::ingame::resources::dungeon::ground::Ground;
use crate::ingame::resources::dungeon::layer::Layer;
use crate::ingame::survival_mode::dungeon::{TOTAL_TILE_HEIGHT, TOTAL_TILE_WIDTH};
use crate::ingame::survival_mode::SurvivalModeData;

const START_Y: f32 = 0.0 + TOTAL_TILE_HEIGHT * TILE_SIZE / 2.0 - TILE_SIZE / 2.0;
const START_X: f32 = 0.0 - TOTAL_TILE_WIDTH * TILE_SIZE / 2.0 + TILE_SIZE / 2.0;

pub fn ground(
    mut commands: Commands,
    ingame_materials: Res<InGameMaterials>,
    mut data: ResMut<SurvivalModeData>,
) {
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
            for row_index in 0..TOTAL_TILE_HEIGHT as usize {
                for column_index in 0..TOTAL_TILE_WIDTH as usize {
                    // let floor_image = ingame_materials.dungeon_materials.floor.clone();
                    // layer(parent, row_index, column_index, Some(floor_image.clone()));
                }
            }
        })
        .insert(Name::new("Ground"))
        .insert(Ground)
        .id();

    data.ground = Some(ground);
}
