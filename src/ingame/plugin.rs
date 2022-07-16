use bevy::prelude::*;

use crate::ingame::dungeon::initiate::initiate_dungeon;
use crate::ingame::dungeon::room::*;
use crate::ingame::player::animation::*;
use crate::ingame::player::initiate::initiate_player;
use crate::scenes::SceneState;

pub struct InGameScenePlugin;

impl Plugin for InGameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::InGameScene)
                .with_system(initiate_dungeon)
                .with_system(initiate_player),
        );
        app.add_system_set(
            SystemSet::on_update(SceneState::InGameScene)
                .with_system(draw_room)
                .with_system(player_animation_system),
        );
    }
}
