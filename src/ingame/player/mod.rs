use bevy::prelude::*;

use crate::scenes::SceneState;

pub mod animation;
pub mod door_interaction;
pub mod initiate;
pub mod ladder_interaction;
pub mod treasure_interaction;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::InGameScene).with_system(initiate::initiate_player),
        );
        app.add_system_set(
            SystemSet::on_update(SceneState::InGameScene)
                .with_system(door_interaction::door_interaction_handle_system)
                .with_system(ladder_interaction::ladder_interaction_handle_system)
                .with_system(animation::player_animation_system)
                .with_system(treasure_interaction::treasure_interaction_handle_system),
        );
    }
}
