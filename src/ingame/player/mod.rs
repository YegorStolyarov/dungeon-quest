use bevy::prelude::*;

use crate::scenes::SceneState;

mod animation;
mod initiate;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::InGameClassicModeScene)
                .with_system(initiate::initiate_player),
        );
        app.add_system_set(
            SystemSet::on_update(SceneState::InGameClassicModeScene)
                .with_system(animation::player_animation_system),
        );

        app.add_system_set(
            SystemSet::on_exit(SceneState::InGameClassicModeScene)
                .with_system(initiate::clean_up_player),
        );
    }
}
