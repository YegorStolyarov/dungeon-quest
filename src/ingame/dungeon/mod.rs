use bevy::prelude::*;

use crate::ingame::dungeon::doors::{clear_doors, draw_doors};
use crate::ingame::dungeon::initiate::initiate_dungeon;
use crate::ingame::dungeon::layer::{change_floor_to_ladder, layer};
use crate::ingame::dungeon::treasure::{enable_treasure, treasure};
use crate::ingame::dungeon::walls::{clear_walls, draw_walls};
use crate::scenes::SceneState;

pub mod doors;
pub mod initiate;
pub mod layer;
pub mod treasure;
pub mod walls;

pub const TILE_SIZE: f32 = 64.0;
pub const TOTAL_TILE_WIDTH: usize = 16;
pub const TOTAL_TILE_HEIGHT: usize = 9;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::InGameScene)
                .with_system(initiate_dungeon.label("Initiate"))
                .with_system(layer.after("Initiate"))
                .with_system(treasure),
        );
        app.add_system_set(
            SystemSet::on_update(SceneState::InGameScene)
                .with_system(clear_walls)
                .with_system(draw_walls)
                .with_system(clear_doors)
                .with_system(draw_doors)
                .with_system(change_floor_to_ladder)
                .with_system(enable_treasure),
        );
    }
}
