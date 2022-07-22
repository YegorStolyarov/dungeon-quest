use bevy::prelude::*;

use crate::scenes::SceneState;

pub mod keyboard;
pub mod movement;

pub struct InputHandlePlugin;

impl Plugin for InputHandlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(SceneState::InGameClassicMode)
                .with_system(movement::player_movement_handle_system)
                .with_system(keyboard::escape),
        );

        app.add_system_set(
            SystemSet::on_update(SceneState::InGameSurvivalMode)
                .with_system(movement::player_movement_handle_system)
                .with_system(keyboard::escape),
        );
    }
}
