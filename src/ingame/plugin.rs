use bevy::prelude::*;

use crate::ingame::dungeon::doors::{clear_doors, draw_doors};
use crate::ingame::dungeon::initiate::initiate_dungeon;
use crate::ingame::dungeon::layer::{change_floor_to_ladder, draw_layer};
use crate::ingame::dungeon::walls::{clear_walls, draw_walls};
use crate::ingame::input::input_handle_system;
use crate::ingame::player::animation::*;
use crate::ingame::player::initiate::initiate_player;
use crate::scenes::SceneState;

pub struct InGameScenePlugin;

impl Plugin for InGameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::InGameScene)
                .with_system(initiate_dungeon)
                .with_system(draw_layer),
        );
        app.add_system_set(
            SystemSet::on_update(SceneState::InGameScene)
                .with_system(clear_walls)
                .with_system(draw_walls)
                .with_system(clear_doors)
                .with_system(draw_doors)
                .with_system(change_floor_to_ladder)
                .with_system(player_animation_system)
                .with_system(input_handle_system),
        );
    }
}
