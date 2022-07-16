use bevy::prelude::*;

use crate::ingame::set_up_systems::*;
use crate::scenes::SceneState;

pub struct InGameScenePlugin;

impl Plugin for InGameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::InGameScene)
                .with_system(initiate_dungeon)
                .with_system(initiate_player),
        );
    }
}
