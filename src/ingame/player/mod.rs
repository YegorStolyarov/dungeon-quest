use bevy::prelude::*;

use crate::scenes::SceneState;

mod animation;
mod cleanup;
pub mod collisions;
mod initiate;

pub struct PlayerPlugin;

pub struct PlayerEntity {
    pub entity: Entity,
}

pub const PLAYER_SIZE_WIDTH: f32 = 16.0 * 3.5;
pub const PLAYER_SIZE_HEIGHT: f32 = 28.0 * 3.5;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::InGameClassicMode)
                .with_system(initiate::initiate_player),
        );
        app.add_system_set(
            SystemSet::on_update(SceneState::InGameClassicMode)
                .with_system(animation::player_animation_system),
        );

        app.add_system_set(
            SystemSet::on_exit(SceneState::InGameClassicMode).with_system(cleanup::clean_up_player),
        );

        app.add_system_set(
            SystemSet::on_enter(SceneState::InGameSurvivalMode)
                .with_system(initiate::initiate_player),
        );
        app.add_system_set(
            SystemSet::on_update(SceneState::InGameSurvivalMode)
                .with_system(animation::player_animation_system),
        );

        app.add_system_set(
            SystemSet::on_exit(SceneState::InGameSurvivalMode)
                .with_system(cleanup::clean_up_player),
        );
    }
}
