use bevy::prelude::*;

use crate::ingame::input::input_handle_system;
use crate::scenes::SceneState;

pub struct InGameScenePlugin;

impl Plugin for InGameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(SceneState::InGameScene).with_system(input_handle_system),
        );
    }
}
