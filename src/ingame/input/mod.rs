use bevy::prelude::*;

use crate::scenes::SceneState;

pub mod movement;

pub struct InputHandlePlugin;

impl Plugin for InputHandlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(SceneState::InGameClassicModeScene)
                .with_system(movement::player_movement_handle_system),
        );
    }
}
