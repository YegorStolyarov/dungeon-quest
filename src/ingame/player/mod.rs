use bevy::prelude::*;

use crate::scenes::SceneState;

pub mod animation;
pub mod door_interaction;
pub mod initiate;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::InGameScene).with_system(initiate::initiate_player),
        );
        app.add_system_set(
            SystemSet::on_update(SceneState::InGameScene)
                .with_system(door_interaction::door_interaction_handle_system),
        );
    }
}
